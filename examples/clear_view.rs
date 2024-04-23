use dxwr::d3d::*;
use dxwr::d3d12::*;
use dxwr::dxgi::*;

fn main() -> anyhow::Result<()> {
    const BUFFER_COUNT: usize = 2;

    dxwr::enable_debug_layer()?;
    let adapters = dxwr::enum_adapters()?;
    println!("adapter: {}", adapters[0].description());
    let device = dxwr::Device::new(Some(&adapters[0]), D3D_FEATURE_LEVEL_12_1, None)?;
    let mut event_rx = wiard::EventReceiver::new();
    let window = wiard::Window::builder(&event_rx)
        .title("dxwr clear_view")
        .inner_size(wiard::LogicalSize::new(1024, 768))
        .build()?;
    let size = window.inner_size().unwrap();
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
    let mut rtv = dxwr::DescriptorHeap::new_rtv(&device)
        .len(BUFFER_COUNT)
        .build()?;
    let render_targets = (0..BUFFER_COUNT)
        .map(|i| -> anyhow::Result<dxwr::Resource> {
            let buffer = swap_chain.get_buffer(i)?;
            rtv.create_render_target_view(i, &buffer, dxwr::RenderTargetViewDesc::none());
            Ok(buffer)
        })
        .collect::<anyhow::Result<Vec<_>>>()?;
    let fence = dxwr::Fence::new(&device).build()?;
    let cmd_allocator =
        dxwr::CommandAllocator::new(&device, dxwr::command_list_type::Direct).build()?;
    let cmd_list =
        dxwr::GraphicsCommandList::new(&device, dxwr::command_list_type::Direct).build()?;
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
                        left: 0,
                        top: 0,
                        right: size.width as i32,
                        bottom: size.height as i32,
                    }]);
                    cmd.clear_render_target_view(rtv_handle, &[0.0, 0.0, 0.3, 0.0], None);
                    cmd.resource_barrier(&[dxwr::TransitionBarrier::new(
                        &rt,
                        0,
                        D3D12_RESOURCE_STATE_RENDER_TARGET,
                        D3D12_RESOURCE_STATE_PRESENT,
                        D3D12_RESOURCE_BARRIER_FLAG_NONE,
                    )]);
                })?;
                cmd_queue.execute_command_lists(&[&cmd_list]);
                let signal = swap_chain.present(&fence, 0, 0)?;
                signal.wait()?;
            }
            _ => {}
        }
    }
    Ok(())
}
