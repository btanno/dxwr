use super::*;
use windows::Win32::Graphics::Direct3D12::*;
use windows::Win32::Graphics::Dxgi::Common::*;

#[derive(Clone, Debug)]
pub struct HeapProperties(D3D12_HEAP_PROPERTIES);

impl HeapProperties {
    #[inline]
    #[allow(clippy::should_implement_trait)]
    pub fn default() -> Self {
        Self(D3D12_HEAP_PROPERTIES {
            Type: D3D12_HEAP_TYPE_DEFAULT,
            CreationNodeMask: 1,
            VisibleNodeMask: 1,
            ..Default::default()
        })
    }

    #[inline]
    pub fn upload() -> Self {
        Self(D3D12_HEAP_PROPERTIES {
            Type: D3D12_HEAP_TYPE_UPLOAD,
            CreationNodeMask: 1,
            VisibleNodeMask: 1,
            ..Default::default()
        })
    }

    #[inline]
    pub fn readback() -> Self {
        Self(D3D12_HEAP_PROPERTIES {
            Type: D3D12_HEAP_TYPE_READBACK,
            CreationNodeMask: 1,
            VisibleNodeMask: 1,
            ..Default::default()
        })
    }

    #[inline]
    pub fn custom() -> Self {
        Self(D3D12_HEAP_PROPERTIES {
            Type: D3D12_HEAP_TYPE_CUSTOM,
            CreationNodeMask: 1,
            VisibleNodeMask: 1,
            ..Default::default()
        })
    }

    #[inline]
    pub fn gpu_upload() -> Self {
        Self(D3D12_HEAP_PROPERTIES {
            Type: D3D12_HEAP_TYPE_GPU_UPLOAD,
            CreationNodeMask: 1,
            VisibleNodeMask: 1,
            ..Default::default()
        })
    }

    #[inline]
    pub fn cpu_page_property(mut self, value: D3D12_CPU_PAGE_PROPERTY) -> Self {
        self.0.CPUPageProperty = value;
        self
    }

    #[inline]
    pub fn creation_node_mask(mut self, mask: u32) -> Self {
        self.0.CreationNodeMask = mask;
        self
    }

    #[inline]
    pub fn visible_node_mask(mut self, mask: u32) -> Self {
        self.0.VisibleNodeMask = mask;
        self
    }
}

pub mod dimension {
    pub struct Buffer;
    pub struct Texture1D;
    pub struct Texture2D;
    pub struct Texture3D;
}

#[derive(Clone, Debug)]
pub struct ResourceDesc<T = ()> {
    desc: D3D12_RESOURCE_DESC,
    _t: std::marker::PhantomData<T>,
}

impl ResourceDesc<()> {
    #[inline]
    pub fn buffer() -> ResourceDesc<dimension::Buffer> {
        ResourceDesc {
            desc: D3D12_RESOURCE_DESC {
                Dimension: D3D12_RESOURCE_DIMENSION_BUFFER,
                Height: 1,
                DepthOrArraySize: 1,
                MipLevels: 1,
                Layout: D3D12_TEXTURE_LAYOUT_ROW_MAJOR,
                SampleDesc: DXGI_SAMPLE_DESC {
                    Count: 1,
                    Quality: 0,
                },
                ..Default::default()
            },
            _t: std::marker::PhantomData,
        }
    }

    #[inline]
    pub fn texture1d() -> ResourceDesc<dimension::Texture1D> {
        ResourceDesc {
            desc: D3D12_RESOURCE_DESC {
                Dimension: D3D12_RESOURCE_DIMENSION_TEXTURE1D,
                Height: 1,
                DepthOrArraySize: 1,
                SampleDesc: DXGI_SAMPLE_DESC {
                    Count: 1,
                    Quality: 0,
                },
                ..Default::default()
            },
            _t: std::marker::PhantomData,
        }
    }

    #[inline]
    pub fn texture2d() -> ResourceDesc<dimension::Texture2D> {
        ResourceDesc {
            desc: D3D12_RESOURCE_DESC {
                Dimension: D3D12_RESOURCE_DIMENSION_TEXTURE2D,
                DepthOrArraySize: 1,
                SampleDesc: DXGI_SAMPLE_DESC {
                    Count: 1,
                    Quality: 0,
                },
                ..Default::default()
            },
            _t: std::marker::PhantomData,
        }
    }
}

impl<T> ResourceDesc<T> {
    #[inline]
    pub fn alignment(mut self, alignment: u64) -> Self {
        self.desc.Alignment = alignment;
        self
    }

    #[inline]
    pub fn format(mut self, format: DXGI_FORMAT) -> Self {
        self.desc.Format = format;
        self
    }

    #[inline]
    pub fn layout(mut self, layout: D3D12_TEXTURE_LAYOUT) -> Self {
        self.desc.Layout = layout;
        self
    }

    #[inline]
    pub fn flags(mut self, flags: D3D12_RESOURCE_FLAGS) -> Self {
        self.desc.Flags = flags;
        self
    }
}

impl ResourceDesc<dimension::Buffer> {
    #[inline]
    pub fn width(mut self, width: u64) -> Self {
        self.desc.Width = width;
        self
    }
}

impl ResourceDesc<dimension::Texture1D> {
    #[inline]
    pub fn width(mut self, width: u64) -> Self {
        self.desc.Width = width;
        self
    }

    #[inline]
    pub fn array_size(mut self, size: u16) -> Self {
        self.desc.DepthOrArraySize = size;
        self
    }

    #[inline]
    pub fn mip_levels(mut self, levels: u16) -> Self {
        self.desc.MipLevels = levels;
        self
    }

    #[inline]
    pub fn sample_desc(mut self, desc: SampleDesc) -> Self {
        self.desc.SampleDesc = desc.0;
        self
    }
}

impl ResourceDesc<dimension::Texture2D> {
    #[inline]
    pub fn width(mut self, width: u64) -> Self {
        self.desc.Width = width;
        self
    }

    #[inline]
    pub fn height(mut self, height: u32) -> Self {
        self.desc.Height = height;
        self
    }

    #[inline]
    pub fn array_size(mut self, array_size: u16) -> Self {
        self.desc.DepthOrArraySize = array_size;
        self
    }

    #[inline]
    pub fn mip_levels(mut self, levels: u16) -> Self {
        self.desc.MipLevels = levels;
        self
    }

    #[inline]
    pub fn sample_desc(mut self, desc: SampleDesc) -> Self {
        self.desc.SampleDesc = desc.0;
        self
    }
}

impl ResourceDesc<dimension::Texture3D> {
    #[inline]
    pub fn width(mut self, width: u64) -> Self {
        self.desc.Width = width;
        self
    }

    #[inline]
    pub fn height(mut self, height: u32) -> Self {
        self.desc.Height = height;
        self
    }

    #[inline]
    pub fn depth(mut self, depth: u16) -> Self {
        self.desc.DepthOrArraySize = depth;
        self
    }

    #[inline]
    pub fn mip_levels(mut self, levels: u16) -> Self {
        self.desc.MipLevels = levels;
        self
    }

    #[inline]
    pub fn sample_desc(mut self, desc: SampleDesc) -> Self {
        self.desc.SampleDesc = desc.0;
        self
    }
}

pub struct MappedData<'a> {
    resource: ID3D12Resource,
    subresource: u32,
    data: *mut std::ffi::c_void,
    _a: std::marker::PhantomData<&'a ()>,
}

impl<'a> MappedData<'a> {
    fn new(resource: &ID3D12Resource, subresource: u32) -> windows::core::Result<Self> {
        let mut p = std::ptr::null_mut();
        unsafe {
            resource.Map(subresource, None, Some(&mut p))?;
            Ok(Self {
                resource: resource.clone(),
                subresource,
                data: p,
                _a: std::marker::PhantomData,
            })
        }
    }

    #[inline]
    pub unsafe fn as_ref<T>(&self) -> &'a T {
        self.data.cast::<T>().as_ref().unwrap()
    }

    #[inline]
    pub unsafe fn as_mut<T>(&self) -> &'a mut T {
        self.data.cast::<T>().as_mut().unwrap()
    }

    #[inline]
    pub unsafe fn as_slice<T>(&self, len: usize) -> &'a [T] {
        std::slice::from_raw_parts(self.data as *const T, len)
    }

    #[inline]
    pub unsafe fn as_slice_mut<T>(&self, len: usize) -> &'a mut [T] {
        std::slice::from_raw_parts_mut(self.data as *mut T, len)
    }

    #[inline]
    pub fn as_ptr(&self) -> *const std::ffi::c_void {
        self.data
    }

    #[inline]
    pub fn as_mut_ptr(&self) -> *mut std::ffi::c_void {
        self.data
    }
}

impl<'a> Drop for MappedData<'a> {
    fn drop(&mut self) {
        unsafe {
            self.resource.Unmap(self.subresource, None);
        }
    }
}

pub struct Builder<HeapProps = (), Desc = ()> {
    device: ID3D12Device,
    heap_props: HeapProps,
    heap_flags: D3D12_HEAP_FLAGS,
    desc: Desc,
    init_state: D3D12_RESOURCE_STATES,
    clear_value: Option<D3D12_CLEAR_VALUE>,
    name: Option<String>,
}

impl Builder<(), ()> {
    fn new(device: ID3D12Device) -> Self {
        Self {
            device,
            heap_props: (),
            heap_flags: D3D12_HEAP_FLAG_NONE,
            desc: (),
            init_state: D3D12_RESOURCE_STATE_COMMON,
            clear_value: None,
            name: None,
        }
    }
}

impl<HeapProps, Desc> Builder<HeapProps, Desc> {
    #[inline]
    pub fn heap_properties(self, heap_props: &HeapProperties) -> Builder<&HeapProperties, Desc> {
        Builder {
            device: self.device,
            heap_props,
            heap_flags: self.heap_flags,
            desc: self.desc,
            init_state: self.init_state,
            clear_value: self.clear_value,
            name: self.name,
        }
    }

    #[inline]
    pub fn heap_flags(mut self, flags: D3D12_HEAP_FLAGS) -> Self {
        self.heap_flags = flags;
        self
    }

    #[inline]
    pub fn resource_desc<T>(self, desc: &ResourceDesc<T>) -> Builder<HeapProps, &ResourceDesc<T>> {
        Builder {
            device: self.device,
            heap_props: self.heap_props,
            heap_flags: self.heap_flags,
            desc,
            init_state: self.init_state,
            clear_value: self.clear_value,
            name: self.name,
        }
    }

    #[inline]
    pub fn init_state(mut self, state: D3D12_RESOURCE_STATES) -> Self {
        self.init_state = state;
        self
    }

    #[inline]
    pub fn clear_value(mut self, value: D3D12_CLEAR_VALUE) -> Self {
        self.clear_value = Some(value);
        self
    }

    #[inline]
    pub fn name(mut self, name: impl AsRef<str>) -> Self {
        self.name = Some(name.as_ref().to_string());
        self
    }
}

impl<'a, 'b, T> Builder<&'a HeapProperties, &'b ResourceDesc<T>> {
    #[inline]
    pub fn build(self) -> windows::core::Result<Resource> {
        let handle = unsafe {
            let mut p: Option<ID3D12Resource> = None;
            self.device
                .CreateCommittedResource(
                    &self.heap_props.0,
                    self.heap_flags,
                    &self.desc.desc,
                    self.init_state,
                    self.clear_value.as_ref().map(|v| v as *const _),
                    &mut p,
                )
                .map(|_| p.unwrap())?
        };
        let name = self.name.map(|n| Name::new(&handle, n));
        Ok(Resource { handle, name })
    }
}

#[derive(Clone, Debug)]
pub struct Resource {
    handle: ID3D12Resource,
    name: Option<Name>,
}

impl Resource {
    #[inline]
    pub fn new(device: &Device) -> Builder {
        Builder::new(device.handle().clone().into())
    }

    #[inline]
    pub(crate) fn from_raw(handle: ID3D12Resource) -> Self {
        Self { handle, name: None }
    }

    #[inline]
    pub fn map(&self, subresource: u32) -> windows::core::Result<MappedData> {
        MappedData::new(&self.handle, subresource)
    }

    #[inline]
    pub fn get_gpu_virtual_address(&self) -> u64 {
        unsafe { self.handle.GetGPUVirtualAddress() }
    }

    #[inline]
    pub fn handle(&self) -> &ID3D12Resource {
        &self.handle
    }

    #[inline]
    pub fn name(&self) -> Option<&str> {
        self.name.as_ref().map(|n| n.as_str())
    }
}

impl PartialEq for Resource {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.handle == other.handle
    }
}

impl Eq for Resource {}
