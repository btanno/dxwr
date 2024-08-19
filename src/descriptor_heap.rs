use super::*;
use std::ops::Bound;
use std::sync::Arc;
use windows::Win32::Graphics::{Direct3D12::*, Dxgi::Common::DXGI_FORMAT};

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

pub mod dimension {
    #[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
    pub struct Buffer;

    #[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
    pub struct Texture1D;

    #[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
    pub struct Texture1DArray;

    #[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
    pub struct Texture2D;

    #[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
    pub struct Texture2DArray;

    #[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
    pub struct Texture2DMs;

    #[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
    pub struct Texture2DMsArray;

    #[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
    pub struct Texture3D;

    #[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
    pub struct TextureCube;

    #[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
    pub struct TextureCubeArray;

    #[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
    pub struct RaytracingAccelerationStructure;
}

#[repr(transparent)]
pub struct ConstantBufferViewDesc(D3D12_CONSTANT_BUFFER_VIEW_DESC);

impl ConstantBufferViewDesc {
    #[inline]
    pub fn new() -> Self {
        Self(D3D12_CONSTANT_BUFFER_VIEW_DESC::default())
    }

    #[inline]
    pub fn buffer_location(mut self, loc: u64) -> Self {
        self.0.BufferLocation = loc;
        self
    }

    #[inline]
    pub fn size_in_bytes(mut self, size: u32) -> Self {
        self.0.SizeInBytes = size;
        self
    }
}

#[repr(transparent)]
pub struct ShaderResourceViewDesc<T = ()> {
    desc: D3D12_SHADER_RESOURCE_VIEW_DESC,
    _t: std::marker::PhantomData<T>,
}

impl ShaderResourceViewDesc<()> {
    fn new<T>(dimension: D3D12_SRV_DIMENSION) -> ShaderResourceViewDesc<T> {
        ShaderResourceViewDesc {
            desc: D3D12_SHADER_RESOURCE_VIEW_DESC {
                ViewDimension: dimension,
                Shader4ComponentMapping: D3D12_DEFAULT_SHADER_4_COMPONENT_MAPPING,
                ..Default::default()
            },
            _t: std::marker::PhantomData,
        }
    }

    #[inline]
    pub fn buffer() -> ShaderResourceViewDesc<dimension::Buffer> {
        Self::new(D3D12_SRV_DIMENSION_BUFFER)
    }

    #[inline]
    pub fn texture1d() -> ShaderResourceViewDesc<dimension::Texture1D> {
        Self::new(D3D12_SRV_DIMENSION_TEXTURE1D)
    }

    #[inline]
    pub fn texture1d_array() -> ShaderResourceViewDesc<dimension::Texture1DArray> {
        Self::new(D3D12_SRV_DIMENSION_TEXTURE1DARRAY)
    }

    #[inline]
    pub fn texture2d() -> ShaderResourceViewDesc<dimension::Texture2D> {
        Self::new(D3D12_SRV_DIMENSION_TEXTURE2D)
    }

    #[inline]
    pub fn texture2d_array() -> ShaderResourceViewDesc<dimension::Texture2DArray> {
        Self::new(D3D12_SRV_DIMENSION_TEXTURE2DARRAY)
    }

    #[inline]
    pub fn texture2d_ms() -> ShaderResourceViewDesc<dimension::Texture2DMs> {
        Self::new(D3D12_SRV_DIMENSION_TEXTURE2DMS)
    }

    #[inline]
    pub fn texture2d_ms_array() -> ShaderResourceViewDesc<dimension::Texture2DMsArray> {
        Self::new(D3D12_SRV_DIMENSION_TEXTURE2DMSARRAY)
    }

    #[inline]
    pub fn texture3d() -> ShaderResourceViewDesc<dimension::Texture3D> {
        Self::new(D3D12_SRV_DIMENSION_TEXTURE3D)
    }

    #[inline]
    pub fn texture_cube() -> ShaderResourceViewDesc<dimension::TextureCube> {
        Self::new(D3D12_SRV_DIMENSION_TEXTURECUBE)
    }

    #[inline]
    pub fn texture_cube_array() -> ShaderResourceViewDesc<dimension::TextureCubeArray> {
        Self::new(D3D12_SRV_DIMENSION_TEXTURECUBEARRAY)
    }

    #[inline]
    pub fn raytracing_acceleration_structure(
    ) -> ShaderResourceViewDesc<dimension::RaytracingAccelerationStructure> {
        Self::new(D3D12_SRV_DIMENSION_RAYTRACING_ACCELERATION_STRUCTURE)
    }

    #[inline]
    pub fn none() -> Option<&'static Self> {
        None
    }
}

impl<T> ShaderResourceViewDesc<T> {
    #[inline]
    pub fn format(mut self, format: DXGI_FORMAT) -> Self {
        self.desc.Format = format;
        self
    }

    #[inline]
    pub fn shader_4_component_mapping(mut self, mapping: u32) -> Self {
        self.desc.Shader4ComponentMapping = mapping;
        self
    }
}

impl ShaderResourceViewDesc<dimension::Buffer> {
    #[inline]
    pub fn first_element(mut self, element: u64) -> Self {
        self.desc.Anonymous.Buffer.FirstElement = element;
        self
    }

    #[inline]
    pub fn num_elements(mut self, n: u32) -> Self {
        self.desc.Anonymous.Buffer.NumElements = n;
        self
    }

    #[inline]
    pub fn structure_byte_stride(mut self, stride: u32) -> Self {
        self.desc.Anonymous.Buffer.StructureByteStride = stride;
        self
    }

    #[inline]
    pub fn flags(mut self, flags: D3D12_BUFFER_SRV_FLAGS) -> Self {
        self.desc.Anonymous.Buffer.Flags = flags;
        self
    }
}

impl ShaderResourceViewDesc<dimension::Texture1D> {
    #[inline]
    pub fn most_detailed_mip(mut self, mip: u32) -> Self {
        self.desc.Anonymous.Texture1D.MostDetailedMip = mip;
        self
    }

    #[inline]
    pub fn mip_levels(mut self, levels: u32) -> Self {
        self.desc.Anonymous.Texture1D.MipLevels = levels;
        self
    }

    #[inline]
    pub fn resource_min_lod_clamp(mut self, clamp: f32) -> Self {
        self.desc.Anonymous.Texture1D.ResourceMinLODClamp = clamp;
        self
    }
}

impl ShaderResourceViewDesc<dimension::Texture1DArray> {
    #[inline]
    pub fn most_detailed_mip(mut self, mip: u32) -> Self {
        self.desc.Anonymous.Texture1DArray.MostDetailedMip = mip;
        self
    }

    #[inline]
    pub fn mip_levels(mut self, levels: u32) -> Self {
        self.desc.Anonymous.Texture1DArray.MipLevels = levels;
        self
    }

    #[inline]
    pub fn resource_min_lod_clamp(mut self, clamp: f32) -> Self {
        self.desc.Anonymous.Texture1DArray.ResourceMinLODClamp = clamp;
        self
    }

    #[inline]
    pub fn first_array_slice(mut self, slice: u32) -> Self {
        self.desc.Anonymous.Texture1DArray.FirstArraySlice = slice;
        self
    }

    #[inline]
    pub fn array_size(mut self, size: u32) -> Self {
        self.desc.Anonymous.Texture1DArray.ArraySize = size;
        self
    }
}

impl ShaderResourceViewDesc<dimension::Texture2D> {
    #[inline]
    pub fn most_detailed_mip(mut self, mip: u32) -> Self {
        self.desc.Anonymous.Texture2D.MostDetailedMip = mip;
        self
    }

    #[inline]
    pub fn mip_levels(mut self, levels: u32) -> Self {
        self.desc.Anonymous.Texture2D.MipLevels = levels;
        self
    }

    #[inline]
    pub fn resource_min_lod_clamp(mut self, clamp: f32) -> Self {
        self.desc.Anonymous.Texture2D.ResourceMinLODClamp = clamp;
        self
    }

    #[inline]
    pub fn plane_slice(mut self, slice: u32) -> Self {
        self.desc.Anonymous.Texture2D.PlaneSlice = slice;
        self
    }
}

impl ShaderResourceViewDesc<dimension::Texture2DArray> {
    #[inline]
    pub fn most_detailed_mip(mut self, mip: u32) -> Self {
        self.desc.Anonymous.Texture2DArray.MostDetailedMip = mip;
        self
    }

    #[inline]
    pub fn mip_levels(mut self, levels: u32) -> Self {
        self.desc.Anonymous.Texture2DArray.MipLevels = levels;
        self
    }

    #[inline]
    pub fn resource_min_lod_clamp(mut self, clamp: f32) -> Self {
        self.desc.Anonymous.Texture2DArray.ResourceMinLODClamp = clamp;
        self
    }

    #[inline]
    pub fn plane_slice(mut self, slice: u32) -> Self {
        self.desc.Anonymous.Texture2DArray.PlaneSlice = slice;
        self
    }

    #[inline]
    pub fn first_array_slice(mut self, slice: u32) -> Self {
        self.desc.Anonymous.Texture2DArray.FirstArraySlice = slice;
        self
    }

    #[inline]
    pub fn array_size(mut self, size: u32) -> Self {
        self.desc.Anonymous.Texture2DArray.ArraySize = size;
        self
    }
}

impl ShaderResourceViewDesc<dimension::Texture2DMsArray> {
    #[inline]
    pub fn first_array_slice(mut self, slice: u32) -> Self {
        self.desc.Anonymous.Texture2DMSArray.FirstArraySlice = slice;
        self
    }

    #[inline]
    pub fn array_size(mut self, size: u32) -> Self {
        self.desc.Anonymous.Texture2DMSArray.ArraySize = size;
        self
    }
}

impl ShaderResourceViewDesc<dimension::Texture3D> {
    #[inline]
    pub fn most_detailed_mip(mut self, mip: u32) -> Self {
        self.desc.Anonymous.Texture3D.MostDetailedMip = mip;
        self
    }

    #[inline]
    pub fn mip_levels(mut self, levels: u32) -> Self {
        self.desc.Anonymous.Texture3D.MipLevels = levels;
        self
    }

    #[inline]
    pub fn resource_min_lod_clamp(mut self, clamp: f32) -> Self {
        self.desc.Anonymous.Texture3D.ResourceMinLODClamp = clamp;
        self
    }
}

impl ShaderResourceViewDesc<dimension::TextureCube> {
    #[inline]
    pub fn most_detailed_mip(mut self, mip: u32) -> Self {
        self.desc.Anonymous.TextureCube.MostDetailedMip = mip;
        self
    }

    #[inline]
    pub fn mip_levels(mut self, levels: u32) -> Self {
        self.desc.Anonymous.TextureCube.MipLevels = levels;
        self
    }

    #[inline]
    pub fn resource_min_lod_clamp(mut self, clamp: f32) -> Self {
        self.desc.Anonymous.TextureCube.ResourceMinLODClamp = clamp;
        self
    }
}

impl ShaderResourceViewDesc<dimension::TextureCubeArray> {
    #[inline]
    pub fn most_detailed_mip(mut self, mip: u32) -> Self {
        self.desc.Anonymous.TextureCube.MostDetailedMip = mip;
        self
    }

    #[inline]
    pub fn mip_levels(mut self, levels: u32) -> Self {
        self.desc.Anonymous.TextureCube.MipLevels = levels;
        self
    }

    #[inline]
    pub fn resource_min_lod_clamp(mut self, clamp: f32) -> Self {
        self.desc.Anonymous.TextureCube.ResourceMinLODClamp = clamp;
        self
    }

    #[inline]
    pub fn first_2d_array_face(mut self, face: u32) -> Self {
        self.desc.Anonymous.TextureCubeArray.First2DArrayFace = face;
        self
    }

    #[inline]
    pub fn num_cubes(mut self, n: u32) -> Self {
        self.desc.Anonymous.TextureCubeArray.NumCubes = n;
        self
    }
}

impl ShaderResourceViewDesc<dimension::RaytracingAccelerationStructure> {
    #[inline]
    pub fn location(mut self, loc: u64) -> Self {
        self.desc.Anonymous.RaytracingAccelerationStructure.Location = loc;
        self
    }
}

#[repr(transparent)]
pub struct UnorderedAccessViewDesc<T = ()> {
    desc: D3D12_UNORDERED_ACCESS_VIEW_DESC,
    _t: std::marker::PhantomData<T>,
}

impl UnorderedAccessViewDesc<()> {
    fn new<T>(dimension: D3D12_UAV_DIMENSION) -> UnorderedAccessViewDesc<T> {
        UnorderedAccessViewDesc {
            desc: D3D12_UNORDERED_ACCESS_VIEW_DESC {
                ViewDimension: dimension,
                ..Default::default()
            },
            _t: std::marker::PhantomData,
        }
    }

    #[inline]
    pub fn buffer() -> UnorderedAccessViewDesc<dimension::Buffer> {
        Self::new(D3D12_UAV_DIMENSION_BUFFER)
    }

    #[inline]
    pub fn texture1d() -> UnorderedAccessViewDesc<dimension::Texture1D> {
        Self::new(D3D12_UAV_DIMENSION_TEXTURE1D)
    }

    #[inline]
    pub fn texture1d_array() -> UnorderedAccessViewDesc<dimension::Texture1DArray> {
        Self::new(D3D12_UAV_DIMENSION_TEXTURE1DARRAY)
    }

    #[inline]
    pub fn texture2d() -> UnorderedAccessViewDesc<dimension::Texture2D> {
        Self::new(D3D12_UAV_DIMENSION_TEXTURE2D)
    }

    #[inline]
    pub fn texture2d_array() -> UnorderedAccessViewDesc<dimension::Texture2DArray> {
        Self::new(D3D12_UAV_DIMENSION_TEXTURE2DARRAY)
    }

    #[inline]
    pub fn texture2d_ms() -> UnorderedAccessViewDesc<dimension::Texture2DMs> {
        Self::new(D3D12_UAV_DIMENSION_TEXTURE2DMS)
    }

    #[inline]
    pub fn texture2d_ms_array() -> UnorderedAccessViewDesc<dimension::Texture2DMsArray> {
        Self::new(D3D12_UAV_DIMENSION_TEXTURE2DMSARRAY)
    }

    #[inline]
    pub fn texture3d() -> UnorderedAccessViewDesc<dimension::Texture3D> {
        Self::new(D3D12_UAV_DIMENSION_TEXTURE3D)
    }

    #[inline]
    pub fn none() -> Option<&'static Self> {
        None
    }
}

impl<T> UnorderedAccessViewDesc<T> {
    #[inline]
    pub fn format(mut self, format: DXGI_FORMAT) -> Self {
        self.desc.Format = format;
        self
    }
}

impl UnorderedAccessViewDesc<dimension::Buffer> {
    #[inline]
    pub fn first_element(mut self, element: u64) -> Self {
        self.desc.Anonymous.Buffer.FirstElement = element;
        self
    }

    #[inline]
    pub fn num_elements(mut self, n: u32) -> Self {
        self.desc.Anonymous.Buffer.NumElements = n;
        self
    }

    #[inline]
    pub fn structure_byte_stride(mut self, stride: u32) -> Self {
        self.desc.Anonymous.Buffer.StructureByteStride = stride;
        self
    }

    #[inline]
    pub fn counter_offset_in_bytes(mut self, offset: u64) -> Self {
        self.desc.Anonymous.Buffer.CounterOffsetInBytes = offset;
        self
    }

    #[inline]
    pub fn flags(mut self, flags: D3D12_BUFFER_UAV_FLAGS) -> Self {
        self.desc.Anonymous.Buffer.Flags = flags;
        self
    }
}

impl UnorderedAccessViewDesc<dimension::Texture1D> {
    #[inline]
    pub fn mip_slice(mut self, slice: u32) -> Self {
        self.desc.Anonymous.Texture1D.MipSlice = slice;
        self
    }
}

impl UnorderedAccessViewDesc<dimension::Texture1DArray> {
    #[inline]
    pub fn mip_slice(mut self, slice: u32) -> Self {
        self.desc.Anonymous.Texture1DArray.MipSlice = slice;
        self
    }

    #[inline]
    pub fn first_array_slice(mut self, slice: u32) -> Self {
        self.desc.Anonymous.Texture1DArray.FirstArraySlice = slice;
        self
    }

    #[inline]
    pub fn array_size(mut self, size: u32) -> Self {
        self.desc.Anonymous.Texture1DArray.ArraySize = size;
        self
    }
}

impl UnorderedAccessViewDesc<dimension::Texture2D> {
    #[inline]
    pub fn mip_slice(mut self, slice: u32) -> Self {
        self.desc.Anonymous.Texture2D.MipSlice = slice;
        self
    }

    #[inline]
    pub fn plane_slice(mut self, slice: u32) -> Self {
        self.desc.Anonymous.Texture2D.PlaneSlice = slice;
        self
    }
}

impl UnorderedAccessViewDesc<dimension::Texture2DArray> {
    #[inline]
    pub fn mip_slice(mut self, slice: u32) -> Self {
        self.desc.Anonymous.Texture2DArray.MipSlice = slice;
        self
    }

    #[inline]
    pub fn plane_slice(mut self, slice: u32) -> Self {
        self.desc.Anonymous.Texture2DArray.PlaneSlice = slice;
        self
    }

    #[inline]
    pub fn first_array_slice(mut self, slice: u32) -> Self {
        self.desc.Anonymous.Texture2DArray.FirstArraySlice = slice;
        self
    }

    #[inline]
    pub fn array_size(mut self, size: u32) -> Self {
        self.desc.Anonymous.Texture2DArray.ArraySize = size;
        self
    }
}

impl UnorderedAccessViewDesc<dimension::Texture2DMsArray> {
    #[inline]
    pub fn first_array_slice(mut self, slice: u32) -> Self {
        self.desc.Anonymous.Texture2DMSArray.FirstArraySlice = slice;
        self
    }

    #[inline]
    pub fn array_size(mut self, size: u32) -> Self {
        self.desc.Anonymous.Texture2DMSArray.ArraySize = size;
        self
    }
}

impl UnorderedAccessViewDesc<dimension::Texture3D> {
    #[inline]
    pub fn mip_slice(mut self, slice: u32) -> Self {
        self.desc.Anonymous.Texture3D.MipSlice = slice;
        self
    }

    #[inline]
    pub fn first_w_slice(mut self, slice: u32) -> Self {
        self.desc.Anonymous.Texture3D.FirstWSlice = slice;
        self
    }

    #[inline]
    pub fn w_size(mut self, size: u32) -> Self {
        self.desc.Anonymous.Texture3D.WSize = size;
        self
    }
}

#[repr(transparent)]
pub struct RenderTargetViewDesc<T = ()> {
    desc: D3D12_RENDER_TARGET_VIEW_DESC,
    _t: std::marker::PhantomData<T>,
}

impl RenderTargetViewDesc<()> {
    fn new<T>(dimension: D3D12_RTV_DIMENSION) -> RenderTargetViewDesc<T> {
        RenderTargetViewDesc {
            desc: D3D12_RENDER_TARGET_VIEW_DESC {
                ViewDimension: dimension,
                ..Default::default()
            },
            _t: std::marker::PhantomData,
        }
    }

    #[inline]
    pub fn buffer() -> RenderTargetViewDesc<dimension::Buffer> {
        Self::new(D3D12_RTV_DIMENSION_BUFFER)
    }

    #[inline]
    pub fn texture1d() -> RenderTargetViewDesc<dimension::Texture1D> {
        Self::new(D3D12_RTV_DIMENSION_TEXTURE1D)
    }

    #[inline]
    pub fn texture1d_array() -> RenderTargetViewDesc<dimension::Texture1DArray> {
        Self::new(D3D12_RTV_DIMENSION_TEXTURE1DARRAY)
    }

    #[inline]
    pub fn texture2d() -> RenderTargetViewDesc<dimension::Texture2D> {
        Self::new(D3D12_RTV_DIMENSION_TEXTURE2D)
    }

    #[inline]
    pub fn texture2d_array() -> RenderTargetViewDesc<dimension::Texture2DArray> {
        Self::new(D3D12_RTV_DIMENSION_TEXTURE2DARRAY)
    }

    #[inline]
    pub fn texture2d_ms() -> RenderTargetViewDesc<dimension::Texture2DMs> {
        Self::new(D3D12_RTV_DIMENSION_TEXTURE2DMS)
    }

    #[inline]
    pub fn texture2d_ms_array() -> RenderTargetViewDesc<dimension::Texture2DMsArray> {
        Self::new(D3D12_RTV_DIMENSION_TEXTURE2DMSARRAY)
    }

    #[inline]
    pub fn texture3d() -> RenderTargetViewDesc<dimension::Texture3D> {
        Self::new(D3D12_RTV_DIMENSION_TEXTURE3D)
    }

    #[inline]
    pub fn none() -> Option<&'static Self> {
        None
    }
}

impl RenderTargetViewDesc<dimension::Buffer> {
    #[inline]
    pub fn first_element(mut self, element: u64) -> Self {
        self.desc.Anonymous.Buffer.FirstElement = element;
        self
    }

    #[inline]
    pub fn num_elements(mut self, n: u32) -> Self {
        self.desc.Anonymous.Buffer.NumElements = n;
        self
    }
}

impl RenderTargetViewDesc<dimension::Texture1D> {
    #[inline]
    pub fn mip_slice(mut self, slice: u32) -> Self {
        self.desc.Anonymous.Texture1D.MipSlice = slice;
        self
    }
}

impl RenderTargetViewDesc<dimension::Texture1DArray> {
    #[inline]
    pub fn mip_slice(mut self, slice: u32) -> Self {
        self.desc.Anonymous.Texture1DArray.MipSlice = slice;
        self
    }

    #[inline]
    pub fn first_array_slice(mut self, slice: u32) -> Self {
        self.desc.Anonymous.Texture1DArray.FirstArraySlice = slice;
        self
    }

    #[inline]
    pub fn array_size(mut self, size: u32) -> Self {
        self.desc.Anonymous.Texture1DArray.ArraySize = size;
        self
    }
}

impl RenderTargetViewDesc<dimension::Texture2D> {
    #[inline]
    pub fn mip_slice(mut self, slice: u32) -> Self {
        self.desc.Anonymous.Texture2D.MipSlice = slice;
        self
    }

    #[inline]
    pub fn plane_slice(mut self, slice: u32) -> Self {
        self.desc.Anonymous.Texture2D.PlaneSlice = slice;
        self
    }
}

impl RenderTargetViewDesc<dimension::Texture2DArray> {
    #[inline]
    pub fn mip_slice(mut self, slice: u32) -> Self {
        self.desc.Anonymous.Texture2DArray.MipSlice = slice;
        self
    }

    #[inline]
    pub fn plane_slice(mut self, slice: u32) -> Self {
        self.desc.Anonymous.Texture2DArray.PlaneSlice = slice;
        self
    }

    #[inline]
    pub fn first_array_slice(mut self, slice: u32) -> Self {
        self.desc.Anonymous.Texture2DArray.FirstArraySlice = slice;
        self
    }

    #[inline]
    pub fn array_size(mut self, size: u32) -> Self {
        self.desc.Anonymous.Texture2DArray.ArraySize = size;
        self
    }
}

impl RenderTargetViewDesc<dimension::Texture2DMsArray> {
    #[inline]
    pub fn first_array_slice(mut self, slice: u32) -> Self {
        self.desc.Anonymous.Texture2DMSArray.FirstArraySlice = slice;
        self
    }

    #[inline]
    pub fn array_size(mut self, size: u32) -> Self {
        self.desc.Anonymous.Texture2DMSArray.ArraySize = size;
        self
    }
}

impl RenderTargetViewDesc<dimension::Texture3D> {
    #[inline]
    pub fn mip_slice(mut self, slice: u32) -> Self {
        self.desc.Anonymous.Texture3D.MipSlice = slice;
        self
    }

    #[inline]
    pub fn first_w_slice(mut self, slice: u32) -> Self {
        self.desc.Anonymous.Texture3D.FirstWSlice = slice;
        self
    }

    #[inline]
    pub fn w_size(mut self, size: u32) -> Self {
        self.desc.Anonymous.Texture3D.WSize = size;
        self
    }
}

#[repr(transparent)]
pub struct DepthStencilViewDesc<T = ()> {
    desc: D3D12_DEPTH_STENCIL_VIEW_DESC,
    _t: std::marker::PhantomData<T>,
}

impl DepthStencilViewDesc<()> {
    fn new<T>(dimension: D3D12_DSV_DIMENSION) -> DepthStencilViewDesc<T> {
        DepthStencilViewDesc {
            desc: D3D12_DEPTH_STENCIL_VIEW_DESC {
                ViewDimension: dimension,
                ..Default::default()
            },
            _t: std::marker::PhantomData,
        }
    }

    #[inline]
    pub fn texture1d() -> DepthStencilViewDesc<dimension::Texture1D> {
        Self::new(D3D12_DSV_DIMENSION_TEXTURE1D)
    }

    #[inline]
    pub fn texture1d_array() -> DepthStencilViewDesc<dimension::Texture1DArray> {
        Self::new(D3D12_DSV_DIMENSION_TEXTURE1DARRAY)
    }

    #[inline]
    pub fn texture2d() -> DepthStencilViewDesc<dimension::Texture2D> {
        Self::new(D3D12_DSV_DIMENSION_TEXTURE2D)
    }

    #[inline]
    pub fn texture2d_array() -> DepthStencilViewDesc<dimension::Texture2DArray> {
        Self::new(D3D12_DSV_DIMENSION_TEXTURE2DARRAY)
    }

    #[inline]
    pub fn texture2d_ms() -> DepthStencilViewDesc<dimension::Texture2DMs> {
        Self::new(D3D12_DSV_DIMENSION_TEXTURE2DMS)
    }

    #[inline]
    pub fn texture2d_ms_array() -> DepthStencilViewDesc<dimension::Texture2DMsArray> {
        Self::new(D3D12_DSV_DIMENSION_TEXTURE2DMSARRAY)
    }

    #[inline]
    pub fn none() -> Option<&'static Self> {
        None
    }
}

impl<T> DepthStencilViewDesc<T> {
    #[inline]
    pub fn flags(mut self, flags: D3D12_DSV_FLAGS) -> Self {
        self.desc.Flags = flags;
        self
    }
}

impl DepthStencilViewDesc<dimension::Texture1D> {
    #[inline]
    pub fn mip_slice(mut self, slice: u32) -> Self {
        self.desc.Anonymous.Texture1D.MipSlice = slice;
        self
    }
}

impl DepthStencilViewDesc<dimension::Texture1DArray> {
    #[inline]
    pub fn mip_slice(mut self, slice: u32) -> Self {
        self.desc.Anonymous.Texture1D.MipSlice = slice;
        self
    }

    #[inline]
    pub fn first_array_slice(mut self, slice: u32) -> Self {
        self.desc.Anonymous.Texture1DArray.FirstArraySlice = slice;
        self
    }

    #[inline]
    pub fn array_size(mut self, size: u32) -> Self {
        self.desc.Anonymous.Texture1DArray.ArraySize = size;
        self
    }
}

impl DepthStencilViewDesc<dimension::Texture2D> {
    #[inline]
    pub fn mip_slice(mut self, slice: u32) -> Self {
        self.desc.Anonymous.Texture2D.MipSlice = slice;
        self
    }
}

impl DepthStencilViewDesc<dimension::Texture2DArray> {
    #[inline]
    pub fn mip_slice(mut self, slice: u32) -> Self {
        self.desc.Anonymous.Texture2DArray.MipSlice = slice;
        self
    }

    #[inline]
    pub fn first_array_slice(mut self, slice: u32) -> Self {
        self.desc.Anonymous.Texture2DArray.FirstArraySlice = slice;
        self
    }

    #[inline]
    pub fn array_size(mut self, size: u32) -> Self {
        self.desc.Anonymous.Texture2DArray.ArraySize = size;
        self
    }
}

impl DepthStencilViewDesc<dimension::Texture2DMsArray> {
    #[inline]
    pub fn first_array_slice(mut self, slice: u32) -> Self {
        self.desc.Anonymous.Texture2DMSArray.FirstArraySlice = slice;
        self
    }

    #[inline]
    pub fn array_size(mut self, size: u32) -> Self {
        self.desc.Anonymous.Texture2DMSArray.ArraySize = size;
        self
    }
}

#[repr(transparent)]
pub struct SamplerDesc(D3D12_SAMPLER_DESC);

impl SamplerDesc {
    #[inline]
    pub fn new() -> Self {
        Self(D3D12_SAMPLER_DESC::default())
    }

    #[inline]
    pub fn filter(mut self, filter: D3D12_FILTER) -> Self {
        self.0.Filter = filter;
        self
    }

    #[inline]
    pub fn address_u(mut self, mode: D3D12_TEXTURE_ADDRESS_MODE) -> Self {
        self.0.AddressU = mode;
        self
    }

    #[inline]
    pub fn address_v(mut self, mode: D3D12_TEXTURE_ADDRESS_MODE) -> Self {
        self.0.AddressV = mode;
        self
    }

    #[inline]
    pub fn address_w(mut self, mode: D3D12_TEXTURE_ADDRESS_MODE) -> Self {
        self.0.AddressV = mode;
        self
    }

    #[inline]
    pub fn mip_lod_bias(mut self, bias: f32) -> Self {
        self.0.MipLODBias = bias;
        self
    }

    #[inline]
    pub fn max_anisotropy(mut self, anisotropy: u32) -> Self {
        self.0.MaxAnisotropy = anisotropy;
        self
    }

    #[inline]
    pub fn comparison_func(mut self, func: D3D12_COMPARISON_FUNC) -> Self {
        self.0.ComparisonFunc = func;
        self
    }

    #[inline]
    pub fn border_color(mut self, color: [f32; 4]) -> Self {
        self.0.BorderColor = color;
        self
    }

    #[inline]
    pub fn min_lod(mut self, lod: f32) -> Self {
        self.0.MinLOD = lod;
        self
    }

    #[inline]
    pub fn max_lod(mut self, lod: f32) -> Self {
        self.0.MaxLOD = lod;
        self
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
            Ok(DescriptorHeap {
                field: Arc::new(Field {
                    device: self.device,
                    handle,
                    len: self.desc.NumDescriptors as usize,
                    inc,
                    _t: self._t,
                }),
                name,
            })
        }
    }
}

#[derive(Debug)]
struct Field<T> {
    device: ID3D12Device,
    handle: ID3D12DescriptorHeap,
    len: usize,
    inc: usize,
    _t: std::marker::PhantomData<T>,
}

#[derive(Clone, Debug)]
pub struct DescriptorHeap<T = ()> {
    field: Arc<Field<T>>,
    name: Option<Name>,
}

impl DescriptorHeap<()> {
    #[inline]
    pub fn new_cbv_srv_uav(device: &Device) -> Builder<CbvSrvUav> {
        Builder::new(device.handle())
    }

    #[inline]
    pub fn new_rtv(device: &Device) -> Builder<Rtv> {
        Builder::new(device.handle())
    }

    #[inline]
    pub fn new_dsv(device: &Device) -> Builder<Dsv> {
        Builder::new(device.handle())
    }

    #[inline]
    pub fn new_sampler(device: &Device) -> Builder<Sampler> {
        Builder::new(device.handle())
    }
}

impl<T> DescriptorHeap<T>
where
    T: Type,
{
    #[inline]
    pub fn len(&self) -> usize {
        self.field.len
    }

    #[inline]
    pub fn cpu_handle(&self, index: usize) -> CpuDescriptorHandle<T> {
        assert!(index < self.field.len);
        unsafe {
            let mut dh = self.field.handle.GetCPUDescriptorHandleForHeapStart();
            dh.ptr += self.field.inc * index;
            CpuDescriptorHandle {
                heap: self.field.handle.clone(),
                handle: dh,
                _t: std::marker::PhantomData,
            }
        }
    }

    #[inline]
    pub fn gpu_handle(&self, index: usize) -> GpuDescriptorHandle<T> {
        assert!(index < self.field.len);
        unsafe {
            let mut dh = self.field.handle.GetGPUDescriptorHandleForHeapStart();
            dh.ptr += (self.field.inc * index) as u64;
            GpuDescriptorHandle {
                heap: self.field.handle.clone(),
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
            self.field.device.CopyDescriptorsSimple(
                len as u32,
                self.cpu_handle(start).handle,
                src.cpu_handle(dest_start).handle,
                T::VALUE,
            );
        }
    }

    #[inline]
    pub fn handle(&self) -> &ID3D12DescriptorHeap {
        &self.field.handle
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

impl<T> PartialEq for DescriptorHeap<T> {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.field.handle == other.field.handle
    }
}

impl<T> Eq for DescriptorHeap<T> {}

impl DescriptorHeap<CbvSrvUav> {
    #[inline]
    pub fn new(device: &Device) -> Builder<CbvSrvUav> {
        Builder::new(device.handle())
    }

    #[inline]
    pub fn create_constant_buffer_view(
        &mut self,
        index: usize,
        desc: Option<&ConstantBufferViewDesc>,
    ) {
        unsafe {
            self.field.device.CreateConstantBufferView(
                desc.map(|d| &d.0 as *const _),
                self.cpu_handle(index).handle(),
            );
        }
    }

    #[inline]
    pub fn create_shader_resource_view<U>(
        &mut self,
        index: usize,
        resource: &Resource,
        desc: Option<&ShaderResourceViewDesc<U>>,
    ) {
        unsafe {
            self.field.device.CreateShaderResourceView(
                resource.handle(),
                desc.map(|d| &d.desc as *const _),
                self.cpu_handle(index).handle(),
            );
        }
    }

    #[inline]
    pub fn create_unordered_access_view<U>(
        &mut self,
        index: usize,
        resource: &Resource,
        counter_resource: Option<&Resource>,
        desc: Option<&UnorderedAccessViewDesc<U>>,
    ) {
        unsafe {
            self.field.device.CreateUnorderedAccessView(
                resource.handle(),
                counter_resource.map(|r| r.handle()),
                desc.map(|d| &d.desc as *const _),
                self.cpu_handle(index).handle(),
            );
        }
    }
}

impl DescriptorHeap<Rtv> {
    #[inline]
    pub fn new(device: &Device) -> Builder<Rtv> {
        Builder::new(device.handle())
    }

    #[inline]
    pub fn create_render_target_view<U>(
        &mut self,
        index: usize,
        resource: &Resource,
        desc: Option<&RenderTargetViewDesc<U>>,
    ) {
        unsafe {
            self.field.device.CreateRenderTargetView(
                resource.handle(),
                desc.map(|d| &d.desc as *const _),
                self.cpu_handle(index).handle(),
            );
        }
    }
}

impl DescriptorHeap<Dsv> {
    #[inline]
    pub fn new(device: &Device) -> Builder<Dsv> {
        Builder::new(device.handle())
    }

    #[inline]
    pub fn create_depth_stencil_view<U>(
        &mut self,
        index: usize,
        resource: &Resource,
        desc: Option<&DepthStencilViewDesc<U>>,
    ) {
        unsafe {
            self.field.device.CreateDepthStencilView(
                resource.handle(),
                desc.map(|d| &d.desc as *const _),
                self.cpu_handle(index).handle(),
            );
        }
    }
}

impl DescriptorHeap<Sampler> {
    #[inline]
    pub fn new(device: &Device) -> Builder<Sampler> {
        Builder::new(device.handle())
    }

    #[inline]
    pub fn create_sampler(&self, index: usize, desc: &SamplerDesc) {
        unsafe {
            self.field
                .device
                .CreateSampler(&desc.0, self.cpu_handle(index).handle());
        }
    }
}

pub type RtvDescriptorHeap = DescriptorHeap<descriptor_heap_type::Rtv>;
pub type DsvDescriptorHeap = DescriptorHeap<descriptor_heap_type::Dsv>;
pub type CbvSrvUavDescriptorHeap = DescriptorHeap<descriptor_heap_type::CbvSrvUav>;
pub type SamplerDescriptorHeap = DescriptorHeap<descriptor_heap_type::Sampler>;

pub type RtvCpuDescriptorHandle = CpuDescriptorHandle<descriptor_heap_type::Rtv>;
pub type DsvCpuDescriptorHandle = CpuDescriptorHandle<descriptor_heap_type::Dsv>;
pub type CbvSrvUavCpuDescriptorHandle = CpuDescriptorHandle<descriptor_heap_type::CbvSrvUav>;
pub type SamplerCpuDescriptorHandle = CpuDescriptorHandle<descriptor_heap_type::Sampler>;

pub type RtvGpuDescriptorHandle = GpuDescriptorHandle<descriptor_heap_type::Rtv>;
pub type DsvGpuDescriptorHandle = GpuDescriptorHandle<descriptor_heap_type::Dsv>;
pub type CbvSrvUavGpuDescriptorHandle = GpuDescriptorHandle<descriptor_heap_type::CbvSrvUav>;
pub type SamplerGpuDescriptorHandle = GpuDescriptorHandle<descriptor_heap_type::Sampler>;
