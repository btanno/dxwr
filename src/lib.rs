mod adapter;
mod command_allocator;
mod command_list;
pub mod command_list_type;
mod command_queue;
mod debug;
pub mod descriptor_heap;
mod device;
pub mod dxc;
pub mod features;
mod fence;
mod pipeline_state;
pub mod prelude;
pub mod raytracing;
pub mod reflection;
mod resource_barriers;
pub mod resources;
mod root_signature;
mod state_object;
mod swap_chain;
mod utility;

pub mod com {
    pub use ::windows::core::Interface;
}

pub mod api {
    pub use windows::*;
}

pub mod d3d {
    pub use windows::Win32::Graphics::Direct3D::*;
}

pub mod d3d12 {
    pub use windows::Win32::Graphics::Direct3D12::*;
}

pub mod dxgi {
    pub use windows::Win32::Graphics::Dxgi::{Common::*, *};
}

pub use adapter::{Adapter, AdapterId, AdapterMemoryInfo, enum_adapters, enum_warp_adapter};
pub use command_allocator::{
    BundleCommandAllocator, CommandAllocator, ComputeCommandAllocator, CopyCommandAllocator,
    DirectCommandAllocator, VideoDecodeCommandAllocator, VideoEncodeCommandAllocator,
    VideoProcessCommandAllocator,
};
pub use command_list::{
    BundleCommands, BundleGraphicsCommandList, CommandList, Commands, ComputeCommands,
    ComputeGraphicsCommandList, CopyCommands, CopyGraphicsCommandList, DirectCommands,
    DirectGraphicsCommandList, DiscardRegion, DispatchRaysDesc, GraphicsCommandList,
    IndexBufferView, TextureCopyLocation, VertexBufferView, Viewport,
};
pub use command_queue::{
    CommandQueue, ComputeCommandQueue, CopyCommandQueue, DirectCommandQueue,
    VideoDecodeCommandQueue, VideoEncodeCommandQueue, VideoProcessCommandQueue,
};
pub use debug::*;
pub use descriptor_heap::{
    CbvSrvUavCpuDescriptorHandle, CbvSrvUavDescriptorHeap, CbvSrvUavGpuDescriptorHandle,
    ConstantBufferViewDesc, CpuDescriptorHandle, DepthStencilViewDesc, DescriptorHeap,
    DsvCpuDescriptorHandle, DsvDescriptorHeap, DsvGpuDescriptorHandle, GpuDescriptorHandle,
    RenderTargetViewDesc, RtvCpuDescriptorHandle, RtvDescriptorHeap, RtvGpuDescriptorHandle,
    SamplerCpuDescriptorHandle, SamplerDesc, SamplerDescriptorHeap, SamplerGpuDescriptorHandle,
    ShaderResourceViewDesc, UnorderedAccessViewDesc, descriptor_heap_type,
};
pub use device::{Device, PlacedSubresourceFootprint, SubresourceFootprint};
pub use dxc::{Blob, BlobType, RefBlob};
pub use features::{Feature, RequestFeature};
pub use fence::{Fence, Signal};
pub use pipeline_state::*;
pub use raytracing::{
    BuildRaytracingAccelerationStructureDesc, BuildRaytracingAccelerationStructureInputs,
    BuildRaytracingAccelerationStructureInputsType, RaytracingAccelerationStructurePrebuildInfo,
    RaytracingGeometryDesc, RaytracingInstanceDesc,
};
pub use reflection::{LibraryReflection, ReflectionType, ShaderReflection};
pub use resource_barriers::{AliasingBarrier, ResourceBarrier, TransitionBarrier, UavBarrier};
pub use resources::{ClearValue, Heap, HeapProperties, Resource, ResourceDesc};
pub use root_signature::{
    DescriptorRange, RootParameter, RootSignature, RootSignatureDesc, StaticSamplerDesc,
    root_parameter_type,
};
pub use state_object::{
    DxilLibraryDesc, DxilSubobjectToExportsAssociation, ExistingCollectionDesc, ExportDesc,
    GlobalRootSignature, HitGroupDesc, LocalRootSignature, NodeMask, RaytracingPipelineConfig,
    RaytracingShaderConfig, StateObject, StateObjectConfig, StateObjectProperties,
    SubobjectToExportsAssociation,
};
pub use swap_chain::{ResizeBuffers, SwapChain};
pub use utility::*;

use device::DeviceType;
