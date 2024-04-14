mod adapter;
mod command_allocator;
mod command_list;
pub mod command_list_type;
mod command_queue;
mod descriptor_heap;
mod device;
mod fence;
mod pipeline_state;
mod resource_barriers;
mod resources;
mod root_signature;
mod swap_chain;
mod utility;
mod debug;

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
pub use command_list::{GraphicsCommandList, VertexBufferView, IndexBufferView};
pub use command_queue::CommandQueue;
pub use descriptor_heap::{
    descriptor_heap_type, CpuDescriptorHandle, DescriptorHeap, GpuDescriptorHandle,
};
pub use device::Device;
pub use fence::{Fence, Signal};
pub use pipeline_state::*;
pub use resource_barriers::{AliasingBarrier, TransitionBarrier, UavBarrier};
pub use resources::{HeapProperties, Resource, ResourceDesc};
pub use root_signature::{
    root_parameter_type, DescriptorRange, RootParameter, RootSignature, RootSignatureDesc,
    StaticSamplerDesc,
};
pub use swap_chain::SwapChain;
pub use debug::*;

use utility::*;
