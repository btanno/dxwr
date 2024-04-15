use super::*;
use std::mem::ManuallyDrop;
use windows::core::PCSTR;
use windows::Win32::Graphics::Direct3D12::*;
use windows::Win32::Graphics::Dxgi::Common::*;

#[derive(Clone, Debug)]
#[repr(transparent)]
pub struct ShaderBytecode<'a> {
    desc: D3D12_SHADER_BYTECODE,
    _a: std::marker::PhantomData<&'a ()>,
}

impl<'a> ShaderBytecode<'a> {
    #[inline]
    pub fn new(data: &'a [u8]) -> Self {
        Self {
            desc: D3D12_SHADER_BYTECODE {
                pShaderBytecode: data.as_ptr() as *const std::ffi::c_void,
                BytecodeLength: data.len(),
            },
            _a: std::marker::PhantomData,
        }
    }
}

#[derive(Clone, Debug)]
#[repr(transparent)]
pub struct SoDeclarationEntry<'a> {
    entry: D3D12_SO_DECLARATION_ENTRY,
    _a: std::marker::PhantomData<&'a ()>,
}

impl<'a> SoDeclarationEntry<'a> {
    #[inline]
    pub fn stream(mut self, index: u32) -> Self {
        self.entry.Stream = index;
        self
    }

    #[inline]
    pub fn semantic_name(self, name: Option<&'a str>) -> SoDeclarationEntry<'a> {
        SoDeclarationEntry {
            entry: D3D12_SO_DECLARATION_ENTRY {
                SemanticName: name.map_or(PCSTR::null(), |n| PCSTR(n.as_ptr())),
                ..Default::default()
            },
            _a: std::marker::PhantomData,
        }
    }

    #[inline]
    pub fn semantic_index(mut self, index: u32) -> Self {
        self.entry.SemanticIndex = index;
        self
    }

    #[inline]
    pub fn start_component(mut self, component: u8) -> Self {
        self.entry.StartComponent = component;
        self
    }

    #[inline]
    pub fn component_count(mut self, count: u8) -> Self {
        self.entry.ComponentCount = count;
        self
    }

    #[inline]
    pub fn output_slot(mut self, slot: u8) -> Self {
        self.entry.OutputSlot = slot;
        self
    }
}

#[derive(Clone, Debug)]
#[repr(transparent)]
pub struct StreamOutputDesc<'decl, 'strides> {
    desc: D3D12_STREAM_OUTPUT_DESC,
    _decl: std::marker::PhantomData<&'decl ()>,
    _strides: std::marker::PhantomData<&'strides ()>,
}

impl<'decl, 'strides> StreamOutputDesc<'decl, 'strides> {
    #[inline]
    pub fn new() -> Self {
        Self {
            desc: Default::default(),
            _decl: std::marker::PhantomData,
            _strides: std::marker::PhantomData,
        }
    }

    #[inline]
    pub fn so_declaration<'a>(
        self,
        entry: &'a [SoDeclarationEntry],
    ) -> StreamOutputDesc<'a, 'strides> {
        StreamOutputDesc {
            desc: D3D12_STREAM_OUTPUT_DESC {
                pSODeclaration: entry.as_ptr() as *const D3D12_SO_DECLARATION_ENTRY,
                NumEntries: entry.len() as u32,
                ..self.desc
            },
            _decl: std::marker::PhantomData,
            ..self
        }
    }

    #[inline]
    pub fn buffer_strides<'a>(self, strides: &'a [u32]) -> StreamOutputDesc<'decl, 'a> {
        StreamOutputDesc {
            desc: D3D12_STREAM_OUTPUT_DESC {
                pBufferStrides: strides.as_ptr(),
                NumStrides: strides.len() as u32,
                ..self.desc
            },
            _strides: std::marker::PhantomData,
            ..self
        }
    }

    #[inline]
    pub fn rasterized_stream(mut self, index: u32) -> Self {
        self.desc.RasterizedStream = index;
        self
    }
}

pub struct RenderTargetBlendDesc<'a>(&'a mut D3D12_RENDER_TARGET_BLEND_DESC);

impl<'a> RenderTargetBlendDesc<'a> {
    #[inline]
    pub fn blend_enable(&mut self, value: bool) {
        self.0.BlendEnable = value.into();
    }

    #[inline]
    pub fn logic_op_enable(&mut self, value: bool) {
        self.0.LogicOpEnable = value.into();
    }

    #[inline]
    pub fn src_blend(&mut self, value: D3D12_BLEND) {
        self.0.SrcBlend = value;
    }

    #[inline]
    pub fn dest_blend(&mut self, value: D3D12_BLEND) {
        self.0.DestBlend = value;
    }

    #[inline]
    pub fn blend_op(&mut self, value: D3D12_BLEND_OP) {
        self.0.BlendOp = value;
    }

    #[inline]
    pub fn src_blend_alpha(&mut self, value: D3D12_BLEND) {
        self.0.SrcBlendAlpha = value;
    }

    #[inline]
    pub fn dest_blend_alpha(&mut self, value: D3D12_BLEND) {
        self.0.DestBlendAlpha = value;
    }

    #[inline]
    pub fn blend_op_alpha(&mut self, value: D3D12_BLEND_OP) {
        self.0.BlendOpAlpha = value;
    }

    #[inline]
    pub fn logic_op(&mut self, value: D3D12_LOGIC_OP) {
        self.0.LogicOp = value;
    }

    #[inline]
    pub fn render_target_write_mask(&mut self, value: u8) {
        self.0.RenderTargetWriteMask = value;
    }
}

#[derive(Clone, Debug)]
#[repr(transparent)]
pub struct BlendDesc(D3D12_BLEND_DESC);

impl BlendDesc {
    #[inline]
    pub fn new() -> Self {
        let mut desc = D3D12_BLEND_DESC::default();
        desc.RenderTarget[0].SrcBlend = D3D12_BLEND_ONE;
        desc.RenderTarget[0].DestBlend = D3D12_BLEND_ZERO;
        desc.RenderTarget[0].BlendOp = D3D12_BLEND_OP_ADD;
        desc.RenderTarget[0].SrcBlendAlpha = D3D12_BLEND_ONE;
        desc.RenderTarget[0].DestBlendAlpha = D3D12_BLEND_ZERO;
        desc.RenderTarget[0].BlendOpAlpha = D3D12_BLEND_OP_ADD;
        desc.RenderTarget[0].LogicOp = D3D12_LOGIC_OP_NOOP;
        desc.RenderTarget[0].RenderTargetWriteMask = D3D12_COLOR_WRITE_ENABLE_ALL.0 as u8;
        Self(desc)
    }

    #[inline]
    pub fn alpha_to_coverage_enable(mut self, value: bool) -> Self {
        self.0.AlphaToCoverageEnable = value.into();
        self
    }

    #[inline]
    pub fn independent_blend_enable(mut self, value: bool) -> Self {
        self.0.IndependentBlendEnable = value.into();
        self
    }

    #[inline]
    pub fn render_target_blends<F>(mut self, f: F) -> Self
    where
        F: FnOnce(&mut [RenderTargetBlendDesc; 8]),
    {
        let mut rtbs = self
            .0
            .RenderTarget
            .each_mut()
            .map(|rt| RenderTargetBlendDesc(rt));
        f(&mut rtbs);
        self
    }
}

impl Default for BlendDesc {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Clone, Debug)]
#[repr(transparent)]
pub struct RasterizerDesc(D3D12_RASTERIZER_DESC);

impl RasterizerDesc {
    #[inline]
    pub fn new() -> Self {
        Self(D3D12_RASTERIZER_DESC {
            FillMode: D3D12_FILL_MODE_SOLID,
            CullMode: D3D12_CULL_MODE_BACK,
            DepthClipEnable: true.into(),
            ..Default::default()
        })
    }

    #[inline]
    pub fn fill_mode(mut self, mode: D3D12_FILL_MODE) -> Self {
        self.0.FillMode = mode;
        self
    }

    #[inline]
    pub fn cull_mode(mut self, mode: D3D12_CULL_MODE) -> Self {
        self.0.CullMode = mode;
        self
    }

    #[inline]
    pub fn front_counter_clockwise(mut self, value: bool) -> Self {
        self.0.FrontCounterClockwise = value.into();
        self
    }

    #[inline]
    pub fn depth_bias(mut self, value: i32) -> Self {
        self.0.DepthBias = value;
        self
    }

    #[inline]
    pub fn depth_bias_clamp(mut self, value: f32) -> Self {
        self.0.DepthBiasClamp = value;
        self
    }

    #[inline]
    pub fn slope_scaled_depth_bias(mut self, value: f32) -> Self {
        self.0.SlopeScaledDepthBias = value;
        self
    }

    #[inline]
    pub fn depth_clip_enable(mut self, value: bool) -> Self {
        self.0.DepthClipEnable = value.into();
        self
    }

    #[inline]
    pub fn multisample_enable(mut self, value: bool) -> Self {
        self.0.MultisampleEnable = value.into();
        self
    }

    #[inline]
    pub fn antialiased_line_enable(mut self, value: bool) -> Self {
        self.0.AntialiasedLineEnable = value.into();
        self
    }

    #[inline]
    pub fn force_sample_count(mut self, value: u32) -> Self {
        self.0.ForcedSampleCount = value;
        self
    }

    #[inline]
    pub fn conservative_raster(mut self, value: D3D12_CONSERVATIVE_RASTERIZATION_MODE) -> Self {
        self.0.ConservativeRaster = value;
        self
    }
}

impl Default for RasterizerDesc {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Clone, Debug)]
#[repr(transparent)]
pub struct DepthStencilOpDesc(D3D12_DEPTH_STENCILOP_DESC);

impl DepthStencilOpDesc {
    #[inline]
    pub fn new() -> Self {
        Self(D3D12_DEPTH_STENCILOP_DESC {
            StencilFailOp: D3D12_STENCIL_OP_KEEP,
            StencilDepthFailOp: D3D12_STENCIL_OP_KEEP,
            StencilPassOp: D3D12_STENCIL_OP_KEEP,
            StencilFunc: D3D12_COMPARISON_FUNC_ALWAYS,
        })
    }

    #[inline]
    pub fn stencil_fail_op(mut self, op: D3D12_STENCIL_OP) -> Self {
        self.0.StencilFailOp = op;
        self
    }

    #[inline]
    pub fn stencil_depth_fail_op(mut self, op: D3D12_STENCIL_OP) -> Self {
        self.0.StencilDepthFailOp = op;
        self
    }

    #[inline]
    pub fn stencil_pass_op(mut self, op: D3D12_STENCIL_OP) -> Self {
        self.0.StencilPassOp = op;
        self
    }

    #[inline]
    pub fn stencil_func(mut self, func: D3D12_COMPARISON_FUNC) -> Self {
        self.0.StencilFunc = func;
        self
    }
}

impl Default for DepthStencilOpDesc {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Clone, Debug)]
#[repr(transparent)]
pub struct DepthStencilDesc(D3D12_DEPTH_STENCIL_DESC);

impl DepthStencilDesc {
    #[inline]
    pub fn new() -> Self {
        Self(D3D12_DEPTH_STENCIL_DESC {
            DepthEnable: true.into(),
            DepthWriteMask: D3D12_DEPTH_WRITE_MASK_ALL,
            DepthFunc: D3D12_COMPARISON_FUNC_LESS,
            StencilEnable: false.into(),
            StencilReadMask: D3D12_DEFAULT_STENCIL_READ_MASK as u8,
            StencilWriteMask: D3D12_DEFAULT_STENCIL_WRITE_MASK as u8,
            FrontFace: DepthStencilOpDesc::default().0,
            BackFace: DepthStencilOpDesc::default().0,
        })
    }

    #[inline]
    pub fn depth_enable(mut self, value: bool) -> Self {
        self.0.DepthEnable = value.into();
        self
    }

    #[inline]
    pub fn depth_write_mask(mut self, mask: D3D12_DEPTH_WRITE_MASK) -> Self {
        self.0.DepthWriteMask = mask;
        self
    }

    #[inline]
    pub fn depth_func(mut self, func: D3D12_COMPARISON_FUNC) -> Self {
        self.0.DepthFunc = func;
        self
    }

    #[inline]
    pub fn stencil_enable(mut self, value: bool) -> Self {
        self.0.StencilEnable = value.into();
        self
    }

    #[inline]
    pub fn stencil_read_mask(mut self, value: u8) -> Self {
        self.0.StencilReadMask = value;
        self
    }

    #[inline]
    pub fn stencil_write_mask(mut self, value: u8) -> Self {
        self.0.StencilWriteMask = value;
        self
    }

    #[inline]
    pub fn front_face(mut self, desc: &DepthStencilOpDesc) -> Self {
        self.0.FrontFace = desc.clone().0;
        self
    }

    #[inline]
    pub fn back_face(mut self, desc: &DepthStencilOpDesc) -> Self {
        self.0.BackFace = desc.clone().0;
        self
    }
}

impl Default for DepthStencilDesc {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Clone, Debug)]
#[repr(transparent)]
pub struct InputElementDesc<'a> {
    desc: D3D12_INPUT_ELEMENT_DESC,
    _a: std::marker::PhantomData<&'a ()>,
}

impl<'a> InputElementDesc<'a> {
    #[inline]
    pub fn new() -> Self {
        Self {
            desc: D3D12_INPUT_ELEMENT_DESC {
                AlignedByteOffset: D3D12_APPEND_ALIGNED_ELEMENT,
                InputSlotClass: D3D12_INPUT_CLASSIFICATION_PER_VERTEX_DATA,
                ..Default::default()
            },
            _a: std::marker::PhantomData,
        }
    }

    #[inline]
    pub fn semantic_name<'b>(self, name: &'b [u8]) -> InputElementDesc<'b> {
        InputElementDesc {
            desc: D3D12_INPUT_ELEMENT_DESC {
                SemanticName: PCSTR(name.as_ptr()),
                ..self.desc
            },
            _a: std::marker::PhantomData,
        }
    }

    #[inline]
    pub fn semantic_index(mut self, index: u32) -> Self {
        self.desc.SemanticIndex = index;
        self
    }

    #[inline]
    pub fn format(mut self, format: DXGI_FORMAT) -> Self {
        self.desc.Format = format;
        self
    }

    #[inline]
    pub fn input_slot(mut self, slot: u32) -> Self {
        self.desc.InputSlot = slot;
        self
    }

    #[inline]
    pub fn aligned_byte_offset(mut self, offset: u32) -> Self {
        self.desc.AlignedByteOffset = offset;
        self
    }

    #[inline]
    pub fn input_slot_class(mut self, class: D3D12_INPUT_CLASSIFICATION) -> Self {
        self.desc.InputSlotClass = class;
        self
    }

    #[inline]
    pub fn instance_data_step_rate(mut self, step_rate: u32) -> Self {
        self.desc.InstanceDataStepRate = step_rate;
        self
    }
}

#[derive(Clone, Debug)]
pub struct GraphicsPipelineStateDesc<'vs, 'ps, 'ds, 'hs, 'gs, 'so_decl, 'so_strides, 'input_layout>
{
    desc: D3D12_GRAPHICS_PIPELINE_STATE_DESC,
    _vs: std::marker::PhantomData<&'vs ()>,
    _ps: std::marker::PhantomData<&'ps ()>,
    _ds: std::marker::PhantomData<&'ds ()>,
    _hs: std::marker::PhantomData<&'hs ()>,
    _gs: std::marker::PhantomData<&'gs ()>,
    _so_decl: std::marker::PhantomData<&'so_decl ()>,
    _so_strides: std::marker::PhantomData<&'so_strides ()>,
    _input_layout: std::marker::PhantomData<&'input_layout ()>,
}

impl<'vs, 'ps, 'ds, 'hs, 'gs, 'so_decl, 'so_strides, 'input_layout>
    GraphicsPipelineStateDesc<'vs, 'ps, 'ds, 'hs, 'gs, 'so_decl, 'so_strides, 'input_layout>
{
    #[inline]
    pub fn new() -> Self {
        Self {
            desc: D3D12_GRAPHICS_PIPELINE_STATE_DESC {
                BlendState: BlendDesc::default().0,
                RasterizerState: RasterizerDesc::default().0,
                DepthStencilState: DepthStencilDesc::default().0,
                SampleMask: u32::MAX,
                SampleDesc: DXGI_SAMPLE_DESC {
                    Count: 1,
                    Quality: 0,
                },
                ..Default::default()
            },
            _vs: std::marker::PhantomData,
            _ps: std::marker::PhantomData,
            _ds: std::marker::PhantomData,
            _hs: std::marker::PhantomData,
            _gs: std::marker::PhantomData,
            _so_decl: std::marker::PhantomData,
            _so_strides: std::marker::PhantomData,
            _input_layout: std::marker::PhantomData,
        }
    }

    #[inline]
    pub fn root_signature(mut self, root_sig: &RootSignature) -> Self {
        self.desc.pRootSignature = ManuallyDrop::new(Some(root_sig.handle().clone()));
        self
    }

    #[inline]
    pub fn vs<'a>(
        self,
        shader_bytecode: ShaderBytecode<'a>,
    ) -> GraphicsPipelineStateDesc<'a, 'ps, 'ds, 'hs, 'gs, 'so_decl, 'so_strides, 'input_layout>
    {
        GraphicsPipelineStateDesc {
            desc: D3D12_GRAPHICS_PIPELINE_STATE_DESC {
                VS: shader_bytecode.desc,
                ..self.desc
            },
            _vs: std::marker::PhantomData,
            ..self
        }
    }

    #[inline]
    pub fn ps<'a>(
        self,
        shader_bytecode: ShaderBytecode<'a>,
    ) -> GraphicsPipelineStateDesc<'vs, 'a, 'ds, 'hs, 'gs, 'so_decl, 'so_strides, 'input_layout>
    {
        GraphicsPipelineStateDesc {
            desc: D3D12_GRAPHICS_PIPELINE_STATE_DESC {
                PS: shader_bytecode.desc,
                ..self.desc
            },
            _ps: std::marker::PhantomData,
            ..self
        }
    }

    #[inline]
    pub fn ds<'a>(
        self,
        shader_bytecode: ShaderBytecode<'a>,
    ) -> GraphicsPipelineStateDesc<'vs, 'ps, 'a, 'hs, 'gs, 'so_decl, 'so_strides, 'input_layout>
    {
        GraphicsPipelineStateDesc {
            desc: D3D12_GRAPHICS_PIPELINE_STATE_DESC {
                DS: shader_bytecode.desc,
                ..self.desc
            },
            _ds: std::marker::PhantomData,
            ..self
        }
    }

    #[inline]
    pub fn hs<'a>(
        self,
        shader_bytecode: ShaderBytecode<'a>,
    ) -> GraphicsPipelineStateDesc<'vs, 'ps, 'ds, 'a, 'gs, 'so_decl, 'so_strides, 'input_layout>
    {
        GraphicsPipelineStateDesc {
            desc: D3D12_GRAPHICS_PIPELINE_STATE_DESC {
                HS: shader_bytecode.desc,
                ..self.desc
            },
            _hs: std::marker::PhantomData,
            ..self
        }
    }

    #[inline]
    pub fn gs<'a>(
        self,
        shader_bytecode: ShaderBytecode<'a>,
    ) -> GraphicsPipelineStateDesc<'vs, 'ps, 'ds, 'hs, 'a, 'so_decl, 'so_strides, 'input_layout>
    {
        GraphicsPipelineStateDesc {
            desc: D3D12_GRAPHICS_PIPELINE_STATE_DESC {
                GS: shader_bytecode.desc,
                ..self.desc
            },
            _gs: std::marker::PhantomData,
            ..self
        }
    }

    #[inline]
    pub fn stream_output(mut self, desc: StreamOutputDesc) -> Self {
        self.desc.StreamOutput = desc.desc;
        self
    }

    #[inline]
    pub fn blend_state(mut self, desc: BlendDesc) -> Self {
        self.desc.BlendState = desc.0;
        self
    }

    #[inline]
    pub fn sample_mask(mut self, mask: u32) -> Self {
        self.desc.SampleMask = mask;
        self
    }

    #[inline]
    pub fn rasterizer_state(mut self, desc: RasterizerDesc) -> Self {
        self.desc.RasterizerState = desc.0;
        self
    }

    #[inline]
    pub fn depth_stencil_desc(mut self, desc: DepthStencilDesc) -> Self {
        self.desc.DepthStencilState = desc.0;
        self
    }

    #[inline]
    pub fn input_layout(mut self, elements: &[InputElementDesc]) -> Self {
        self.desc.InputLayout = D3D12_INPUT_LAYOUT_DESC {
            pInputElementDescs: elements.as_ptr() as *const D3D12_INPUT_ELEMENT_DESC,
            NumElements: elements.len() as u32,
        };
        self
    }

    #[inline]
    pub fn ib_strip_cut_value(mut self, value: D3D12_INDEX_BUFFER_STRIP_CUT_VALUE) -> Self {
        self.desc.IBStripCutValue = value;
        self
    }

    #[inline]
    pub fn primitive_topology_type(mut self, ty: D3D12_PRIMITIVE_TOPOLOGY_TYPE) -> Self {
        self.desc.PrimitiveTopologyType = ty;
        self
    }

    #[inline]
    pub fn rtv_formats(mut self, formats: &[DXGI_FORMAT]) -> Self {
        assert!(formats.len() <= 8);
        self.desc.RTVFormats[..formats.len()].copy_from_slice(formats);
        self.desc.NumRenderTargets = formats.len() as u32;
        self
    }

    #[inline]
    pub fn dsv_format(mut self, format: DXGI_FORMAT) -> Self {
        self.desc.DSVFormat = format;
        self
    }

    #[inline]
    pub fn sample_desc(mut self, desc: SampleDesc) -> Self {
        self.desc.SampleDesc = desc.0;
        self
    }

    #[inline]
    pub fn node_mask(mut self, mask: u32) -> Self {
        self.desc.NodeMask = mask;
        self
    }

    #[inline]
    pub fn cached_pso(mut self, state: D3D12_CACHED_PIPELINE_STATE) -> Self {
        self.desc.CachedPSO = state;
        self
    }

    #[inline]
    pub fn flags(mut self, flags: D3D12_PIPELINE_STATE_FLAGS) -> Self {
        self.desc.Flags = flags;
        self
    }
}

#[derive(Clone, Debug)]
pub struct ComputePipelineStateDesc<'cs> {
    desc: D3D12_COMPUTE_PIPELINE_STATE_DESC,
    _cs: std::marker::PhantomData<&'cs ()>,
}

impl<'cs> ComputePipelineStateDesc<'cs> {
    #[inline]
    pub fn new() -> Self {
        Self {
            desc: D3D12_COMPUTE_PIPELINE_STATE_DESC::default(),
            _cs: std::marker::PhantomData,
        }
    }

    #[inline]
    pub fn root_signature(mut self, root_sig: &RootSignature) -> Self {
        self.desc.pRootSignature = ManuallyDrop::new(Some(root_sig.handle().clone()));
        self
    }

    #[inline]
    pub fn cs<'a>(self, bytecode: ShaderBytecode<'a>) -> ComputePipelineStateDesc<'a> {
        ComputePipelineStateDesc {
            desc: D3D12_COMPUTE_PIPELINE_STATE_DESC {
                CS: bytecode.desc,
                ..self.desc
            },
            _cs: std::marker::PhantomData,
        }
    }

    #[inline]
    pub fn node_mask(mut self, mask: u32) -> Self {
        self.desc.NodeMask = mask;
        self
    }

    #[inline]
    pub fn cached_pso(mut self, state: D3D12_CACHED_PIPELINE_STATE) -> Self {
        self.desc.CachedPSO = state;
        self
    }

    #[inline]
    pub fn flags(mut self, flags: D3D12_PIPELINE_STATE_FLAGS) -> Self {
        self.desc.Flags = flags;
        self
    }
}

#[repr(transparent)]
pub struct Vs<'a>(pub ShaderBytecode<'a>);

#[repr(transparent)]
pub struct Ps<'a>(pub ShaderBytecode<'a>);

#[repr(transparent)]
pub struct Ds<'a>(pub ShaderBytecode<'a>);

#[repr(transparent)]
pub struct Hs<'a>(pub ShaderBytecode<'a>);

#[repr(transparent)]
pub struct Gs<'a>(pub ShaderBytecode<'a>);

#[repr(transparent)]
pub struct Ms<'a>(pub ShaderBytecode<'a>);

#[repr(transparent)]
pub struct As<'a>(pub ShaderBytecode<'a>);

#[repr(transparent)]
pub struct SampleMask(pub u32);

#[repr(transparent)]
pub struct IbStripCutValue(pub D3D12_INDEX_BUFFER_STRIP_CUT_VALUE);

#[repr(transparent)]
pub struct InputLayout<'a, 'b>(pub &'a [InputElementDesc<'b>]);

#[repr(transparent)]
pub struct PrimitiveTopologyType(pub D3D12_PRIMITIVE_TOPOLOGY_TYPE);

#[repr(transparent)]
pub struct RenderTargetFormats<'a>(pub &'a [DXGI_FORMAT]);

#[repr(transparent)]
pub struct DepthStencilFormat(pub DXGI_FORMAT);

#[repr(transparent)]
pub struct NodeMask(pub u32);

#[repr(transparent)]
pub struct PipelineFlags(pub D3D12_PIPELINE_STATE_FLAGS);

#[repr(transparent)]
pub struct ViewInstanceLocation(D3D12_VIEW_INSTANCE_LOCATION);

impl ViewInstanceLocation {
    #[inline]
    pub fn new() -> Self {
        Self(D3D12_VIEW_INSTANCE_LOCATION::default())
    }

    #[inline]
    pub fn viewport_array_index(mut self, index: u32) -> Self {
        self.0.ViewportArrayIndex = index;
        self
    }

    #[inline]
    pub fn render_target_array_index(mut self, index: u32) -> Self {
        self.0.RenderTargetArrayIndex = index;
        self
    }
}

#[repr(transparent)]
pub struct ViewInstancingDesc<'a> {
    desc: D3D12_VIEW_INSTANCING_DESC,
    _a: std::marker::PhantomData<&'a ()>,
}

impl<'a> ViewInstancingDesc<'a> {
    #[inline]
    pub fn new(locations: &'a [ViewInstanceLocation]) -> Self {
        Self {
            desc: D3D12_VIEW_INSTANCING_DESC {
                ViewInstanceCount: locations.len() as u32,
                pViewInstanceLocations: locations.as_ptr() as *const D3D12_VIEW_INSTANCE_LOCATION,
                ..Default::default()
            },
            _a: std::marker::PhantomData,
        }
    }

    #[inline]
    pub fn flags(mut self, flags: D3D12_VIEW_INSTANCING_FLAGS) -> Self {
        self.desc.Flags = flags;
        self
    }
}

pub trait Subobject {
    const VALUE: D3D12_PIPELINE_STATE_SUBOBJECT_TYPE;
    type Inner;

    fn new(self) -> Self::Inner;
}

impl Subobject for RootSignature {
    const VALUE: D3D12_PIPELINE_STATE_SUBOBJECT_TYPE =
        D3D12_PIPELINE_STATE_SUBOBJECT_TYPE_ROOT_SIGNATURE;
    type Inner = ID3D12RootSignature;

    fn new(self) -> Self::Inner {
        self.handle().clone()
    }
}

impl<'a> Subobject for Vs<'a> {
    const VALUE: D3D12_PIPELINE_STATE_SUBOBJECT_TYPE = D3D12_PIPELINE_STATE_SUBOBJECT_TYPE_VS;
    type Inner = Self;

    fn new(self) -> Self {
        self
    }
}

impl<'a> Subobject for Ps<'a> {
    const VALUE: D3D12_PIPELINE_STATE_SUBOBJECT_TYPE = D3D12_PIPELINE_STATE_SUBOBJECT_TYPE_PS;
    type Inner = Self;

    fn new(self) -> Self {
        self
    }
}

impl<'a> Subobject for Ds<'a> {
    const VALUE: D3D12_PIPELINE_STATE_SUBOBJECT_TYPE = D3D12_PIPELINE_STATE_SUBOBJECT_TYPE_DS;
    type Inner = Self;

    fn new(self) -> Self {
        self
    }
}

impl<'a> Subobject for Hs<'a> {
    const VALUE: D3D12_PIPELINE_STATE_SUBOBJECT_TYPE = D3D12_PIPELINE_STATE_SUBOBJECT_TYPE_HS;
    type Inner = Self;

    fn new(self) -> Self {
        self
    }
}

impl<'a> Subobject for Gs<'a> {
    const VALUE: D3D12_PIPELINE_STATE_SUBOBJECT_TYPE = D3D12_PIPELINE_STATE_SUBOBJECT_TYPE_GS;
    type Inner = Self;

    fn new(self) -> Self {
        self
    }
}

impl<'decl, 'strides> Subobject for StreamOutputDesc<'decl, 'strides> {
    const VALUE: D3D12_PIPELINE_STATE_SUBOBJECT_TYPE =
        D3D12_PIPELINE_STATE_SUBOBJECT_TYPE_STREAM_OUTPUT;
    type Inner = Self;

    fn new(self) -> Self {
        self
    }
}

impl Subobject for BlendDesc {
    const VALUE: D3D12_PIPELINE_STATE_SUBOBJECT_TYPE = D3D12_PIPELINE_STATE_SUBOBJECT_TYPE_BLEND;
    type Inner = Self;

    fn new(self) -> Self {
        self
    }
}

impl Subobject for SampleMask {
    const VALUE: D3D12_PIPELINE_STATE_SUBOBJECT_TYPE =
        D3D12_PIPELINE_STATE_SUBOBJECT_TYPE_SAMPLE_MASK;
    type Inner = Self;

    fn new(self) -> Self {
        self
    }
}

impl Subobject for RasterizerDesc {
    const VALUE: D3D12_PIPELINE_STATE_SUBOBJECT_TYPE =
        D3D12_PIPELINE_STATE_SUBOBJECT_TYPE_RASTERIZER;
    type Inner = Self;

    fn new(self) -> Self {
        self
    }
}

impl Subobject for DepthStencilDesc {
    const VALUE: D3D12_PIPELINE_STATE_SUBOBJECT_TYPE =
        D3D12_PIPELINE_STATE_SUBOBJECT_TYPE_DEPTH_STENCIL;
    type Inner = Self;

    fn new(self) -> Self {
        self
    }
}

impl<'a, 'b> Subobject for InputLayout<'a, 'b> {
    const VALUE: D3D12_PIPELINE_STATE_SUBOBJECT_TYPE =
        D3D12_PIPELINE_STATE_SUBOBJECT_TYPE_INPUT_LAYOUT;
    type Inner = Self;

    fn new(self) -> Self {
        self
    }
}

impl Subobject for IbStripCutValue {
    const VALUE: D3D12_PIPELINE_STATE_SUBOBJECT_TYPE =
        D3D12_PIPELINE_STATE_SUBOBJECT_TYPE_IB_STRIP_CUT_VALUE;
    type Inner = Self;

    fn new(self) -> Self {
        self
    }
}

impl Subobject for PrimitiveTopologyType {
    const VALUE: D3D12_PIPELINE_STATE_SUBOBJECT_TYPE =
        D3D12_PIPELINE_STATE_SUBOBJECT_TYPE_PRIMITIVE_TOPOLOGY;
    type Inner = Self;

    fn new(self) -> Self {
        self
    }
}

impl<'a> Subobject for RenderTargetFormats<'a> {
    const VALUE: D3D12_PIPELINE_STATE_SUBOBJECT_TYPE =
        D3D12_PIPELINE_STATE_SUBOBJECT_TYPE_RENDER_TARGET_FORMATS;
    type Inner = D3D12_RT_FORMAT_ARRAY;

    fn new(self) -> Self::Inner {
        let mut inner = D3D12_RT_FORMAT_ARRAY {
            RTFormats: [DXGI_FORMAT_UNKNOWN; 8],
            NumRenderTargets: self.0.len() as u32,
        };
        inner.RTFormats[..self.0.len()].copy_from_slice(self.0);
        inner
    }
}

impl Subobject for DepthStencilFormat {
    const VALUE: D3D12_PIPELINE_STATE_SUBOBJECT_TYPE =
        D3D12_PIPELINE_STATE_SUBOBJECT_TYPE_DEPTH_STENCIL_FORMAT;
    type Inner = Self;

    fn new(self) -> Self {
        self
    }
}

impl Subobject for SampleDesc {
    const VALUE: D3D12_PIPELINE_STATE_SUBOBJECT_TYPE =
        D3D12_PIPELINE_STATE_SUBOBJECT_TYPE_SAMPLE_DESC;
    type Inner = Self;

    fn new(self) -> Self {
        self
    }
}

#[repr(C, align(8))]
pub struct StreamSubobject<T: Subobject> {
    ty: D3D12_PIPELINE_STATE_SUBOBJECT_TYPE,
    inner: T::Inner,
}

impl<T> StreamSubobject<T>
where
    T: Subobject,
{
    #[inline]
    pub fn new(object: T) -> Self {
        Self {
            ty: T::VALUE,
            inner: object.new(),
        }
    }
}

impl<T> Default for StreamSubobject<T>
where
    T: Default + Subobject,
{
    #[inline]
    fn default() -> Self {
        Self::new(T::default())
    }
}

#[repr(transparent)]
pub struct PipelineStateStreamDesc<'a, T> {
    desc: D3D12_PIPELINE_STATE_STREAM_DESC,
    _a: std::marker::PhantomData<&'a T>,
}

impl<'a, T> PipelineStateStreamDesc<'a, T> {
    #[inline]
    pub fn new(object: &'a T) -> Self {
        Self {
            desc: D3D12_PIPELINE_STATE_STREAM_DESC {
                pPipelineStateSubobjectStream: object as *const _ as *mut _,
                SizeInBytes: std::mem::size_of::<T>(),
            },
            _a: std::marker::PhantomData,
        }
    }
}

pub struct Builder<T = ()> {
    device: ID3D12Device4,
    desc: T,
    name: Option<String>,
}

impl Builder<()> {
    #[inline]
    fn new<T>(device: &T) -> Self
    where
        T: Into<ID3D12Device4> + Clone,
    {
        let device = device.clone().into();
        Self {
            device,
            desc: (),
            name: None,
        }
    }
}

impl<T> Builder<T> {
    #[inline]
    pub fn desc<U>(self, desc: U) -> Builder<U> {
        Builder {
            device: self.device,
            desc,
            name: self.name,
        }
    }

    #[inline]
    pub fn name(mut self, name: impl AsRef<str>) -> Self {
        self.name = Some(name.as_ref().to_string());
        self
    }
}

impl<'vs, 'ps, 'ds, 'hs, 'gs, 'so_decl, 'so_strides, 'input_layout>
    Builder<
        GraphicsPipelineStateDesc<'vs, 'ps, 'ds, 'hs, 'gs, 'so_decl, 'so_strides, 'input_layout>,
    >
{
    #[inline]
    pub fn build(self) -> windows::core::Result<PipelineState> {
        let handle = unsafe { self.device.CreateGraphicsPipelineState(&self.desc.desc) };
        ManuallyDrop::into_inner(self.desc.desc.pRootSignature);
        let handle = handle?;
        let name = self.name.as_ref().map(|n| Name::new(&handle, n));
        Ok(PipelineState { handle, name })
    }
}

impl<'cs> Builder<ComputePipelineStateDesc<'cs>> {
    #[inline]
    pub fn build(self) -> windows::core::Result<PipelineState> {
        let handle = unsafe { self.device.CreateComputePipelineState(&self.desc.desc) };
        ManuallyDrop::into_inner(self.desc.desc.pRootSignature);
        let handle = handle?;
        let name = self.name.as_ref().map(|n| Name::new(&handle, n));
        Ok(PipelineState { handle, name })
    }
}

impl<'a, T> Builder<PipelineStateStreamDesc<'a, T>> {
    #[inline]
    pub fn build(self) -> windows::core::Result<PipelineState> {
        let handle = unsafe { self.device.CreatePipelineState(&self.desc.desc)? };
        let name = self.name.as_ref().map(|n| Name::new(&handle, n));
        Ok(PipelineState { handle, name })
    }
}

#[derive(Clone, Debug)]
pub struct PipelineState {
    handle: ID3D12PipelineState,
    name: Option<Name>,
}

impl PipelineState {
    #[inline]
    pub fn new(device: &Device) -> Builder {
        Builder::new(device.handle())
    }

    #[inline]
    pub fn handle(&self) -> &ID3D12PipelineState {
        &self.handle
    }

    #[inline]
    pub fn name(&self) -> Option<&str> {
        self.name.as_ref().map(|n| n.as_str())
    }
}
