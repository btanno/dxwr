mod adapter;
mod command_allocator;
mod command_list;
pub mod command_list_type;
mod command_queue;
mod debug;
mod descriptor_heap;
mod device;
pub mod dxc;
pub mod features;
mod fence;
mod pipeline_state;
pub mod raytracing;
pub mod reflection;
mod resource_barriers;
mod resources;
mod root_signature;
mod state_object;
mod swap_chain;
mod utility;

pub mod d3d {
    pub use windows::Win32::Graphics::Direct3D::*;
}

pub mod d3d12 {
    pub use windows::Win32::Graphics::Direct3D12::*;
}

pub mod dxgi {
    pub use windows::Win32::Graphics::Dxgi::{Common::*, *};
}

pub type Rect = windows::Win32::Foundation::RECT;

pub use adapter::{enum_adapters, enum_warp_adapter, Adapter, AdapterId, AdapterMemoryInfo};
pub use command_allocator::{
    BundleCommandAllocator, CommandAllocator, ComputeCommandAllocator, CopyCommandAllocator,
    DirectCommandAllocator, VideoDecodeCommandAllocator, VideoEncodeCommandAllocator,
    VideoProcessCommandAllocator,
};
pub use command_list::{
    BundleCommands, BundleGraphicsCommandList, Commands, ComputeCommands,
    ComputeGraphicsCommandList, CopyCommands, CopyGraphicsCommandList, DirectCommands,
    DirectGraphicsCommandList, DiscardRegion, DispatchRaysDesc, GraphicsCommandList,
    IndexBufferView, TextureCopyLocation, VertexBufferView, VideoDecodeCommands,
    VideoDecodeGraphicsCommandList, VideoEncodeCommands, VideoEncodeGraphicsCommandList,
    VideoProcessCommands, VideoProcessGraphicsCommandList,
};
pub use command_queue::{
    CommandQueue, ComputeCommandQueue, CopyCommandQueue, DirectCommandQueue,
    VideoDecodeCommandQueue, VideoEncodeCommandQueue, VideoProcessCommandQueue,
};
pub use debug::*;
pub use descriptor_heap::{
    descriptor_heap_type, CbvSrvUavCpuDescriptorHandle, CbvSrvUavDescriptorHeap,
    CbvSrvUavGpuDescriptorHandle, ConstantBufferViewDesc, CpuDescriptorHandle,
    DepthStencilViewDesc, DescriptorHeap, DsvCpuDescriptorHandle, DsvDescriptorHeap,
    DsvGpuDescriptorHandle, GpuDescriptorHandle, RenderTargetViewDesc, RtvCpuDescriptorHandle,
    RtvDescriptorHeap, RtvGpuDescriptorHandle, SamplerCpuDescriptorHandle, SamplerDesc,
    SamplerDescriptorHeap, SamplerGpuDescriptorHandle, ShaderResourceViewDesc,
    UnorderedAccessViewDesc,
};
pub use device::{Device, PlacedSubresourceFootprint, SubresourceFootprint};
pub use features::{Feature, RequestFeature};
pub use fence::{Fence, Signal};
pub use pipeline_state::*;
pub use raytracing::{
    BuildRaytracingAccelerationStructureDesc, BuildRaytracingAccelerationStructureInputs,
    BuildRaytracingAccelerationStructureInputsType, RaytracingAccelerationStructurePrebuildInfo,
    RaytracingGeometryDesc, RaytracingInstanceDesc,
};
pub use reflection::{LibraryReflection, ReflectionType, ShaderReflection};
pub use resource_barriers::{AliasingBarrier, TransitionBarrier, UavBarrier};
pub use resources::{ClearValue, Heap, HeapProperties, Resource, ResourceDesc};
pub use root_signature::{
    root_parameter_type, DescriptorRange, RootParameter, RootSignature, RootSignatureDesc,
    StaticSamplerDesc,
};
pub use state_object::{
    DxilLibraryDesc, DxilSubobjectToExportsAssociation, ExistingCollectionDesc, ExportDesc,
    GlobalRootSignature, HitGroupDesc, LocalRootSignature, NodeMask, RaytracingPipelineConfig,
    RaytracingShaderConfig, StateObject, StateObjectConfig, StateObjectProperties,
    SubobjectToExportsAssociation,
};
pub use swap_chain::{ResizeBuffers, SwapChain};
pub use utility::*;
