use dxwr::{d3d::*, d3d12::*, dxgi::*};

const RAY_GEN_SHADER: &str = "ray_gen_shader";
const CLOSEST_HIT_SHADER: &str = "closest_hit_shader";
const MISS_SHADER: &str = "miss_shader";
const HIT_GROUP: &str = "hit_group";

#[repr(C)]
struct Viewport {
    left: f32,
    top: f32,
    right: f32,
    bottom: f32,
}

#[repr(C)]
struct RayGenConstantBuffer {
    viewport: Viewport,
    stencil: Viewport,
}

#[repr(C)]
struct RootArguments {
    cb: RayGenConstantBuffer,
}

#[repr(C)]
struct Vertex([f32; 3]);

fn build_geometry(device: &dxwr::Device) -> anyhow::Result<(dxwr::Resource, dxwr::Resource)> {
    let offset = 0.7;
    let depth = 1.0;
    let vertices = [
        Vertex([0.0, -offset, depth]),
        Vertex([-offset, offset, depth]),
        Vertex([offset, offset, depth]),
    ];
    let vertex_buffer = dxwr::Resource::new(&device)
        .heap_properties(&dxwr::HeapProperties::upload())
        .resource_desc(&dxwr::ResourceDesc::buffer().width(std::mem::size_of_val(&vertices) as u64))
        .init_state(D3D12_RESOURCE_STATE_GENERIC_READ)
        .name("vertex_buffer")
        .build()?;
    unsafe {
        let data = vertex_buffer.map(0)?;
        std::ptr::copy_nonoverlapping(
            vertices.as_ptr(),
            data.as_mut_ptr() as *mut Vertex,
            vertices.len(),
        );
    }
    let indices: [u32; 3] = [0, 1, 2];
    let index_buffer = dxwr::Resource::new(&device)
        .heap_properties(&dxwr::HeapProperties::upload())
        .resource_desc(&dxwr::ResourceDesc::buffer().width(std::mem::size_of_val(&indices) as u64))
        .init_state(D3D12_RESOURCE_STATE_GENERIC_READ)
        .name("index_buffer")
        .build()?;
    unsafe {
        let data = index_buffer.map(0)?;
        std::ptr::copy_nonoverlapping(
            indices.as_ptr(),
            data.as_mut_ptr() as *mut u32,
            indices.len(),
        );
    }
    Ok((vertex_buffer, index_buffer))
}

fn main() -> anyhow::Result<()> {
    dxwr::output_debug_string_to_stderr();
    dxwr::enable_debug_layer()?;

    let device = dxwr::Device::new()
        .min_feature_level(D3D_FEATURE_LEVEL_12_1)
        .build()?;
    let cmd_queue = dxwr::DirectCommandQueue::new(&device)
        .name("direct_cmd_queue")
        .build()?;
    let cmd_allocator = dxwr::DirectCommandAllocator::new(&device).build()?;
    let cmd_list = dxwr::DirectGraphicsCommandList::new(&device)
        .name("direct_cmd_list")
        .build()?;
    let fence = dxwr::Fence::new(&device).build()?;
    let global_root_signature = dxwr::RootSignature::new(&device)
        .name("global_root_signature")
        .build_from_desc(
            &dxwr::RootSignatureDesc::new().parameters(&[
                dxwr::RootParameter::new(dxwr::root_parameter_type::DescriptorTable)
                    .ranges(&[dxwr::DescriptorRange::uav().num_descriptors(1)])
                    .into(),
                dxwr::RootParameter::new(dxwr::root_parameter_type::Srv)
                    .register_space(0)
                    .into(),
            ]),
        )?;
    let local_root_signature = dxwr::RootSignature::new(&device)
        .name("local_root_signature")
        .build_from_desc(
            &dxwr::RootSignatureDesc::new()
                .parameters(&[
                    dxwr::RootParameter::new(dxwr::root_parameter_type::Constants32bit)
                        .num_32bit_values(8)
                        .into(),
                ])
                .flags(D3D12_ROOT_SIGNATURE_FLAG_LOCAL_ROOT_SIGNATURE),
        )?;
    let state_object = {
        let local_root_signature = dxwr::LocalRootSignature::new(&local_root_signature);
        dxwr::StateObject::new(&device, D3D12_STATE_OBJECT_TYPE_RAYTRACING_PIPELINE)
            .subobject(
                &dxwr::DxilLibraryDesc::new(dxwr::ShaderBytecode::new(include_bytes!(
                    "../examples/raytracing_triangle/raytracing.cso"
                )))
                .exports(&[
                    dxwr::ExportDesc::new().name(RAY_GEN_SHADER),
                    dxwr::ExportDesc::new().name(CLOSEST_HIT_SHADER),
                    dxwr::ExportDesc::new().name(MISS_SHADER),
                ]),
            )
            .subobject(
                &dxwr::HitGroupDesc::new()
                    .hit_group_type(D3D12_HIT_GROUP_TYPE_TRIANGLES)
                    .export(HIT_GROUP)
                    .closest_hit_shader_import(CLOSEST_HIT_SHADER),
            )
            .subobject(&dxwr::GlobalRootSignature::new(&global_root_signature))
            .subobject(&local_root_signature)
            .associate(
                &dxwr::SubobjectToExportsAssociation::new(&local_root_signature)
                    .exports(&[RAY_GEN_SHADER]),
            )
            .subobject(
                &dxwr::RaytracingShaderConfig::new()
                    .max_payload_size_in_bytes((std::mem::size_of::<f32>() * 4) as u32)
                    .max_attribute_size_in_bytes((std::mem::size_of::<f32>() * 2) as u32),
            )
            .subobject(&dxwr::RaytracingPipelineConfig::new().max_trace_recursion_depth(1))
            .name("state_object")
            .build()?
    };
    let state_object_props = dxwr::StateObjectProperties::new(&state_object);
    let (vertex_buffer, index_buffer) = build_geometry(&device)?;
    let geometry_desc: dxwr::RaytracingGeometryDesc = dxwr::RaytracingGeometryDesc::triangles()
        .vertex_buffer(
            dxwr::GpuVirtualAddressAndStride::new()
                .start_address(vertex_buffer.get_gpu_virtual_address())
                .stride_in_bytes(std::mem::size_of::<Vertex>() as u64),
            3,
            DXGI_FORMAT_R32G32B32_FLOAT,
        )
        .index_buffer(
            index_buffer.get_gpu_virtual_address(),
            3,
            DXGI_FORMAT_R32_UINT,
        )
        .flags(D3D12_RAYTRACING_GEOMETRY_FLAG_OPAQUE)
        .into();
    let top_level_inputs = dxwr::BuildRaytracingAccelerationStructureInputs::top_level()
        .num_descs(1)
        .flags(D3D12_RAYTRACING_ACCELERATION_STRUCTURE_BUILD_FLAG_PREFER_FAST_TRACE);
    let top_level_prebuild_info =
        device.get_raytracing_acceleration_structure_prebuild_info(&top_level_inputs);
    let bottom_level_inputs = dxwr::BuildRaytracingAccelerationStructureInputs::bottom_level()
        .geometry_descs(&[geometry_desc])
        .flags(D3D12_RAYTRACING_ACCELERATION_STRUCTURE_BUILD_FLAG_PREFER_FAST_TRACE);
    let bottom_level_prebuild_info =
        device.get_raytracing_acceleration_structure_prebuild_info(&bottom_level_inputs);
    let scratch = dxwr::Resource::new(&device)
        .heap_properties(&dxwr::HeapProperties::default())
        .resource_desc(
            &dxwr::ResourceDesc::buffer()
                .width(
                    top_level_prebuild_info
                        .scratch_data_size_in_bytes
                        .max(bottom_level_prebuild_info.scratch_data_size_in_bytes),
                )
                .flags(D3D12_RESOURCE_FLAG_ALLOW_UNORDERED_ACCESS),
        )
        .init_state(D3D12_RESOURCE_STATE_COMMON)
        .name("scratch")
        .build()?;
    let tlas = dxwr::Resource::new(&device)
        .heap_properties(&dxwr::HeapProperties::default())
        .resource_desc(
            &dxwr::ResourceDesc::buffer()
                .width(top_level_prebuild_info.result_data_size_in_bytes)
                .flags(D3D12_RESOURCE_FLAG_ALLOW_UNORDERED_ACCESS),
        )
        .init_state(D3D12_RESOURCE_STATE_RAYTRACING_ACCELERATION_STRUCTURE)
        .name("tlas")
        .build()?;
    let blas = dxwr::Resource::new(&device)
        .heap_properties(&dxwr::HeapProperties::default())
        .resource_desc(
            &dxwr::ResourceDesc::buffer()
                .width(bottom_level_prebuild_info.result_data_size_in_bytes)
                .flags(D3D12_RESOURCE_FLAG_ALLOW_UNORDERED_ACCESS),
        )
        .init_state(D3D12_RESOURCE_STATE_RAYTRACING_ACCELERATION_STRUCTURE)
        .name("blas")
        .build()?;
    let instance_descs = {
        let desc = dxwr::RaytracingInstanceDesc::new()
            .instance_mask(1)
            .accelration_structure(blas.get_gpu_virtual_address());
        let buffer = dxwr::Resource::new(&device)
            .heap_properties(&dxwr::HeapProperties::upload())
            .resource_desc(
                &dxwr::ResourceDesc::buffer()
                    .width(std::mem::size_of::<dxwr::RaytracingInstanceDesc>() as u64),
            )
            .init_state(D3D12_RESOURCE_STATE_GENERIC_READ)
            .name("instance_descs")
            .build()?;
        unsafe {
            let data = buffer.map(0)?;
            let p = data.as_mut_ptr() as *mut dxwr::RaytracingInstanceDesc;
            std::ptr::copy_nonoverlapping(&desc, p, 1);
        }
        buffer
    };
    cmd_list.record(&cmd_allocator, |cmd| {
        let top_level_inputs =
            top_level_inputs.instance_descs(instance_descs.get_gpu_virtual_address());
        let top_level = dxwr::BuildRaytracingAccelerationStructureDesc::new()
            .inputs(&top_level_inputs)
            .dest_acceleration_structure_data(tlas.get_gpu_virtual_address())
            .scratch_acceleration_structure_data(scratch.get_gpu_virtual_address());
        let bottom_level = dxwr::BuildRaytracingAccelerationStructureDesc::new()
            .inputs(&bottom_level_inputs)
            .dest_acceleration_structure_data(blas.get_gpu_virtual_address())
            .scratch_acceleration_structure_data(scratch.get_gpu_virtual_address());
        cmd.build_raytracing_acceleration_structure(&bottom_level);
        cmd.resource_barrier(&[dxwr::UavBarrier::new().resource(&blas)]);
        cmd.build_raytracing_acceleration_structure(&top_level);
    })?;
    cmd_queue.execute_command_lists(&[&cmd_list]);
    cmd_queue.signal(&fence)?.wait()?;
    let mut descriptor_heap = dxwr::CbvSrvUavDescriptorHeap::new(&device)
        .len(1)
        .flags(D3D12_DESCRIPTOR_HEAP_FLAG_SHADER_VISIBLE)
        .build()?;

    let mut event_rx = wiard::EventReceiver::new();
    let window = wiard::Window::builder(&mut event_rx)
        .title("dxwr raytracing")
        .build()?;
    let window_size = window.inner_size().unwrap();
    let swap_chain = dxwr::SwapChain::new()
        .command_queue(&cmd_queue)
        .width(window_size.width)
        .height(window_size.height)
        .buffer_count(2)
        .format(DXGI_FORMAT_R8G8B8A8_UNORM)
        .buffer_usage(DXGI_USAGE_RENDER_TARGET_OUTPUT)
        .swap_effect(DXGI_SWAP_EFFECT_FLIP_DISCARD)
        .build_for_hwnd(window.raw_handle())?;
    let mut rtv_heap = dxwr::RtvDescriptorHeap::new(&device).len(2).build()?;
    let render_targets = (0..2)
        .map(|i| -> anyhow::Result<dxwr::Resource> {
            let buffer = swap_chain.get_buffer(i)?;
            rtv_heap.create_render_target_view(i, &buffer, dxwr::RenderTargetViewDesc::none());
            Ok(buffer)
        })
        .collect::<anyhow::Result<Vec<_>>>()?;

    let (ray_gen_shader_table, ray_gen_shader_table_size) = {
        let aspect = window_size.width as f32 / window_size.height as f32;
        let border = 0.1;
        let stencil = if window_size.width <= window_size.height {
            Viewport {
                left: -1.0 + border,
                top: -1.0 + border * aspect,
                right: 1.0 - border,
                bottom: 1.0 - border * aspect,
            }
        } else {
            Viewport {
                left: -1.0 + border / aspect,
                top: -1.0 + border,
                right: 1.0 - border / aspect,
                bottom: 1.0 - border,
            }
        };
        let args = RootArguments {
            cb: RayGenConstantBuffer {
                viewport: Viewport {
                    left: -1.0,
                    top: -1.0,
                    right: 1.0,
                    bottom: 1.0,
                },
                stencil,
            },
        };
        let size =
            D3D12_SHADER_IDENTIFIER_SIZE_IN_BYTES + std::mem::size_of::<RootArguments>() as u32;
        let align = D3D12_RAYTRACING_SHADER_RECORD_BYTE_ALIGNMENT;
        let size = (size + (align - 1)) & !(align - 1);
        let buffer = dxwr::Resource::new(&device)
            .heap_properties(&dxwr::HeapProperties::upload())
            .resource_desc(&dxwr::ResourceDesc::buffer().width(size as u64))
            .init_state(D3D12_RESOURCE_STATE_GENERIC_READ)
            .name(RAY_GEN_SHADER)
            .build()?;
        let id = state_object_props.get_shader_identifier(RAY_GEN_SHADER) as *const u8;
        unsafe {
            let data = buffer.map(0)?;
            let p = data.as_mut_ptr() as *mut u8;
            std::ptr::copy_nonoverlapping(id, p, D3D12_SHADER_IDENTIFIER_SIZE_IN_BYTES as usize);
            std::ptr::copy_nonoverlapping(
                &args as *const RootArguments as *const u8,
                p.offset(D3D12_SHADER_IDENTIFIER_SIZE_IN_BYTES as isize),
                std::mem::size_of::<RootArguments>(),
            );
        }
        (buffer, size)
    };
    let (miss_shader_table, miss_shader_table_size) = {
        let size = D3D12_SHADER_IDENTIFIER_SIZE_IN_BYTES;
        let align = D3D12_RAYTRACING_SHADER_RECORD_BYTE_ALIGNMENT;
        let size = (size + (align - 1)) & !(align - 1);
        let buffer = dxwr::Resource::new(&device)
            .heap_properties(&dxwr::HeapProperties::upload())
            .resource_desc(&dxwr::ResourceDesc::buffer().width(size as u64))
            .init_state(D3D12_RESOURCE_STATE_GENERIC_READ)
            .name(MISS_SHADER)
            .build()?;
        let id = state_object_props.get_shader_identifier(MISS_SHADER) as *const u8;
        unsafe {
            let data = buffer.map(0)?;
            let p = data.as_mut_ptr() as *mut u8;
            std::ptr::copy_nonoverlapping(id, p, D3D12_SHADER_IDENTIFIER_SIZE_IN_BYTES as usize);
        }
        (buffer, size)
    };
    let (hit_group_table, hit_group_table_size) = {
        let size = D3D12_SHADER_IDENTIFIER_SIZE_IN_BYTES;
        let align = D3D12_RAYTRACING_SHADER_RECORD_BYTE_ALIGNMENT;
        let size = (size + (align - 1)) & !(align - 1);
        let buffer = dxwr::Resource::new(&device)
            .heap_properties(&dxwr::HeapProperties::upload())
            .resource_desc(&dxwr::ResourceDesc::buffer().width(size as u64))
            .init_state(D3D12_RESOURCE_STATE_GENERIC_READ)
            .name(HIT_GROUP)
            .build()?;
        let id = state_object_props.get_shader_identifier(HIT_GROUP) as *const u8;
        unsafe {
            let data = buffer.map(0)?;
            let p = data.as_mut_ptr() as *mut u8;
            std::ptr::copy_nonoverlapping(id, p, D3D12_SHADER_IDENTIFIER_SIZE_IN_BYTES as usize);
        }
        (buffer, size)
    };
    let dispatch_rays_desc = dxwr::DispatchRaysDesc::new()
        .ray_generation_shader_record(
            dxwr::GpuVirtualAddressRange::new()
                .start_address(ray_gen_shader_table.get_gpu_virtual_address())
                .size_in_bytes(ray_gen_shader_table_size as u64),
        )
        .miss_shader_table(
            dxwr::GpuVirtualAddressRangeAndStride::new()
                .start_address(miss_shader_table.get_gpu_virtual_address())
                .size_in_bytes(miss_shader_table_size as u64)
                .stride_in_bytes(miss_shader_table_size as u64),
        )
        .hit_group_table(
            dxwr::GpuVirtualAddressRangeAndStride::new()
                .start_address(hit_group_table.get_gpu_virtual_address())
                .size_in_bytes(hit_group_table_size as u64)
                .stride_in_bytes(hit_group_table_size as u64),
        )
        .width(window_size.width)
        .height(window_size.height)
        .depth(1);
    let raytracing_output = dxwr::Resource::new(&device)
        .heap_properties(&dxwr::HeapProperties::default())
        .resource_desc(
            &dxwr::ResourceDesc::texture2d()
                .width(window_size.width as u64)
                .height(window_size.height)
                .format(DXGI_FORMAT_R8G8B8A8_UNORM)
                .mip_levels(1)
                .flags(D3D12_RESOURCE_FLAG_ALLOW_UNORDERED_ACCESS),
        )
        .init_state(D3D12_RESOURCE_STATE_UNORDERED_ACCESS)
        .name("raytracing_output")
        .build()?;
    descriptor_heap.create_unordered_access_view(
        0,
        &raytracing_output,
        None,
        Some(&dxwr::UnorderedAccessViewDesc::texture2d()),
    );

    loop {
        match event_rx.try_recv() {
            Ok(_event) => continue,
            Err(wiard::TryRecvError::Empty) => {}
            Err(wiard::TryRecvError::Disconnected) => break,
        }
        let index = swap_chain.get_current_back_buffer_index();
        let rtv_handle = rtv_heap.cpu_handle(index);
        let rt = &render_targets[index];
        cmd_list.record(&cmd_allocator, |cmd| {
            cmd.set_compute_root_signature(&global_root_signature);
            cmd.set_descriptor_heaps(Some(&descriptor_heap), None);
            cmd.set_compute_root_descriptor_table(0, descriptor_heap.gpu_handle(0));
            cmd.set_compute_root_shader_resource_view(1, tlas.get_gpu_virtual_address());
            cmd.set_pipeline_state(&state_object);
            cmd.dispatch_rays(&dispatch_rays_desc);
            cmd.resource_barrier(&[dxwr::TransitionBarrier::new()
                .resource(&rt)
                .subresource(0)
                .state_before(D3D12_RESOURCE_STATE_PRESENT)
                .state_after(D3D12_RESOURCE_STATE_RENDER_TARGET)]);
            cmd.clear_render_target_view(rtv_handle, &[0.0, 0.0, 0.3, 0.0], None);
            cmd.resource_barrier(&[
                dxwr::TransitionBarrier::new()
                    .resource(rt)
                    .subresource(0)
                    .state_before(D3D12_RESOURCE_STATE_RENDER_TARGET)
                    .state_after(D3D12_RESOURCE_STATE_COPY_DEST),
                dxwr::TransitionBarrier::new()
                    .resource(&raytracing_output)
                    .subresource(0)
                    .state_before(D3D12_RESOURCE_STATE_UNORDERED_ACCESS)
                    .state_after(D3D12_RESOURCE_STATE_COPY_SOURCE),
            ]);
            cmd.copy_resource(&raytracing_output, &rt);
            cmd.resource_barrier(&[
                dxwr::TransitionBarrier::new()
                    .resource(rt)
                    .subresource(0)
                    .state_before(D3D12_RESOURCE_STATE_COPY_DEST)
                    .state_after(D3D12_RESOURCE_STATE_PRESENT),
                dxwr::TransitionBarrier::new()
                    .resource(&raytracing_output)
                    .subresource(0)
                    .state_before(D3D12_RESOURCE_STATE_COPY_SOURCE)
                    .state_after(D3D12_RESOURCE_STATE_UNORDERED_ACCESS),
            ]);
        })?;
        cmd_queue.execute_command_lists(&[&cmd_list]);
        let signal = swap_chain.present(&fence, 0, 0)?;
        signal.wait()?;
    }
    Ok(())
}
