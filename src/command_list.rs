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
    pub fn buffer_location(mut self, loc: u64) -> Self {
        self.view.BufferLocation = loc;
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
    pub fn buffer_location(mut self, loc: u64) -> Self {
        self.view.BufferLocation = loc;
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

pub struct Commands<'a, T> {
    cmd_list: &'a ID3D12GraphicsCommandList6,
    _t: std::marker::PhantomData<T>,
}

impl<'a, T> Commands<'a, T> {
    #[inline]
    pub fn clear_depth_stencil_view(
        &self,
        dsv: CpuDescriptorHandle<Dsv>,
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
        rtv: CpuDescriptorHandle<Rtv>,
        color: &[f32; 4],
        rects: Option<&[Rect]>,
    ) {
        unsafe {
            self.cmd_list
                .ClearRenderTargetView(rtv.handle(), color, rects);
        }
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
        src: &D3D12_TEXTURE_COPY_LOCATION,
        src_box: Option<&D3D12_BOX>,
        dest: &D3D12_TEXTURE_COPY_LOCATION,
        dest_x: u32,
        dest_y: u32,
        dest_z: u32,
    ) {
        unsafe {
            self.cmd_list.CopyTextureRegion(
                dest,
                dest_x,
                dest_y,
                dest_z,
                src,
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
        rtvs: Option<&[CpuDescriptorHandle<Rtv>]>,
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
    pub fn set_pipeline_state(&self, state: &PipelineState) {
        unsafe {
            self.cmd_list.SetPipelineState(state.handle());
        }
    }

    #[inline]
    pub fn set_graphics_root_signature(&self, root_sig: &RootSignature) {
        unsafe {
            self.cmd_list.SetGraphicsRootSignature(root_sig.handle());
        }
    }

    #[inline]
    pub fn set_compute_root_signature(&self, root_sig: &RootSignature) {
        unsafe {
            self.cmd_list.SetComputeRootSignature(root_sig.handle());
        }
    }
}

pub struct Builder<T> {
    device: ID3D12Device,
    node_mask: u32,
    name: Option<String>,
    _t: std::marker::PhantomData<T>,
}

impl<T> Builder<T>
where
    T: Type,
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
        let handle: ID3D12GraphicsCommandList6 = unsafe {
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
pub struct GraphicsCommandList<T> {
    handle: ID3D12GraphicsCommandList6,
    name: Option<Name>,
    _t: std::marker::PhantomData<T>,
}

impl<T> GraphicsCommandList<T>
where
    T: Type,
{
    #[inline]
    pub fn new(device: &Device, _ty: T) -> Builder<T> {
        Builder::new(device.handle())
    }

    #[inline]
    pub fn record<F, R>(&self, allocator: &CommandAllocator<T>, f: F) -> windows::core::Result<R>
    where
        F: FnOnce(Commands<T>) -> R,
    {
        unsafe {
            allocator.handle().Reset()?;
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
    pub fn handle(&self) -> &ID3D12GraphicsCommandList6 {
        &self.handle
    }

    #[inline]
    pub fn name(&self) -> Option<&str> {
        self.name.as_ref().map(|n| n.as_str())
    }
}
