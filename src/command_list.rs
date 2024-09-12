use super::command_list_type::*;
use super::descriptor_heap_type::*;
use super::resource_barriers::*;
use super::*;
use windows::Win32::Graphics::{Direct3D::*, Direct3D12::*, Dxgi::Common::DXGI_FORMAT};

#[derive(Clone, Debug)]
#[repr(transparent)]
pub struct VertexBufferView {
    view: D3D12_VERTEX_BUFFER_VIEW,
}

impl VertexBufferView {
    #[inline]
    pub fn new() -> Self {
        Self {
            view: D3D12_VERTEX_BUFFER_VIEW::default(),
        }
    }

    #[inline]
    pub fn buffer_location(mut self, loc: GpuVirtualAddress) -> Self {
        self.view.BufferLocation = loc.0;
        self
    }

    #[inline]
    pub fn size_in_bytes(mut self, size: u32) -> Self {
        self.view.SizeInBytes = size;
        self
    }

    #[inline]
    pub fn stride_in_bytes(mut self, stride: u32) -> Self {
        self.view.StrideInBytes = stride;
        self
    }
}

#[derive(Clone, Debug)]
#[repr(transparent)]
pub struct IndexBufferView {
    view: D3D12_INDEX_BUFFER_VIEW,
}

impl IndexBufferView {
    #[inline]
    pub fn new() -> Self {
        Self {
            view: D3D12_INDEX_BUFFER_VIEW::default(),
        }
    }

    #[inline]
    pub fn buffer_location(mut self, loc: GpuVirtualAddress) -> Self {
        self.view.BufferLocation = loc.0;
        self
    }

    #[inline]
    pub fn size_in_bytes(mut self, size: u32) -> Self {
        self.view.SizeInBytes = size;
        self
    }

    #[inline]
    pub fn format(mut self, format: DXGI_FORMAT) -> Self {
        self.view.Format = format;
        self
    }
}

pub struct DiscardRegion<'a> {
    region: D3D12_DISCARD_REGION,
    _a: std::marker::PhantomData<&'a ()>,
}

impl<'a> DiscardRegion<'a> {
    #[inline]
    pub fn new() -> Self {
        Self {
            region: D3D12_DISCARD_REGION::default(),
            _a: std::marker::PhantomData,
        }
    }

    #[inline]
    pub fn rects<'b>(self, rects: &'b [Rect]) -> DiscardRegion<'b> {
        DiscardRegion {
            region: D3D12_DISCARD_REGION {
                NumRects: rects.len() as u32,
                pRects: rects.as_ptr(),
                ..self.region
            },
            _a: std::marker::PhantomData,
        }
    }

    #[inline]
    pub fn first_subresource(mut self, subresource: u32) -> Self {
        self.region.FirstSubresource = subresource;
        self
    }

    #[inline]
    pub fn num_subresource(mut self, n: u32) -> Self {
        self.region.NumSubresources = n;
        self
    }
}

#[derive(Clone, Debug)]
#[repr(transparent)]
pub struct DispatchRaysDesc(D3D12_DISPATCH_RAYS_DESC);

impl DispatchRaysDesc {
    #[inline]
    pub fn new() -> Self {
        Self(D3D12_DISPATCH_RAYS_DESC::default())
    }

    #[inline]
    pub fn ray_generation_shader_record(mut self, range: GpuVirtualAddressRange) -> Self {
        self.0.RayGenerationShaderRecord = range.0;
        self
    }

    #[inline]
    pub fn miss_shader_table(mut self, value: GpuVirtualAddressRangeAndStride) -> Self {
        self.0.MissShaderTable = value.0;
        self
    }

    #[inline]
    pub fn hit_group_table(mut self, value: GpuVirtualAddressRangeAndStride) -> Self {
        self.0.HitGroupTable = value.0;
        self
    }

    #[inline]
    pub fn callable_shader_table(mut self, value: GpuVirtualAddressRangeAndStride) -> Self {
        self.0.CallableShaderTable = value.0;
        self
    }

    #[inline]
    pub fn width(mut self, width: u32) -> Self {
        self.0.Width = width;
        self
    }

    #[inline]
    pub fn height(mut self, height: u32) -> Self {
        self.0.Height = height;
        self
    }

    #[inline]
    pub fn depth(mut self, depth: u32) -> Self {
        self.0.Depth = depth;
        self
    }
}

#[derive(Clone, Debug)]
#[repr(transparent)]
pub struct StreamOutputBufferView(D3D12_STREAM_OUTPUT_BUFFER_VIEW);

impl StreamOutputBufferView {
    #[inline]
    pub fn new() -> Self {
        Self(D3D12_STREAM_OUTPUT_BUFFER_VIEW::default())
    }

    #[inline]
    pub fn buffer_location(mut self, loc: GpuVirtualAddress) -> Self {
        self.0.BufferLocation = loc.0;
        self
    }

    #[inline]
    pub fn size_in_bytes(mut self, size: u64) -> Self {
        self.0.SizeInBytes = size;
        self
    }

    #[inline]
    pub fn buffer_filled_size_location(mut self, v: u64) -> Self {
        self.0.BufferFilledSizeLocation = v;
        self
    }
}

pub trait ClearUnorderedAccessView: Sized {
    fn call(
        cmd_list: &ID3D12GraphicsCommandList7,
        view_gpu_handle: &GpuDescriptorHandle<descriptor_heap_type::CbvSrvUav>,
        view_cpu_handle: &CpuDescriptorHandle<descriptor_heap_type::CbvSrvUav>,
        resource: &Resource,
        values: &[Self; 4],
        rects: &[Rect],
    );
}

impl ClearUnorderedAccessView for f32 {
    fn call(
        cmd_list: &ID3D12GraphicsCommandList7,
        view_gpu_handle: &GpuDescriptorHandle<descriptor_heap_type::CbvSrvUav>,
        view_cpu_handle: &CpuDescriptorHandle<descriptor_heap_type::CbvSrvUav>,
        resource: &Resource,
        values: &[Self; 4],
        rects: &[Rect],
    ) {
        unsafe {
            cmd_list.ClearUnorderedAccessViewFloat(
                view_gpu_handle.handle(),
                view_cpu_handle.handle(),
                resource.handle(),
                values,
                rects,
            );
        }
    }
}

impl ClearUnorderedAccessView for u32 {
    fn call(
        cmd_list: &ID3D12GraphicsCommandList7,
        view_gpu_handle: &GpuDescriptorHandle<descriptor_heap_type::CbvSrvUav>,
        view_cpu_handle: &CpuDescriptorHandle<descriptor_heap_type::CbvSrvUav>,
        resource: &Resource,
        values: &[Self; 4],
        rects: &[Rect],
    ) {
        unsafe {
            cmd_list.ClearUnorderedAccessViewUint(
                view_gpu_handle.handle(),
                view_cpu_handle.handle(),
                resource.handle(),
                values,
                rects,
            );
        }
    }
}

pub trait SetComputeRoot32BitConstants {
    fn call(
        self,
        cmd_list: &ID3D12GraphicsCommandList7,
        root_parameter_index: u32,
        dest_offset_in_32bit_values: u32,
    );
}

impl SetComputeRoot32BitConstants for u32 {
    fn call(
        self,
        cmd_list: &ID3D12GraphicsCommandList7,
        root_parameter_index: u32,
        dest_offset_in_32bit_values: u32,
    ) {
        unsafe {
            cmd_list.SetComputeRoot32BitConstant(
                root_parameter_index,
                self,
                dest_offset_in_32bit_values,
            );
        }
    }
}

impl SetComputeRoot32BitConstants for &[u32] {
    fn call(
        self,
        cmd_list: &ID3D12GraphicsCommandList7,
        root_parameter_index: u32,
        dest_offset_in_32bit_values: u32,
    ) {
        unsafe {
            cmd_list.SetComputeRoot32BitConstants(
                root_parameter_index,
                self.len() as u32,
                self.as_ptr() as *const std::ffi::c_void,
                dest_offset_in_32bit_values,
            );
        }
    }
}

pub trait SetGraphicsRoot32BitConstants {
    fn call(
        self,
        cmd_list: &ID3D12GraphicsCommandList7,
        root_parameter_index: u32,
        dest_offset_in_32bit_values: u32,
    );
}

impl SetGraphicsRoot32BitConstants for u32 {
    fn call(
        self,
        cmd_list: &ID3D12GraphicsCommandList7,
        root_parameter_index: u32,
        dest_offset_in_32bit_values: u32,
    ) {
        unsafe {
            cmd_list.SetGraphicsRoot32BitConstant(
                root_parameter_index,
                self,
                dest_offset_in_32bit_values,
            );
        }
    }
}

impl SetGraphicsRoot32BitConstants for &[u32] {
    fn call(
        self,
        cmd_list: &ID3D12GraphicsCommandList7,
        root_parameter_index: u32,
        dest_offset_in_32bit_values: u32,
    ) {
        unsafe {
            cmd_list.SetGraphicsRoot32BitConstants(
                root_parameter_index,
                self.len() as u32,
                self.as_ptr() as *const std::ffi::c_void,
                dest_offset_in_32bit_values,
            );
        }
    }
}

pub trait PipelineStateType {
    fn call(&self, cmd_list: &ID3D12GraphicsCommandList7);
}

#[derive(Clone)]
pub struct TextureCopyLocation(D3D12_TEXTURE_COPY_LOCATION);

impl TextureCopyLocation {
    #[inline]
    pub fn placed_footprint(
        resource: &Resource,
        placed_footprint: PlacedSubresourceFootprint,
    ) -> Self {
        Self(D3D12_TEXTURE_COPY_LOCATION {
            pResource: std::mem::ManuallyDrop::new(Some(resource.handle().clone())),
            Type: D3D12_TEXTURE_COPY_TYPE_PLACED_FOOTPRINT,
            Anonymous: D3D12_TEXTURE_COPY_LOCATION_0 {
                PlacedFootprint: placed_footprint.into(),
            },
        })
    }

    #[inline]
    pub fn subresource_index(resource: &Resource, index: u32) -> Self {
        Self(D3D12_TEXTURE_COPY_LOCATION {
            pResource: std::mem::ManuallyDrop::new(Some(resource.handle().clone())),
            Type: D3D12_TEXTURE_COPY_TYPE_SUBRESOURCE_INDEX,
            Anonymous: D3D12_TEXTURE_COPY_LOCATION_0 {
                SubresourceIndex: index,
            },
        })
    }
}

impl Drop for TextureCopyLocation {
    fn drop(&mut self) {
        unsafe {
            std::mem::ManuallyDrop::drop(&mut self.0.pResource);
        }
    }
}

pub struct Commands<'a, T> {
    cmd_list: &'a ID3D12GraphicsCommandList7,
    _t: std::marker::PhantomData<T>,
}

impl<'a, T> Commands<'a, T> {
    #[inline]
    pub fn clear_depth_stencil_view(
        &self,
        dsv: &CpuDescriptorHandle<Dsv>,
        depth: Option<f32>,
        stencil: Option<u8>,
        rects: Option<&[Rect]>,
    ) {
        let flags = depth.map_or(0, |_| D3D12_CLEAR_FLAG_DEPTH.0)
            | stencil.map_or(0, |_| D3D12_CLEAR_FLAG_STENCIL.0);
        unsafe {
            self.cmd_list.ClearDepthStencilView(
                dsv.handle(),
                D3D12_CLEAR_FLAGS(flags),
                depth.unwrap_or(0.0),
                stencil.unwrap_or(0),
                rects.unwrap_or(&[]),
            );
        }
    }

    #[inline]
    pub fn clear_render_target_view(
        &self,
        rtv: &CpuDescriptorHandle<Rtv>,
        color: &[f32; 4],
        rects: Option<&[Rect]>,
    ) {
        unsafe {
            self.cmd_list
                .ClearRenderTargetView(rtv.handle(), color, rects);
        }
    }

    #[inline]
    pub fn clear_unordered_access_view<U>(
        &self,
        view_gpu_handle_in_current_heap: &GpuDescriptorHandle<descriptor_heap_type::CbvSrvUav>,
        view_cpu_handle: &CpuDescriptorHandle<descriptor_heap_type::CbvSrvUav>,
        resource: &Resource,
        values: &[U; 4],
        rects: &[Rect],
    ) where
        U: ClearUnorderedAccessView,
    {
        U::call(
            self.cmd_list,
            view_gpu_handle_in_current_heap,
            view_cpu_handle,
            resource,
            values,
            rects,
        );
    }

    #[inline]
    pub fn copy_buffer_region(
        &self,
        src: &Resource,
        src_offset: u64,
        dest: &Resource,
        dest_offset: u64,
        num_bytes: u64,
    ) {
        unsafe {
            self.cmd_list.CopyBufferRegion(
                dest.handle(),
                dest_offset,
                src.handle(),
                src_offset,
                num_bytes,
            );
        }
    }

    #[inline]
    pub fn copy_resource(&self, src: &Resource, dest: &Resource) {
        unsafe {
            self.cmd_list.CopyResource(dest.handle(), src.handle());
        }
    }

    #[inline]
    pub fn copy_texture_region(
        &self,
        src: &TextureCopyLocation,
        src_box: Option<&D3D12_BOX>,
        dest: &TextureCopyLocation,
        dest_x: u32,
        dest_y: u32,
        dest_z: u32,
    ) {
        unsafe {
            self.cmd_list.CopyTextureRegion(
                &dest.0,
                dest_x,
                dest_y,
                dest_z,
                &src.0,
                src_box.map(|s| s as *const _),
            );
        }
    }

    #[inline]
    pub fn dispatch(
        &self,
        thread_group_count_x: u32,
        thread_group_count_y: u32,
        thread_group_count_z: u32,
    ) {
        unsafe {
            self.cmd_list.Dispatch(
                thread_group_count_x,
                thread_group_count_y,
                thread_group_count_z,
            );
        }
    }

    #[inline]
    pub fn dispatch_mesh(
        &self,
        thread_group_count_x: u32,
        thread_group_count_y: u32,
        thread_group_count_z: u32,
    ) {
        unsafe {
            self.cmd_list.DispatchMesh(
                thread_group_count_x,
                thread_group_count_y,
                thread_group_count_z,
            );
        }
    }

    #[inline]
    pub fn dispatch_rays(&self, desc: &DispatchRaysDesc) {
        unsafe {
            self.cmd_list.DispatchRays(&desc.0);
        }
    }

    #[inline]
    pub fn draw_indexed_instanced(
        &self,
        index_count_per_instance: u32,
        instance_count: u32,
        start_index_location: u32,
        base_vertex_location: i32,
        start_instance_location: u32,
    ) {
        unsafe {
            self.cmd_list.DrawIndexedInstanced(
                index_count_per_instance,
                instance_count,
                start_index_location,
                base_vertex_location,
                start_instance_location,
            );
        }
    }

    #[inline]
    pub fn draw_instanced(
        &self,
        vertex_count_per_instance: u32,
        instance_count: u32,
        start_verte_location: u32,
        start_instance_location: u32,
    ) {
        unsafe {
            self.cmd_list.DrawInstanced(
                vertex_count_per_instance,
                instance_count,
                start_verte_location,
                start_instance_location,
            );
        }
    }

    #[inline]
    pub fn execute_bundle(&self, cmd_list: &GraphicsCommandList<Bundle>) {
        unsafe {
            self.cmd_list.ExecuteBundle(&cmd_list.handle);
        }
    }

    #[inline]
    pub fn ia_set_index_buffer(&self, view: Option<&IndexBufferView>) {
        unsafe {
            self.cmd_list
                .IASetIndexBuffer(view.map(|v| v as *const _ as *const D3D12_INDEX_BUFFER_VIEW));
        }
    }

    #[inline]
    pub fn ia_set_primitive_topology(&self, topology: D3D_PRIMITIVE_TOPOLOGY) {
        unsafe {
            self.cmd_list.IASetPrimitiveTopology(topology);
        }
    }

    #[inline]
    pub fn ia_set_vertex_buffers(&self, start_slot: u32, views: Option<&[VertexBufferView]>) {
        unsafe {
            let views = views.map(|views| {
                std::slice::from_raw_parts(
                    views.as_ptr() as *const D3D12_VERTEX_BUFFER_VIEW,
                    views.len(),
                )
            });
            self.cmd_list.IASetVertexBuffers(start_slot, views);
        }
    }

    #[inline]
    pub fn om_set_blend_factor(&self, factor: &[f32; 4]) {
        unsafe {
            self.cmd_list.OMSetBlendFactor(Some(factor));
        }
    }

    #[inline]
    pub fn om_set_render_targets(
        &self,
        rtvs: Option<&[&CpuDescriptorHandle<Rtv>]>,
        rts_single_handle_to_descriptor_range: bool,
        depth_stencil: Option<&CpuDescriptorHandle<Dsv>>,
    ) {
        let rtvs = rtvs.map(|rtvs| rtvs.iter().map(|rtv| rtv.handle()).collect::<Vec<_>>());
        let depth_stencil = depth_stencil.map(|ds| ds.handle());
        unsafe {
            self.cmd_list.OMSetRenderTargets(
                rtvs.as_ref().map_or(0, |r| r.len() as u32),
                rtvs.as_ref().map(|r| r.as_ptr()),
                rts_single_handle_to_descriptor_range,
                depth_stencil.as_ref().map(|ds| ds as *const _),
            );
        }
    }

    #[inline]
    pub fn om_set_stencil_ref(&self, stencil_ref: u32) {
        unsafe {
            self.cmd_list.OMSetStencilRef(stencil_ref);
        }
    }

    #[inline]
    pub fn resolve_subresource(
        &self,
        src: &Resource,
        src_subresource: u32,
        dest: &Resource,
        dest_resource: u32,
        format: DXGI_FORMAT,
    ) {
        unsafe {
            self.cmd_list.ResolveSubresource(
                dest.handle(),
                dest_resource,
                src.handle(),
                src_subresource,
                format,
            );
        }
    }

    #[inline]
    pub fn resource_barrier(&self, barriers: &[impl ResourceBarrier]) {
        let barriers = barriers
            .iter()
            .map(|b| b.as_raw().clone())
            .collect::<Vec<_>>();
        unsafe {
            self.cmd_list.ResourceBarrier(&barriers);
        }
    }

    #[inline]
    pub fn rs_set_scissor_rects(&self, rects: &[Rect]) {
        unsafe {
            self.cmd_list.RSSetScissorRects(rects);
        }
    }

    #[inline]
    pub fn rs_set_viewports(&self, viewports: &[D3D12_VIEWPORT]) {
        unsafe {
            self.cmd_list.RSSetViewports(viewports);
        }
    }

    #[inline]
    pub fn set_descriptor_heaps(
        &self,
        cbv_srv_uav: Option<&DescriptorHeap<CbvSrvUav>>,
        sampler: Option<&DescriptorHeap<Sampler>>,
    ) {
        unsafe {
            if cbv_srv_uav.is_some() && sampler.is_some() {
                self.cmd_list.SetDescriptorHeaps(&[
                    Some(cbv_srv_uav.unwrap().handle().clone()),
                    Some(sampler.unwrap().handle().clone()),
                ]);
            } else if cbv_srv_uav.is_some() {
                self.cmd_list
                    .SetDescriptorHeaps(&[Some(cbv_srv_uav.unwrap().handle().clone())]);
            } else if sampler.is_some() {
                self.cmd_list
                    .SetDescriptorHeaps(&[Some(sampler.unwrap().handle().clone())]);
            }
        }
    }

    #[inline]
    pub fn clear_state(&self, state: Option<&PipelineState>) {
        unsafe {
            self.cmd_list.ClearState(state.map(|s| s.handle()));
        }
    }

    #[inline]
    pub fn set_pipeline_state(&self, state: &impl PipelineStateType) {
        state.call(&self.cmd_list);
    }

    #[inline]
    pub fn set_graphics_root_signature(&self, root_sig: &RootSignature) {
        unsafe {
            self.cmd_list.SetGraphicsRootSignature(root_sig.handle());
        }
    }

    #[inline]
    pub fn set_graphics_root_32bit_constants<U>(
        &self,
        root_parameter_index: u32,
        src_data: U,
        dest_offset_in_32bit_values: u32,
    ) where
        U: SetGraphicsRoot32BitConstants,
    {
        SetGraphicsRoot32BitConstants::call(
            src_data,
            &self.cmd_list,
            root_parameter_index,
            dest_offset_in_32bit_values,
        );
    }

    #[inline]
    pub fn set_graphics_root_descriptor_table<D>(
        &self,
        root_parameter_index: u32,
        base_descriptor: &GpuDescriptorHandle<D>,
    ) {
        unsafe {
            self.cmd_list
                .SetGraphicsRootDescriptorTable(root_parameter_index, base_descriptor.handle());
        }
    }

    #[inline]
    pub fn set_graphics_root_constant_buffer_view(
        &self,
        root_parameter_index: u32,
        location: GpuVirtualAddress,
    ) {
        unsafe {
            self.cmd_list
                .SetGraphicsRootConstantBufferView(root_parameter_index, location.0);
        }
    }

    #[inline]
    pub fn set_graphics_root_shader_resource_view(
        &self,
        root_parameter_index: u32,
        location: GpuVirtualAddress,
    ) {
        unsafe {
            self.cmd_list
                .SetGraphicsRootShaderResourceView(root_parameter_index, location.0);
        }
    }

    #[inline]
    pub fn set_graphics_root_unordered_access_view(
        &self,
        root_parameter_index: u32,
        location: GpuVirtualAddress,
    ) {
        unsafe {
            self.cmd_list
                .SetGraphicsRootUnorderedAccessView(root_parameter_index, location.0);
        }
    }

    #[inline]
    pub fn set_compute_root_signature(&self, root_sig: &RootSignature) {
        unsafe {
            self.cmd_list.SetComputeRootSignature(root_sig.handle());
        }
    }

    #[inline]
    pub fn set_compute_root_32bit_constants<U>(
        &self,
        root_parameter_index: u32,
        src_data: U,
        dest_offset_in_32bit_values: u32,
    ) where
        U: SetComputeRoot32BitConstants,
    {
        SetComputeRoot32BitConstants::call(
            src_data,
            &self.cmd_list,
            root_parameter_index,
            dest_offset_in_32bit_values,
        );
    }

    #[inline]
    pub fn set_compute_root_descriptor_table<D>(
        &self,
        root_parameter_index: u32,
        base_descriptor: &GpuDescriptorHandle<D>,
    ) {
        unsafe {
            self.cmd_list
                .SetComputeRootDescriptorTable(root_parameter_index, base_descriptor.handle());
        }
    }

    #[inline]
    pub fn set_compute_root_constant_buffer_view(
        &self,
        root_parameter_index: u32,
        location: GpuVirtualAddress,
    ) {
        unsafe {
            self.cmd_list
                .SetComputeRootConstantBufferView(root_parameter_index, location.0);
        }
    }

    #[inline]
    pub fn set_compute_root_shader_resource_view(
        &self,
        root_parameter_index: u32,
        location: GpuVirtualAddress,
    ) {
        unsafe {
            self.cmd_list
                .SetComputeRootShaderResourceView(root_parameter_index, location.0);
        }
    }

    #[inline]
    pub fn set_compute_root_unordered_access_view(
        &self,
        root_parameter_index: u32,
        location: GpuVirtualAddress,
    ) {
        unsafe {
            self.cmd_list
                .SetComputeRootUnorderedAccessView(root_parameter_index, location.0);
        }
    }

    #[inline]
    pub fn so_set_targets(&self, start_slot: u32, views: Option<&[StreamOutputBufferView]>) {
        unsafe {
            let views = views.map(|views| {
                std::slice::from_raw_parts(
                    views.as_ptr() as *const D3D12_STREAM_OUTPUT_BUFFER_VIEW,
                    views.len(),
                )
            });
            self.cmd_list.SOSetTargets(start_slot, views);
        }
    }

    #[inline]
    pub fn build_raytracing_acceleration_structure(
        &self,
        desc: &BuildRaytracingAccelerationStructureDesc,
    ) {
        unsafe {
            self.cmd_list
                .BuildRaytracingAccelerationStructure(&desc.0, None);
        }
    }
}

impl<'a> Commands<'a, Direct> {
    #[inline]
    pub fn discard_resource(&self, resource: &Resource, region: Option<&DiscardRegion>) {
        unsafe {
            self.cmd_list.DiscardResource(
                resource.handle(),
                region.map(|r| &r.region as *const D3D12_DISCARD_REGION),
            );
        }
    }
}

impl<'a> Commands<'a, Compute> {
    #[inline]
    pub fn discard_resource(&self, resource: &Resource, region: Option<&DiscardRegion>) {
        unsafe {
            self.cmd_list.DiscardResource(
                resource.handle(),
                region.map(|r| &r.region as *const D3D12_DISCARD_REGION),
            );
        }
    }
}

impl<'a> Commands<'a, Bundle> {}

pub struct Builder<T> {
    device: ID3D12Device,
    node_mask: u32,
    name: Option<String>,
    _t: std::marker::PhantomData<T>,
}

impl<T> Builder<T>
where
    T: CommandListType,
{
    fn new<U>(device: &U) -> Self
    where
        U: Into<ID3D12Device> + Clone,
    {
        let device: ID3D12Device = device.clone().into();
        Self {
            device,
            node_mask: 0,
            name: None,
            _t: std::marker::PhantomData,
        }
    }

    #[inline]
    pub fn node_mask(mut self, mask: u32) -> Self {
        self.node_mask = mask;
        self
    }

    #[inline]
    pub fn name(mut self, name: impl AsRef<str>) -> Self {
        self.name = Some(name.as_ref().to_string());
        self
    }

    #[inline]
    pub fn build(self) -> windows::core::Result<GraphicsCommandList<T>> {
        let tmp_allocator: ID3D12CommandAllocator =
            unsafe { self.device.CreateCommandAllocator(T::VALUE)? };
        let handle: ID3D12GraphicsCommandList7 = unsafe {
            self.device
                .CreateCommandList(self.node_mask, T::VALUE, &tmp_allocator, None)?
        };
        let name = self.name.map(|n| Name::new(&handle, n));
        unsafe { handle.Close()? };
        Ok(GraphicsCommandList {
            handle,
            name,
            _t: std::marker::PhantomData,
        })
    }
}

#[derive(Clone, Debug)]
pub struct GraphicsCommandList<T = ()> {
    handle: ID3D12GraphicsCommandList7,
    name: Option<Name>,
    _t: std::marker::PhantomData<T>,
}

impl GraphicsCommandList<()> {
    #[inline]
    pub fn new_direct(device: &Device) -> Builder<Direct> {
        Builder::new(device.handle())
    }

    #[inline]
    pub fn new_compute(device: &Device) -> Builder<Compute> {
        Builder::new(device.handle())
    }

    #[inline]
    pub fn new_bundle(device: &Device) -> Builder<Bundle> {
        Builder::new(device.handle())
    }

    #[inline]
    pub fn new_copy(device: &Device) -> Builder<Copy> {
        Builder::new(device.handle())
    }

    #[inline]
    pub fn new_video_encode(device: &Device) -> Builder<VideoEncode> {
        Builder::new(device.handle())
    }

    #[inline]
    pub fn new_video_process(device: &Device) -> Builder<VideoProcess> {
        Builder::new(device.handle())
    }

    #[inline]
    pub fn new_video_decode(device: &Device) -> Builder<VideoDecode> {
        Builder::new(device.handle())
    }
}

impl<T> GraphicsCommandList<T>
where
    T: CommandListType,
{
    #[inline]
    pub fn record<F, R>(&self, allocator: &CommandAllocator<T>, f: F) -> windows::core::Result<R>
    where
        F: FnOnce(Commands<T>) -> R,
    {
        unsafe {
            allocator.reset()?;
            self.handle.Reset(allocator.handle(), None)?;
            let ret = f(Commands {
                cmd_list: &self.handle,
                _t: std::marker::PhantomData,
            });
            self.handle.Close()?;
            Ok(ret)
        }
    }

    #[inline]
    pub fn handle(&self) -> &ID3D12GraphicsCommandList7 {
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

impl GraphicsCommandList<command_list_type::Direct> {
    #[inline]
    pub fn new(device: &Device) -> Builder<command_list_type::Direct> {
        Builder::new(device.handle())
    }
}

impl GraphicsCommandList<command_list_type::Compute> {
    #[inline]
    pub fn new(device: &Device) -> Builder<command_list_type::Compute> {
        Builder::new(device.handle())
    }
}

impl GraphicsCommandList<command_list_type::Bundle> {
    #[inline]
    pub fn new(device: &Device) -> Builder<command_list_type::Bundle> {
        Builder::new(device.handle())
    }
}

impl GraphicsCommandList<command_list_type::Copy> {
    #[inline]
    pub fn new(device: &Device) -> Builder<command_list_type::Copy> {
        Builder::new(device.handle())
    }
}

impl GraphicsCommandList<command_list_type::VideoDecode> {
    #[inline]
    pub fn new(device: &Device) -> Builder<command_list_type::VideoDecode> {
        Builder::new(device.handle())
    }
}

impl GraphicsCommandList<command_list_type::VideoEncode> {
    #[inline]
    pub fn new(device: &Device) -> Builder<command_list_type::VideoEncode> {
        Builder::new(device.handle())
    }
}

impl GraphicsCommandList<command_list_type::VideoProcess> {
    #[inline]
    pub fn new(device: &Device) -> Builder<command_list_type::VideoProcess> {
        Builder::new(device.handle())
    }
}

pub type DirectGraphicsCommandList = GraphicsCommandList<command_list_type::Direct>;
pub type ComputeGraphicsCommandList = GraphicsCommandList<command_list_type::Compute>;
pub type BundleGraphicsCommandList = GraphicsCommandList<command_list_type::Bundle>;
pub type CopyGraphicsCommandList = GraphicsCommandList<command_list_type::Copy>;
pub type VideoDecodeGraphicsCommandList = GraphicsCommandList<command_list_type::VideoDecode>;
pub type VideoEncodeGraphicsCommandList = GraphicsCommandList<command_list_type::VideoEncode>;
pub type VideoProcessGraphicsCommandList = GraphicsCommandList<command_list_type::VideoProcess>;

pub type DirectCommands<'a> = Commands<'a, command_list_type::Direct>;
pub type ComputeCommands<'a> = Commands<'a, command_list::Compute>;
pub type BundleCommands<'a> = Commands<'a, command_list_type::Bundle>;
pub type CopyCommands<'a> = Commands<'a, command_list_type::Copy>;
pub type VideoDecodeCommands<'a> = Commands<'a, command_list_type::VideoDecode>;
pub type VideoEncodeCommands<'a> = Commands<'a, command_list_type::VideoEncode>;
pub type VideoProcessCommands<'a> = Commands<'a, command_list_type::VideoProcess>;
