use dxwr::{d3d::*, d3d12::*, dxgi::*};

#[derive(Clone, Copy)]
#[repr(C)]
struct Vertex {
    position: [f32; 3],
    color: [f32; 4],
}

impl Vertex {
    const fn new(position: [f32; 3], color: [f32; 4]) -> Self {
        Self { position, color }
    }
}

fn main() -> anyhow::Result<()> {
    const BUFFER_COUNT: usize = 2;

    dxwr::enable_debug_layer()?;
    let mut event_rx = wiard::EventReceiver::new();
    let window = wiard::Window::builder(&event_rx)
        .title("dxwr triangle")
        .build()?;
    let size = window.inner_size().unwrap();
    let device = dxwr::Device::new(None, D3D_FEATURE_LEVEL_12_1, None)?;
    let cmd_queue = dxwr::CommandQueue::new(&device, dxwr::command_list_type::Direct).build()?;
    let swap_chain = dxwr::SwapChain::new()
        .command_queue(&cmd_queue)
        .width(size.width)
        .height(size.height)
        .format(DXGI_FORMAT_R8G8B8A8_UNORM)
        .buffer_count(BUFFER_COUNT as u32)
        .buffer_usage(DXGI_USAGE_RENDER_TARGET_OUTPUT)
        .swap_effect(DXGI_SWAP_EFFECT_FLIP_DISCARD)
        .build_for_hwnd(window.raw_handle())?;
    let mut rtv = dxwr::DescriptorHeap::new(&device, dxwr::descriptor_heap_type::Rtv)
        .len(BUFFER_COUNT)
        .build()?;
    let render_targets = (0..BUFFER_COUNT)
        .map(|i| -> anyhow::Result<dxwr::Resource> {
            let buffer = swap_chain.get_buffer(i)?;
            rtv.create_view(i, &buffer, None);
            Ok(buffer)
        })
        .collect::<anyhow::Result<Vec<_>>>()?;
    let fence = dxwr::Fence::new(&device).build()?;
    let cmd_allocator =
        dxwr::CommandAllocator::new(&device, dxwr::command_list_type::Direct).build()?;
    let cmd_list =
        dxwr::GraphicsCommandList::new(&device, dxwr::command_list_type::Direct).build()?;
    let root_signature = dxwr::RootSignature::new(&device).build_from_desc(
        &dxwr::RootSignatureDesc::new()
            .flags(D3D12_ROOT_SIGNATURE_FLAG_ALLOW_INPUT_ASSEMBLER_INPUT_LAYOUT),
    )?;
    let pipeline = dxwr::PipelineState::new(&device)
        .desc(
            dxwr::GraphicsPipelineStateDesc::new()
                .root_signature(&root_signature)
                .vs(dxwr::ShaderBytecode::new(include_bytes!(
                    "../examples/triangle/triangle.vs"
                )))
                .ps(dxwr::ShaderBytecode::new(include_bytes!(
                    "../examples/triangle/triangle.ps"
                )))
                .rtv_formats(&[DXGI_FORMAT_R8G8B8A8_UNORM])
                .primitive_topology_type(D3D12_PRIMITIVE_TOPOLOGY_TYPE_TRIANGLE)
                .input_layout(&[
                    dxwr::InputElementDesc::new()
                        .semantic_name(b"POSITION\0")
                        .semantic_index(0)
                        .format(DXGI_FORMAT_R32G32B32_FLOAT)
                        .input_slot_class(D3D12_INPUT_CLASSIFICATION_PER_VERTEX_DATA),
                    dxwr::InputElementDesc::new()
                        .semantic_name(b"TEXCOORD\0")
                        .semantic_index(0)
                        .format(DXGI_FORMAT_R32G32B32A32_FLOAT)
                        .input_slot_class(D3D12_INPUT_CLASSIFICATION_PER_VERTEX_DATA),
                ])
                .depth_stencil_desc(dxwr::DepthStencilDesc::new().depth_enable(false)),
        )
        .build()?;
    let vertices = [
        Vertex::new([0.0, 0.8, 0.0], [1.0, 0.0, 0.0, 1.0]),
        Vertex::new([-0.8, -0.8, 0.0], [0.0, 1.0, 0.0, 1.0]),
        Vertex::new([0.8, -0.8, 0.0], [0.0, 0.0, 1.0, 1.0]),
    ];
    let indices = [0, 2, 1];
    let vertex_buffer = dxwr::Resource::new(&device)
        .heap_properties(&dxwr::HeapProperties::upload())
        .resource_desc(&dxwr::ResourceDesc::buffer().width(std::mem::size_of_val(&vertices) as u64))
        .init_state(D3D12_RESOURCE_STATE_GENERIC_READ)
        .build()?;
    let index_buffer = dxwr::Resource::new(&device)
        .heap_properties(&dxwr::HeapProperties::upload())
        .resource_desc(&dxwr::ResourceDesc::buffer().width(std::mem::size_of_val(&indices) as u64))
        .init_state(D3D12_RESOURCE_STATE_GENERIC_READ)
        .build()?;
    unsafe {
        let p = vertex_buffer.map(0)?;
        let data: &mut [Vertex; 3] = p.as_mut();
        data.copy_from_slice(&vertices);
    }
    unsafe {
        let p = index_buffer.map(0)?;
        let data: &mut [u32; 3] = p.as_mut();
        data.copy_from_slice(&indices)
    }
    loop {
        let Some((event, _)) = event_rx.recv() else {
            break;
        };
        match event {
            wiard::Event::Draw(_) => {
                let index = swap_chain.get_current_back_buffer_index();
                let rtv_handle = rtv.cpu_handle(index);
                let rt = &render_targets[index];
                cmd_list.record(&cmd_allocator, |cmd| {
                    cmd.set_pipeline_state(&pipeline);
                    cmd.set_graphics_root_signature(&root_signature);
                    cmd.resource_barrier(&[dxwr::TransitionBarrier::new(
                        &rt,
                        0,
                        D3D12_RESOURCE_STATE_PRESENT,
                        D3D12_RESOURCE_STATE_RENDER_TARGET,
                        D3D12_RESOURCE_BARRIER_FLAG_NONE,
                    )]);
                    cmd.rs_set_viewports(&[D3D12_VIEWPORT {
                        Width: size.width as f32,
                        Height: size.height as f32,
                        MaxDepth: 1.0,
                        ..Default::default()
                    }]);
                    cmd.rs_set_scissor_rects(&[dxwr::Rect {
                        right: size.width as i32,
                        bottom: size.height as i32,
                        ..Default::default()
                    }]);
                    cmd.clear_render_target_view(rtv_handle.clone(), &[0.0, 0.0, 0.3, 0.0], None);
                    cmd.om_set_render_targets(Some(&[rtv_handle.clone()]), true, None);
                    cmd.ia_set_vertex_buffers(
                        0,
                        Some(&[dxwr::VertexBufferView::new()
                            .buffer_location(vertex_buffer.get_gpu_virtual_address())
                            .size_in_bytes(std::mem::size_of_val(&vertices) as u32)
                            .stride_in_bytes(std::mem::size_of::<Vertex>() as u32)]),
                    );
                    cmd.ia_set_index_buffer(Some(
                        &dxwr::IndexBufferView::new()
                            .buffer_location(index_buffer.get_gpu_virtual_address())
                            .size_in_bytes(std::mem::size_of_val(&indices) as u32)
                            .format(DXGI_FORMAT_R32_UINT),
                    ));
                    cmd.draw_indexed_instanced(3, 1, 0, 0, 0);
                    cmd.resource_barrier(&[dxwr::TransitionBarrier::new(
                        &rt,
                        0,
                        D3D12_RESOURCE_STATE_RENDER_TARGET,
                        D3D12_RESOURCE_STATE_PRESENT,
                        D3D12_RESOURCE_BARRIER_FLAG_NONE,
                    )]);
                })?;
                cmd_queue.execute_command_lists(&[&cmd_list]);
                swap_chain.present(&fence, 0, 0)?.wait()?;
            }
            _ => {}
        }
    }
    Ok(())
}
