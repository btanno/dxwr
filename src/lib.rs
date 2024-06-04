mod adapter;
mod command_allocator;
mod command_list;
pub mod command_list_type;
mod command_queue;
mod debug;
mod descriptor_heap;
mod device;
pub mod dxc;
mod fence;
mod pipeline_state;
mod raytracing;
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

pub use adapter::{enum_adapters, Adapter, AdapterId, AdapterMemoryInfo};
pub use command_allocator::CommandAllocator;
pub use command_list::{
    DiscardRegion, DispatchRaysDesc, GraphicsCommandList, IndexBufferView, VertexBufferView,
};
pub use command_queue::CommandQueue;
pub use debug::*;
pub use descriptor_heap::{
    descriptor_heap_type, ConstantBufferViewDesc, CpuDescriptorHandle, DepthStencilViewDesc,
    DescriptorHeap, GpuDescriptorHandle, RenderTargetViewDesc, SamplerDesc, ShaderResourceViewDesc,
    UnorderedAccessViewDesc,
};
pub use device::Device;
pub use fence::{Fence, Signal};
pub use pipeline_state::*;
pub use raytracing::{
    BuildRaytracingAccelerationStructureDesc, BuildRaytracingAccelerationStructureInputs,
    RaytracingAccelerationStructurePrebuildInfo, RaytracingGeometryDesc, RaytracingInstanceDesc,
};
pub use reflection::{LibraryReflection, ReflectionType, ShaderReflection};
pub use resource_barriers::{AliasingBarrier, TransitionBarrier, UavBarrier};
pub use resources::{HeapProperties, Resource, ResourceDesc};
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
