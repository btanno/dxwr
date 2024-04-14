use super::*;
use std::ops::Bound;
use std::sync::Arc;
use windows::Win32::Graphics::Direct3D12::*;

pub trait Type {
    const VALUE: D3D12_DESCRIPTOR_HEAP_TYPE;
}

pub mod descriptor_heap_type {
    use super::*;

    pub struct CbvSrvUav;

    impl Type for CbvSrvUav {
        const VALUE: D3D12_DESCRIPTOR_HEAP_TYPE = D3D12_DESCRIPTOR_HEAP_TYPE_CBV_SRV_UAV;
    }

    pub struct Rtv;

    impl Type for Rtv {
        const VALUE: D3D12_DESCRIPTOR_HEAP_TYPE = D3D12_DESCRIPTOR_HEAP_TYPE_RTV;
    }

    pub struct Dsv;

    impl Type for Dsv {
        const VALUE: D3D12_DESCRIPTOR_HEAP_TYPE = D3D12_DESCRIPTOR_HEAP_TYPE_DSV;
    }

    pub struct Sampler;

    impl Type for Sampler {
        const VALUE: D3D12_DESCRIPTOR_HEAP_TYPE = D3D12_DESCRIPTOR_HEAP_TYPE_SAMPLER;
    }
}
use descriptor_heap_type::*;

#[derive(PartialEq, Eq, Debug)]
pub struct CpuDescriptorHandle<T> {
    heap: ID3D12DescriptorHeap,
    handle: D3D12_CPU_DESCRIPTOR_HANDLE,
    _t: std::marker::PhantomData<T>,
}

impl<T> CpuDescriptorHandle<T> {
    #[inline]
    pub fn handle(&self) -> D3D12_CPU_DESCRIPTOR_HANDLE {
        self.handle.clone()
    }
}

impl<T> Clone for CpuDescriptorHandle<T> {
    fn clone(&self) -> Self {
        Self {
            heap: self.heap.clone(),
            handle: self.handle.clone(),
            _t: std::marker::PhantomData,
        }
    }
}

#[derive(PartialEq, Eq, Debug)]
pub struct GpuDescriptorHandle<T> {
    heap: ID3D12DescriptorHeap,
    handle: D3D12_GPU_DESCRIPTOR_HANDLE,
    _t: std::marker::PhantomData<T>,
}

impl<T> GpuDescriptorHandle<T> {
    #[inline]
    pub fn handle(&self) -> D3D12_GPU_DESCRIPTOR_HANDLE {
        self.handle.clone()
    }
}

impl<T> Clone for GpuDescriptorHandle<T> {
    fn clone(&self) -> Self {
        Self {
            heap: self.heap.clone(),
            handle: self.handle.clone(),
            _t: std::marker::PhantomData,
        }
    }
}

pub struct Builder<T, Len = ()> {
    device: ID3D12Device,
    desc: D3D12_DESCRIPTOR_HEAP_DESC,
    name: Option<String>,
    _t: std::marker::PhantomData<T>,
    _len: std::marker::PhantomData<Len>,
}

impl<T> Builder<T, ()>
where
    T: Type,
{
    fn new<U>(device: &U) -> Self
    where
        U: Into<ID3D12Device> + Clone,
    {
        let device: ID3D12Device = device.clone().into();
        let desc = D3D12_DESCRIPTOR_HEAP_DESC {
            Type: T::VALUE,
            ..Default::default()
        };
        Self {
            device,
            desc,
            name: None,
            _t: std::marker::PhantomData,
            _len: std::marker::PhantomData,
        }
    }
}

impl<T, Len> Builder<T, Len> {
    #[inline]
    pub fn len(mut self, len: usize) -> Builder<T, u32> {
        assert!(len <= u32::MAX as usize);
        self.desc.NumDescriptors = len as u32;
        Builder {
            device: self.device,
            desc: self.desc,
            name: self.name,
            _t: self._t,
            _len: std::marker::PhantomData,
        }
    }

    #[inline]
    pub fn flags(mut self, flags: D3D12_DESCRIPTOR_HEAP_FLAGS) -> Self {
        self.desc.Flags = flags;
        self
    }

    #[inline]
    pub fn node_mask(mut self, mask: u32) -> Self {
        self.desc.NodeMask = mask;
        self
    }

    #[inline]
    pub fn name(mut self, name: impl AsRef<str>) -> Self {
        self.name = Some(name.as_ref().to_string());
        self
    }
}

impl<T> Builder<T, u32>
where
    T: Type,
{
    #[inline]
    pub fn build(self) -> windows::core::Result<DescriptorHeap<T>> {
        unsafe {
            let handle = self.device.CreateDescriptorHeap(&self.desc)?;
            let name = self.name.as_ref().map(|name| Name::new(&handle, name));
            let inc = self.device.GetDescriptorHandleIncrementSize(self.desc.Type) as usize;
            Ok(DescriptorHeap(Arc::new(Field {
                device: self.device,
                handle,
                len: self.desc.NumDescriptors as usize,
                name,
                inc,
                _t: self._t,
            })))
        }
    }
}

#[derive(Debug)]
struct Field<T> {
    device: ID3D12Device,
    handle: ID3D12DescriptorHeap,
    len: usize,
    inc: usize,
    name: Option<Name>,
    _t: std::marker::PhantomData<T>,
}

#[derive(Clone, Debug)]
pub struct DescriptorHeap<T>(Arc<Field<T>>);

impl<T> DescriptorHeap<T>
where
    T: Type,
{
    #[inline]
    pub fn new(device: &Device, _ty: T) -> Builder<T> {
        Builder::new(device.handle())
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.0.len
    }

    #[inline]
    pub fn cpu_handle(&self, index: usize) -> CpuDescriptorHandle<T> {
        assert!(index < self.0.len);
        unsafe {
            let mut dh = self.0.handle.GetCPUDescriptorHandleForHeapStart();
            dh.ptr += self.0.inc * index;
            CpuDescriptorHandle {
                heap: self.0.handle.clone(),
                handle: dh,
                _t: std::marker::PhantomData,
            }
        }
    }

    #[inline]
    pub fn gpu_handle(&self, index: usize) -> GpuDescriptorHandle<T> {
        assert!(index < self.0.len);
        unsafe {
            let mut dh = self.0.handle.GetGPUDescriptorHandleForHeapStart();
            dh.ptr += (self.0.inc * index) as u64;
            GpuDescriptorHandle {
                heap: self.0.handle.clone(),
                handle: dh,
                _t: std::marker::PhantomData,
            }
        }
    }

    #[inline]
    pub fn copy_range<R>(&self, src: &DescriptorHeap<T>, src_range: R, dest_start: usize)
    where
        R: std::ops::RangeBounds<usize>,
    {
        let start = match src_range.start_bound() {
            Bound::Unbounded => 0,
            Bound::Included(v) => *v,
            Bound::Excluded(v) => *v + 1,
        };
        let end = match src_range.end_bound() {
            Bound::Unbounded => src.len(),
            Bound::Included(v) => *v + 1,
            Bound::Excluded(v) => *v,
        };
        let len = end - start;
        unsafe {
            self.0.device.CopyDescriptorsSimple(
                len as u32,
                self.cpu_handle(start).handle,
                src.cpu_handle(dest_start).handle,
                T::VALUE,
            );
        }
    }

    #[inline]
    pub fn handle(&self) -> &ID3D12DescriptorHeap {
        &self.0.handle
    }

    #[inline]
    pub fn name(&self) -> Option<&str> {
        self.0.name.as_ref().map(|n| n.as_str())
    }
}

impl<T> PartialEq for DescriptorHeap<T> {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.0.handle == other.0.handle
    }
}

impl<T> Eq for DescriptorHeap<T> {}

impl DescriptorHeap<CbvSrvUav> {
    #[inline]
    pub fn create_constant_buffer_view(
        &mut self,
        index: usize,
        desc: Option<&D3D12_CONSTANT_BUFFER_VIEW_DESC>,
    ) {
        unsafe {
            self.0.device.CreateConstantBufferView(
                desc.map(|d| d as *const _),
                self.cpu_handle(index).handle(),
            );
        }
    }

    #[inline]
    pub fn create_shader_resource_view(
        &mut self,
        index: usize,
        resource: &Resource,
        desc: Option<&D3D12_SHADER_RESOURCE_VIEW_DESC>,
    ) {
        unsafe {
            self.0.device.CreateShaderResourceView(
                resource.handle(),
                desc.map(|d| d as *const _),
                self.cpu_handle(index).handle(),
            );
        }
    }

    #[inline]
    pub fn create_unordered_access_view(
        &mut self,
        index: usize,
        resource: &Resource,
        counter_resource: Option<&Resource>,
        desc: Option<&D3D12_UNORDERED_ACCESS_VIEW_DESC>,
    ) {
        unsafe {
            self.0.device.CreateUnorderedAccessView(
                resource.handle(),
                counter_resource.map(|r| r.handle()),
                desc.map(|d| d as *const _),
                self.cpu_handle(index).handle(),
            );
        }
    }
}

impl DescriptorHeap<Rtv> {
    #[inline]
    pub fn create_view(
        &mut self,
        index: usize,
        resource: &Resource,
        desc: Option<&D3D12_RENDER_TARGET_VIEW_DESC>,
    ) {
        unsafe {
            self.0.device.CreateRenderTargetView(
                resource.handle(),
                desc.map(|d| d as *const _),
                self.cpu_handle(index).handle(),
            );
        }
    }
}

impl DescriptorHeap<Dsv> {
    #[inline]
    pub fn create_view(
        &mut self,
        index: usize,
        resource: &Resource,
        desc: Option<&D3D12_DEPTH_STENCIL_VIEW_DESC>,
    ) {
        unsafe {
            self.0.device.CreateDepthStencilView(
                resource.handle(),
                desc.map(|d| d as *const _),
                self.cpu_handle(index).handle(),
            );
        }
    }
}

impl DescriptorHeap<Sampler> {
    #[inline]
    pub fn create_sampler(&self, index: usize, desc: &D3D12_SAMPLER_DESC) {
        unsafe {
            self.0
                .device
                .CreateSampler(desc, self.cpu_handle(index).handle());
        }
    }
}
