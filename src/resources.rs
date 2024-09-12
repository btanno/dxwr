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
#[repr(transparent)]
pub struct ResourceDesc<T = ()> {
    pub(crate) desc: D3D12_RESOURCE_DESC,
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

#[derive(Clone)]
pub struct ClearValue(D3D12_CLEAR_VALUE);

impl ClearValue {
    #[inline]
    pub fn color(format: DXGI_FORMAT, color: [f32; 4]) -> Self {
        Self(D3D12_CLEAR_VALUE {
            Format: format,
            Anonymous: D3D12_CLEAR_VALUE_0 { Color: color },
        })
    }

    #[inline]
    pub fn depth_stencil(format: DXGI_FORMAT, depth: f32, stencil: u8) -> Self {
        assert!(
            format == DXGI_FORMAT_D16_UNORM
                || format == DXGI_FORMAT_D32_FLOAT
                || format == DXGI_FORMAT_D24_UNORM_S8_UINT
                || format == DXGI_FORMAT_D32_FLOAT_S8X24_UINT
        );
        Self(D3D12_CLEAR_VALUE {
            Format: format,
            Anonymous: D3D12_CLEAR_VALUE_0 {
                DepthStencil: D3D12_DEPTH_STENCIL_VALUE {
                    Depth: depth,
                    Stencil: stencil,
                },
            },
        })
    }
}

impl std::fmt::Debug for ClearValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.0.Format {
            DXGI_FORMAT_D16_UNORM
            | DXGI_FORMAT_D32_FLOAT
            | DXGI_FORMAT_D24_UNORM_S8_UINT
            | DXGI_FORMAT_D32_FLOAT_S8X24_UINT => {
                write!(
                    f,
                    "ClearValue {{ Format: {:?}, DepthStencil: {:?} }}",
                    self.0.Format,
                    unsafe { self.0.Anonymous.DepthStencil }
                )
            }
            _ => {
                write!(
                    f,
                    "ClearValue {{ Format: {:?}, Color: {:?} }}",
                    self.0.Format,
                    unsafe { self.0.Anonymous.Color }
                )
            }
        }
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
    clear_value: Option<ClearValue>,
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
    pub fn clear_value(mut self, value: ClearValue) -> Self {
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
                    self.clear_value.as_ref().map(|v| &v.0 as *const _),
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
    pub fn from_heap(device: &Device) -> placed_resource::Builder {
        placed_resource::Builder::new(device)
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
    pub fn get_gpu_virtual_address(&self) -> GpuVirtualAddress {
        unsafe { GpuVirtualAddress(self.handle.GetGPUVirtualAddress()) }
    }

    #[inline]
    pub fn handle(&self) -> &ID3D12Resource {
        &self.handle
    }

    #[inline]
    pub fn name(&self) -> Option<&str> {
        self.name.as_ref().map(|n| n.as_str())
    }
    
    #[inline]
    pub fn set_name(&mut self, name: impl AsRef<str>) {
        self.name = Some(Name::new(self.handle(), name));
    }
}

impl PartialEq for Resource {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.handle == other.handle
    }
}

impl Eq for Resource {}

pub mod heap {
    use super::*;

    #[derive(Debug)]
    pub struct Builder<Size = (), HeapProps = (), Align = ()> {
        device: Device,
        size: Size,
        heap_properties: HeapProps,
        alignment: Align,
        flags: D3D12_HEAP_FLAGS,
        name: Option<String>,
    }

    impl Builder<(), (), ()> {
        pub(super) fn new(device: &Device) -> Self {
            Self {
                device: device.clone(),
                size: (),
                heap_properties: (),
                alignment: (),
                flags: D3D12_HEAP_FLAG_NONE,
                name: None,
            }
        }
    }

    impl<Size, HeapProps, Align> Builder<Size, HeapProps, Align> {
        #[inline]
        pub fn size(self, size: u64) -> Builder<u64, HeapProps, Align> {
            Builder {
                device: self.device,
                size,
                heap_properties: self.heap_properties,
                alignment: self.alignment,
                flags: self.flags,
                name: self.name,
            }
        }

        #[inline]
        pub fn heap_properties(
            self,
            heap_properties: &HeapProperties,
        ) -> Builder<Size, &HeapProperties, Align> {
            Builder {
                device: self.device,
                size: self.size,
                heap_properties,
                alignment: self.alignment,
                flags: self.flags,
                name: self.name,
            }
        }

        #[inline]
        pub fn alignment(self, alignment: u64) -> Builder<Size, HeapProps, u64> {
            Builder {
                device: self.device,
                size: self.size,
                heap_properties: self.heap_properties,
                alignment,
                flags: self.flags,
                name: self.name,
            }
        }

        #[inline]
        pub fn flags(mut self, flags: D3D12_HEAP_FLAGS) -> Builder<Size, HeapProps, Align> {
            self.flags = flags;
            self
        }

        #[inline]
        pub fn name(mut self, name: impl AsRef<str>) -> Self {
            self.name = Some(name.as_ref().to_string());
            self
        }
    }

    impl Builder<u64, &HeapProperties, u64> {
        #[inline]
        pub fn build(self) -> windows::core::Result<Heap> {
            let handle = unsafe {
                let mut p: Option<ID3D12Heap> = None;
                self.device
                    .handle()
                    .CreateHeap(
                        &D3D12_HEAP_DESC {
                            SizeInBytes: self.size,
                            Properties: self.heap_properties.0.clone(),
                            Alignment: self.alignment,
                            Flags: self.flags,
                        },
                        &mut p,
                    )
                    .map(|_| p.unwrap())?
            };
            let name = self.name.map(|n| Name::new(&handle, n));
            Ok(Heap { handle, name })
        }
    }
}

#[derive(Debug)]
pub struct Heap {
    handle: ID3D12Heap,
    name: Option<Name>,
}

impl Heap {
    #[inline]
    pub fn new(device: &Device) -> heap::Builder {
        heap::Builder::new(device)
    }

    #[inline]
    pub fn name(&self) -> Option<&str> {
        self.name.as_ref().map(|n| n.as_str())
    }

    #[inline]
    pub fn handle(&self) -> &ID3D12Heap {
        &self.handle
    }
}

pub mod placed_resource {
    use super::*;

    #[derive(Debug)]
    pub struct Builder<H = (), Rd = ()> {
        device: Device,
        heap: H,
        offset: u64,
        resource_desc: Rd,
        init_state: D3D12_RESOURCE_STATES,
        clear_value: Option<ClearValue>,
        name: Option<String>,
    }

    impl Builder<()> {
        pub(super) fn new(device: &Device) -> Self {
            Self {
                device: device.clone(),
                heap: (),
                offset: 0,
                resource_desc: (),
                init_state: D3D12_RESOURCE_STATE_COMMON,
                clear_value: None,
                name: None,
            }
        }
    }

    impl<H, Rd> Builder<H, Rd> {
        #[inline]
        pub fn heap(self, heap: &Heap) -> Builder<&Heap, Rd> {
            Builder {
                device: self.device,
                heap,
                offset: self.offset,
                resource_desc: self.resource_desc,
                init_state: self.init_state,
                clear_value: self.clear_value,
                name: self.name,
            }
        }

        #[inline]
        pub fn offset(mut self, offset: u64) -> Self {
            self.offset = offset;
            self
        }

        #[inline]
        pub fn resource_desc(self, resource_desc: &ResourceDesc) -> Builder<H, &ResourceDesc> {
            Builder {
                device: self.device,
                heap: self.heap,
                offset: self.offset,
                resource_desc,
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
        pub fn clear_value(mut self, value: ClearValue) -> Self {
            self.clear_value = Some(value);
            self
        }

        #[inline]
        pub fn name(mut self, name: impl AsRef<str>) -> Self {
            self.name = Some(name.as_ref().to_string());
            self
        }
    }

    impl Builder<&Heap, &ResourceDesc> {
        #[inline]
        pub fn build(self) -> windows::core::Result<Resource> {
            let handle = unsafe {
                let mut p: Option<ID3D12Resource> = None;
                self.device
                    .handle()
                    .CreatePlacedResource(
                        self.heap.handle(),
                        self.offset,
                        &self.resource_desc.desc,
                        self.init_state,
                        self.clear_value.as_ref().map(|c| &c.0 as *const _),
                        &mut p,
                    )
                    .map(|_| p.unwrap())?
            };
            let name = self.name.map(|n| Name::new(&handle, n));
            Ok(Resource { handle, name })
        }
    }
}
