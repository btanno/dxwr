use super::*;
use windows::core::GUID;
use windows::Win32::Graphics::Direct3D::*;
use windows::Win32::Graphics::Direct3D12::*;
use windows::Win32::Graphics::Dxgi::Common::*;

pub trait Feature: Sized {
    fn check(device: &DeviceType) -> windows::core::Result<Self>;
}

pub trait RequestFeature: Sized {
    fn check(device: &DeviceType, feature: Self) -> windows::core::Result<Self>;
}

fn get_feature<T: Default>(device: &DeviceType, value: D3D12_FEATURE) -> windows::core::Result<T> {
    let mut tmp = T::default();
    unsafe {
        device.CheckFeatureSupport(
            value,
            &mut tmp as *mut _ as *mut std::ffi::c_void,
            std::mem::size_of::<T>() as u32,
        )?;
    }
    Ok(tmp)
}

fn request_feature<T>(
    device: &DeviceType,
    value: D3D12_FEATURE,
    feature: &mut T,
) -> windows::core::Result<()> {
    unsafe {
        device.CheckFeatureSupport(
            value,
            feature as *mut _ as *mut std::ffi::c_void,
            std::mem::size_of::<T>() as u32,
        )?;
    }
    Ok(())
}

#[derive(Debug)]
pub struct D3D12Options {
    pub double_precision_float_shader_ops: bool,
    pub output_merger_logic_op: bool,
    pub min_precision_support: D3D12_SHADER_MIN_PRECISION_SUPPORT,
    pub tiled_resources_tier: D3D12_TILED_RESOURCES_TIER,
    pub resource_binding_tier: D3D12_RESOURCE_BINDING_TIER,
    pub ps_specified_stencil_ref_supported: bool,
    pub typed_uav_load_additional_formats: bool,
    pub rovs_supported: bool,
    pub conservative_rasterization_tier: D3D12_CONSERVATIVE_RASTERIZATION_TIER,
    pub max_gpu_virtual_address_bits_per_resource: u32,
    pub standard_swizzle_64kb_supported: bool,
    pub cross_node_sharing_tier: D3D12_CROSS_NODE_SHARING_TIER,
    pub cross_adapter_row_major_texture_supported: bool,
    pub vp_and_rt_array_index_from_any_shader_feeding_rasterizer_supported_without_gs_emulation:
        bool,
    pub resource_heap_tier: D3D12_RESOURCE_HEAP_TIER,
}

impl Feature for D3D12Options {
    fn check(device: &DeviceType) -> windows::core::Result<Self> {
        let tmp: D3D12_FEATURE_DATA_D3D12_OPTIONS =
            get_feature(device, D3D12_FEATURE_D3D12_OPTIONS)?;
        Ok(Self {
            double_precision_float_shader_ops: tmp.DoublePrecisionFloatShaderOps.into(),
            output_merger_logic_op: tmp.OutputMergerLogicOp.into(),
            min_precision_support: tmp.MinPrecisionSupport,
            tiled_resources_tier: tmp.TiledResourcesTier,
            resource_binding_tier: tmp.ResourceBindingTier,
            ps_specified_stencil_ref_supported: tmp.PSSpecifiedStencilRefSupported.into(),
            typed_uav_load_additional_formats: tmp.TypedUAVLoadAdditionalFormats.into(),
            rovs_supported: tmp.ROVsSupported.into(),
            conservative_rasterization_tier: tmp.ConservativeRasterizationTier,
            max_gpu_virtual_address_bits_per_resource: tmp.MaxGPUVirtualAddressBitsPerResource,
            standard_swizzle_64kb_supported: tmp.StandardSwizzle64KBSupported.into(),
            cross_node_sharing_tier: tmp.CrossNodeSharingTier,
            cross_adapter_row_major_texture_supported: tmp.CrossAdapterRowMajorTextureSupported.into(),
            vp_and_rt_array_index_from_any_shader_feeding_rasterizer_supported_without_gs_emulation:
                tmp.VPAndRTArrayIndexFromAnyShaderFeedingRasterizerSupportedWithoutGSEmulation.into(),
            resource_heap_tier: tmp.ResourceHeapTier,
        })
    }
}

#[derive(Debug, Default)]
pub struct Architecture {
    pub node_index: u32,
    pub tile_based_renderer: bool,
    pub uma: bool,
    pub cache_coherent_uma: bool,
    pub isolate_mmu: bool,
}

impl Architecture {
    pub fn new(node_index: u32) -> Self {
        Self {
            node_index,
            ..Default::default()
        }
    }
}

impl RequestFeature for Architecture {
    fn check(device: &DeviceType, feature: Self) -> windows::core::Result<Self> {
        let mut tmp = D3D12_FEATURE_DATA_ARCHITECTURE1 {
            NodeIndex: feature.node_index,
            ..Default::default()
        };
        request_feature(device, D3D12_FEATURE_ARCHITECTURE1, &mut tmp)?;
        Ok(Self {
            node_index: tmp.NodeIndex,
            tile_based_renderer: tmp.TileBasedRenderer.into(),
            uma: tmp.UMA.into(),
            cache_coherent_uma: tmp.CacheCoherentUMA.into(),
            isolate_mmu: tmp.IsolatedMMU.into(),
        })
    }
}

#[derive(Debug)]
pub struct FeatureLevels {
    pub feature_levels_requested: Vec<D3D_FEATURE_LEVEL>,
    pub max_supported_feature_level: D3D_FEATURE_LEVEL,
}

impl FeatureLevels {
    pub fn new(request: &[D3D_FEATURE_LEVEL]) -> Self {
        Self {
            feature_levels_requested: request.to_vec(),
            max_supported_feature_level: Default::default(),
        }
    }
}

impl RequestFeature for FeatureLevels {
    fn check(device: &DeviceType, feature: Self) -> windows::core::Result<Self> {
        let mut tmp = D3D12_FEATURE_DATA_FEATURE_LEVELS {
            NumFeatureLevels: feature.feature_levels_requested.len() as u32,
            pFeatureLevelsRequested: feature.feature_levels_requested.as_ptr(),
            MaxSupportedFeatureLevel: Default::default(),
        };
        request_feature(device, D3D12_FEATURE_FEATURE_LEVELS, &mut tmp)?;
        Ok(Self {
            feature_levels_requested: feature.feature_levels_requested,
            max_supported_feature_level: tmp.MaxSupportedFeatureLevel,
        })
    }
}

#[derive(Debug, Default)]
pub struct FormatSupport {
    pub format: DXGI_FORMAT,
    pub support1: D3D12_FORMAT_SUPPORT1,
    pub support2: D3D12_FORMAT_SUPPORT2,
}

impl FormatSupport {
    #[inline]
    pub fn new(format: DXGI_FORMAT) -> Self {
        Self {
            format,
            ..Default::default()
        }
    }
}

impl RequestFeature for FormatSupport {
    fn check(device: &DeviceType, feature: Self) -> windows::core::Result<Self> {
        let mut tmp = D3D12_FEATURE_DATA_FORMAT_SUPPORT {
            Format: feature.format,
            ..Default::default()
        };
        request_feature(device, D3D12_FEATURE_FORMAT_SUPPORT, &mut tmp)?;
        Ok(Self {
            format: tmp.Format,
            support1: tmp.Support1,
            support2: tmp.Support2,
        })
    }
}

#[derive(Debug, Default)]
pub struct MultisampleQualityLevels {
    pub format: DXGI_FORMAT,
    pub sample_count: u32,
    pub flags: D3D12_MULTISAMPLE_QUALITY_LEVEL_FLAGS,
    pub num_quality_levels: u32,
}

impl MultisampleQualityLevels {
    #[inline]
    pub fn new(format: DXGI_FORMAT, sample_count: u32) -> Self {
        Self {
            format,
            sample_count,
            ..Default::default()
        }
    }
}

impl RequestFeature for MultisampleQualityLevels {
    fn check(device: &DeviceType, feature: Self) -> windows::core::Result<Self> {
        let mut tmp = D3D12_FEATURE_DATA_MULTISAMPLE_QUALITY_LEVELS {
            Format: feature.format,
            SampleCount: feature.sample_count,
            ..Default::default()
        };
        request_feature(device, D3D12_FEATURE_MULTISAMPLE_QUALITY_LEVELS, &mut tmp)?;
        Ok(Self {
            format: tmp.Format,
            sample_count: tmp.SampleCount,
            flags: tmp.Flags,
            num_quality_levels: tmp.NumQualityLevels,
        })
    }
}

#[derive(Debug, Default)]
pub struct FormatInfo {
    pub format: DXGI_FORMAT,
    pub plane_count: u8,
}

impl FormatInfo {
    #[inline]
    pub fn new(format: DXGI_FORMAT) -> Self {
        Self {
            format,
            plane_count: 0,
        }
    }
}

impl RequestFeature for FormatInfo {
    fn check(device: &DeviceType, feature: Self) -> windows::core::Result<Self> {
        let mut tmp = D3D12_FEATURE_DATA_FORMAT_INFO {
            Format: feature.format,
            ..Default::default()
        };
        request_feature(device, D3D12_FEATURE_FORMAT_INFO, &mut tmp)?;
        Ok(Self {
            format: tmp.Format,
            plane_count: tmp.PlaneCount,
        })
    }
}

#[derive(Debug)]
pub struct GpuVirtualAddressSupport {
    pub max_gpu_virtual_address_bits_per_resource: u32,
    pub max_gpu_virtual_address_bits_per_process: u32,
}

impl Feature for GpuVirtualAddressSupport {
    fn check(device: &DeviceType) -> windows::core::Result<Self> {
        let tmp: D3D12_FEATURE_DATA_GPU_VIRTUAL_ADDRESS_SUPPORT =
            get_feature(device, D3D12_FEATURE_GPU_VIRTUAL_ADDRESS_SUPPORT)?;
        Ok(Self {
            max_gpu_virtual_address_bits_per_resource: tmp.MaxGPUVirtualAddressBitsPerResource,
            max_gpu_virtual_address_bits_per_process: tmp.MaxGPUVirtualAddressBitsPerProcess,
        })
    }
}

#[derive(Debug)]
pub struct ShaderModel {
    pub highest_shader_model: D3D_SHADER_MODEL,
}

impl Feature for ShaderModel {
    fn check(device: &DeviceType) -> windows::core::Result<Self> {
        let mut tmp = D3D12_FEATURE_DATA_SHADER_MODEL {
            HighestShaderModel: D3D_SHADER_MODEL_6_7,
        };
        request_feature(device, D3D12_FEATURE_SHADER_MODEL, &mut tmp)?;
        Ok(Self {
            highest_shader_model: tmp.HighestShaderModel,
        })
    }
}

#[derive(Debug)]
pub struct D3D12Options1 {
    pub wave_ops: bool,
    pub wave_lane_count_min: u32,
    pub wave_lane_count_max: u32,
    pub total_lane_count: u32,
    pub expanded_compute_resource_states: bool,
    pub int64_shader_ops: bool,
}

impl Feature for D3D12Options1 {
    fn check(device: &DeviceType) -> windows::core::Result<Self> {
        let tmp: D3D12_FEATURE_DATA_D3D12_OPTIONS1 =
            get_feature(device, D3D12_FEATURE_D3D12_OPTIONS1)?;
        Ok(Self {
            wave_ops: tmp.WaveOps.into(),
            wave_lane_count_min: tmp.WaveLaneCountMin,
            wave_lane_count_max: tmp.WaveLaneCountMax,
            total_lane_count: tmp.TotalLaneCount,
            expanded_compute_resource_states: tmp.ExpandedComputeResourceStates.into(),
            int64_shader_ops: tmp.Int64ShaderOps.into(),
        })
    }
}

#[derive(Debug, Default)]
pub struct ProtectedResourceSessionSupport {
    pub node_index: u32,
    pub support: D3D12_PROTECTED_RESOURCE_SESSION_SUPPORT_FLAGS,
}

impl ProtectedResourceSessionSupport {
    pub fn new(node_index: u32) -> Self {
        Self {
            node_index,
            ..Default::default()
        }
    }
}

impl RequestFeature for ProtectedResourceSessionSupport {
    fn check(device: &DeviceType, feature: Self) -> windows::core::Result<Self> {
        let mut tmp = D3D12_FEATURE_DATA_PROTECTED_RESOURCE_SESSION_SUPPORT {
            NodeIndex: feature.node_index,
            ..Default::default()
        };
        request_feature(
            device,
            D3D12_FEATURE_PROTECTED_RESOURCE_SESSION_SUPPORT,
            &mut tmp,
        )?;
        Ok(Self {
            node_index: tmp.NodeIndex,
            support: tmp.Support,
        })
    }
}

#[derive(Debug)]
pub struct RootSignature {
    pub highest_version: D3D_ROOT_SIGNATURE_VERSION,
}

impl Feature for RootSignature {
    fn check(device: &DeviceType) -> windows::core::Result<Self> {
        let mut tmp = D3D12_FEATURE_DATA_ROOT_SIGNATURE {
            HighestVersion: D3D_ROOT_SIGNATURE_VERSION_1_1,
        };
        request_feature(device, D3D12_FEATURE_ROOT_SIGNATURE, &mut tmp)?;
        Ok(Self {
            highest_version: tmp.HighestVersion,
        })
    }
}

#[derive(Debug)]
pub struct D3D12Options2 {
    pub depth_bounds_test_supported: bool,
    pub programmable_sample_positions_tier: D3D12_PROGRAMMABLE_SAMPLE_POSITIONS_TIER,
}

impl Feature for D3D12Options2 {
    fn check(device: &DeviceType) -> windows::core::Result<Self> {
        let tmp: D3D12_FEATURE_DATA_D3D12_OPTIONS2 =
            get_feature(device, D3D12_FEATURE_D3D12_OPTIONS2)?;
        Ok(Self {
            depth_bounds_test_supported: tmp.DepthBoundsTestSupported.into(),
            programmable_sample_positions_tier: tmp.ProgrammableSamplePositionsTier,
        })
    }
}

#[derive(Debug)]
pub struct ShaderCache {
    pub support_flags: D3D12_SHADER_CACHE_SUPPORT_FLAGS,
}

impl Feature for ShaderCache {
    fn check(device: &DeviceType) -> windows::core::Result<Self> {
        let tmp: D3D12_FEATURE_DATA_SHADER_CACHE = get_feature(device, D3D12_FEATURE_SHADER_CACHE)?;
        Ok(Self {
            support_flags: tmp.SupportFlags,
        })
    }
}

#[derive(Debug, Default)]
pub struct CommandQueuePriority {
    pub command_list_type: D3D12_COMMAND_LIST_TYPE,
    pub priority: u32,
    pub priority_for_type_is_supported: bool,
}

impl CommandQueuePriority {
    pub fn new(command_list_type: D3D12_COMMAND_LIST_TYPE, priority: u32) -> Self {
        Self {
            command_list_type,
            priority,
            ..Default::default()
        }
    }
}

impl RequestFeature for CommandQueuePriority {
    fn check(device: &DeviceType, feature: Self) -> windows::core::Result<Self> {
        let mut tmp = D3D12_FEATURE_DATA_COMMAND_QUEUE_PRIORITY {
            CommandListType: feature.command_list_type,
            Priority: feature.priority,
            ..Default::default()
        };
        request_feature(device, D3D12_FEATURE_COMMAND_QUEUE_PRIORITY, &mut tmp)?;
        Ok(Self {
            command_list_type: tmp.CommandListType,
            priority: tmp.Priority,
            priority_for_type_is_supported: tmp.PriorityForTypeIsSupported.into(),
        })
    }
}

#[derive(Debug)]
pub struct D3D12Options3 {
    pub copy_queue_timestamp_queries_supported: bool,
    pub casting_fully_typed_format_supported: bool,
    pub write_buffer_immediate_support_flags: D3D12_COMMAND_LIST_SUPPORT_FLAGS,
    pub view_instancing_tier: D3D12_VIEW_INSTANCING_TIER,
    pub barycentrics_supported: bool,
}

impl Feature for D3D12Options3 {
    fn check(device: &DeviceType) -> windows::core::Result<Self> {
        let tmp: D3D12_FEATURE_DATA_D3D12_OPTIONS3 =
            get_feature(device, D3D12_FEATURE_D3D12_OPTIONS3)?;
        Ok(Self {
            copy_queue_timestamp_queries_supported: tmp.CopyQueueTimestampQueriesSupported.into(),
            casting_fully_typed_format_supported: tmp.CastingFullyTypedFormatSupported.into(),
            write_buffer_immediate_support_flags: tmp.WriteBufferImmediateSupportFlags,
            view_instancing_tier: tmp.ViewInstancingTier,
            barycentrics_supported: tmp.BarycentricsSupported.into(),
        })
    }
}

#[derive(Debug)]
pub struct ExistingHeaps {
    pub supported: bool,
}

impl Feature for ExistingHeaps {
    fn check(device: &DeviceType) -> windows::core::Result<Self> {
        let tmp: D3D12_FEATURE_DATA_EXISTING_HEAPS =
            get_feature(device, D3D12_FEATURE_EXISTING_HEAPS)?;
        Ok(Self {
            supported: tmp.Supported.into(),
        })
    }
}

#[derive(Debug)]
pub struct D3D12Options4 {
    pub msaa_64kb_aligned_texture_supported: bool,
    pub shared_resource_compatibility_tier: D3D12_SHARED_RESOURCE_COMPATIBILITY_TIER,
    pub native_16bit_shader_ops_supported: bool,
}

impl Feature for D3D12Options4 {
    fn check(device: &DeviceType) -> windows::core::Result<Self> {
        let tmp: D3D12_FEATURE_DATA_D3D12_OPTIONS4 =
            get_feature(device, D3D12_FEATURE_D3D12_OPTIONS4)?;
        Ok(Self {
            msaa_64kb_aligned_texture_supported: tmp.MSAA64KBAlignedTextureSupported.into(),
            shared_resource_compatibility_tier: tmp.SharedResourceCompatibilityTier,
            native_16bit_shader_ops_supported: tmp.Native16BitShaderOpsSupported.into(),
        })
    }
}

#[derive(Debug, Default)]
pub struct Serialization {
    pub node_index: u32,
    pub heap_serialization_tier: D3D12_HEAP_SERIALIZATION_TIER,
}

impl Serialization {
    #[inline]
    pub fn new(node_index: u32) -> Self {
        Self {
            node_index,
            ..Default::default()
        }
    }
}

impl RequestFeature for Serialization {
    fn check(device: &DeviceType, feature: Self) -> windows::core::Result<Self> {
        let mut tmp = D3D12_FEATURE_DATA_SERIALIZATION {
            NodeIndex: feature.node_index,
            ..Default::default()
        };
        request_feature(device, D3D12_FEATURE_SERIALIZATION, &mut tmp)?;
        Ok(Self {
            node_index: tmp.NodeIndex,
            heap_serialization_tier: tmp.HeapSerializationTier,
        })
    }
}

#[derive(Debug)]
pub struct CrossNode {
    pub sharing_tier: D3D12_CROSS_NODE_SHARING_TIER,
    pub atomic_shader_instructions: bool,
}

impl Feature for CrossNode {
    fn check(device: &DeviceType) -> windows::core::Result<Self> {
        let tmp: D3D12_FEATURE_DATA_CROSS_NODE = get_feature(device, D3D12_FEATURE_CROSS_NODE)?;
        Ok(Self {
            sharing_tier: tmp.SharingTier,
            atomic_shader_instructions: tmp.AtomicShaderInstructions.into(),
        })
    }
}

#[derive(Debug)]
pub struct D3D12Options5 {
    pub srv_only_tiled_resource_tier3: bool,
    pub render_pass_tier: D3D12_RENDER_PASS_TIER,
    pub raytracing_tier: D3D12_RAYTRACING_TIER,
}

impl Feature for D3D12Options5 {
    fn check(device: &DeviceType) -> windows::core::Result<Self> {
        let tmp: D3D12_FEATURE_DATA_D3D12_OPTIONS5 =
            get_feature(device, D3D12_FEATURE_D3D12_OPTIONS5)?;
        Ok(Self {
            srv_only_tiled_resource_tier3: tmp.SRVOnlyTiledResourceTier3.into(),
            render_pass_tier: tmp.RenderPassesTier,
            raytracing_tier: tmp.RaytracingTier,
        })
    }
}

#[derive(Debug)]
pub struct Displayable {
    pub displayable_texture: bool,
    pub shared_resource_compatibility_tier: D3D12_SHARED_RESOURCE_COMPATIBILITY_TIER,
}

impl Feature for Displayable {
    fn check(device: &DeviceType) -> windows::core::Result<Self> {
        let tmp: D3D12_FEATURE_DATA_DISPLAYABLE = get_feature(device, D3D12_FEATURE_DISPLAYABLE)?;
        Ok(Self {
            displayable_texture: tmp.DisplayableTexture.into(),
            shared_resource_compatibility_tier: tmp.SharedResourceCompatibilityTier,
        })
    }
}

#[derive(Debug)]
pub struct D3D12Options6 {
    pub additional_shading_rates_supported: bool,
    pub per_primitive_shading_rate_supported_with_viewport_indexing: bool,
    pub variable_shading_rate_tier: D3D12_VARIABLE_SHADING_RATE_TIER,
    pub shading_rate_image_tile_size: u32,
    pub background_processing_supported: bool,
}

impl Feature for D3D12Options6 {
    fn check(device: &DeviceType) -> windows::core::Result<Self> {
        let tmp: D3D12_FEATURE_DATA_D3D12_OPTIONS6 =
            get_feature(device, D3D12_FEATURE_D3D12_OPTIONS6)?;
        Ok(Self {
            additional_shading_rates_supported: tmp.AdditionalShadingRatesSupported.into(),
            per_primitive_shading_rate_supported_with_viewport_indexing: tmp
                .PerPrimitiveShadingRateSupportedWithViewportIndexing
                .into(),
            variable_shading_rate_tier: tmp.VariableShadingRateTier,
            shading_rate_image_tile_size: tmp.ShadingRateImageTileSize,
            background_processing_supported: tmp.BackgroundProcessingSupported.into(),
        })
    }
}

#[derive(Debug)]
pub struct D3D12Options7 {
    pub mesh_shader_tier: D3D12_MESH_SHADER_TIER,
    pub sampler_feedback_tier: D3D12_SAMPLER_FEEDBACK_TIER,
}

impl Feature for D3D12Options7 {
    fn check(device: &DeviceType) -> windows::core::Result<Self> {
        let tmp: D3D12_FEATURE_DATA_D3D12_OPTIONS7 =
            get_feature(device, D3D12_FEATURE_D3D12_OPTIONS7)?;
        Ok(Self {
            mesh_shader_tier: tmp.MeshShaderTier,
            sampler_feedback_tier: tmp.SamplerFeedbackTier,
        })
    }
}

#[derive(Debug, Default)]
pub struct ProtectedResourceSessionTypeCount {
    pub node_index: u32,
    pub count: u32,
}

impl ProtectedResourceSessionTypeCount {
    pub fn new(node_index: u32) -> Self {
        Self {
            node_index,
            ..Default::default()
        }
    }
}

impl RequestFeature for ProtectedResourceSessionTypeCount {
    fn check(device: &DeviceType, feature: Self) -> windows::core::Result<Self> {
        let mut tmp = D3D12_FEATURE_DATA_PROTECTED_RESOURCE_SESSION_TYPE_COUNT {
            NodeIndex: feature.node_index,
            Count: 0,
        };
        request_feature(
            device,
            D3D12_FEATURE_PROTECTED_RESOURCE_SESSION_TYPE_COUNT,
            &mut tmp,
        )?;
        Ok(Self {
            node_index: tmp.NodeIndex,
            count: tmp.Count,
        })
    }
}

#[derive(Debug, Default)]
pub struct ProtectedResourceSessionTypes {
    pub node_index: u32,
    pub types: Vec<GUID>,
}

impl ProtectedResourceSessionTypes {
    pub fn new(node_index: u32) -> Self {
        Self {
            node_index,
            ..Default::default()
        }
    }
}

impl RequestFeature for ProtectedResourceSessionTypes {
    fn check(device: &DeviceType, feature: Self) -> windows::core::Result<Self> {
        let count = RequestFeature::check(
            device,
            ProtectedResourceSessionTypeCount::new(feature.node_index),
        )?
        .count as usize;
        let mut types = Vec::with_capacity(count);
        unsafe {
            types.set_len(count);
        }
        let mut tmp = D3D12_FEATURE_DATA_PROTECTED_RESOURCE_SESSION_TYPES {
            NodeIndex: feature.node_index,
            Count: count as u32,
            pTypes: types.as_mut_ptr(),
        };
        request_feature(
            device,
            D3D12_FEATURE_PROTECTED_RESOURCE_SESSION_TYPES,
            &mut tmp,
        )?;
        Ok(Self {
            node_index: tmp.NodeIndex,
            types,
        })
    }
}

#[derive(Debug)]
pub struct D3D12Options8 {
    pub unaligned_block_textures_supported: bool,
}

impl Feature for D3D12Options8 {
    fn check(device: &DeviceType) -> windows::core::Result<Self> {
        let tmp: D3D12_FEATURE_DATA_D3D12_OPTIONS8 =
            get_feature(device, D3D12_FEATURE_D3D12_OPTIONS8)?;
        Ok(Self {
            unaligned_block_textures_supported: tmp.UnalignedBlockTexturesSupported.into(),
        })
    }
}

#[derive(Debug)]
pub struct D3D12Options9 {
    pub mesh_shader_pipeline_stats_supported: bool,
    pub mesh_shader_supports_full_range_render_target_array_index: bool,
    pub atomic_int64_on_typed_resource_supported: bool,
    pub atomic_int64_on_group_shared_supported: bool,
    pub derivatives_in_mesh_and_amplification_shaders_supported: bool,
    pub wave_mma_tier: D3D12_WAVE_MMA_TIER,
}

impl Feature for D3D12Options9 {
    fn check(device: &DeviceType) -> windows::core::Result<Self> {
        let tmp: D3D12_FEATURE_DATA_D3D12_OPTIONS9 =
            get_feature(device, D3D12_FEATURE_D3D12_OPTIONS9)?;
        Ok(Self {
            mesh_shader_pipeline_stats_supported: tmp.MeshShaderPipelineStatsSupported.into(),
            mesh_shader_supports_full_range_render_target_array_index: tmp
                .MeshShaderSupportsFullRangeRenderTargetArrayIndex
                .into(),
            atomic_int64_on_typed_resource_supported: tmp
                .AtomicInt64OnTypedResourceSupported
                .into(),
            atomic_int64_on_group_shared_supported: tmp.AtomicInt64OnGroupSharedSupported.into(),
            derivatives_in_mesh_and_amplification_shaders_supported: tmp
                .DerivativesInMeshAndAmplificationShadersSupported
                .into(),
            wave_mma_tier: tmp.WaveMMATier,
        })
    }
}

#[derive(Debug)]
pub struct D3D12Options10 {
    pub variable_rate_shading_sum_combiner_supported: bool,
    pub mesh_shader_per_primitive_shading_rate_supported: bool,
}

impl Feature for D3D12Options10 {
    fn check(device: &DeviceType) -> windows::core::Result<Self> {
        let tmp: D3D12_FEATURE_DATA_D3D12_OPTIONS10 =
            get_feature(device, D3D12_FEATURE_D3D12_OPTIONS10)?;
        Ok(Self {
            variable_rate_shading_sum_combiner_supported: tmp
                .VariableRateShadingSumCombinerSupported
                .into(),
            mesh_shader_per_primitive_shading_rate_supported: tmp
                .MeshShaderPerPrimitiveShadingRateSupported
                .into(),
        })
    }
}

#[derive(Debug)]
pub struct D3D12Options11 {
    pub atomic_int64_on_descriptor_heap_resource_supported: bool,
}

impl Feature for D3D12Options11 {
    fn check(device: &DeviceType) -> windows::core::Result<Self> {
        let tmp: D3D12_FEATURE_DATA_D3D12_OPTIONS11 =
            get_feature(device, D3D12_FEATURE_D3D12_OPTIONS11)?;
        Ok(Self {
            atomic_int64_on_descriptor_heap_resource_supported: tmp
                .AtomicInt64OnDescriptorHeapResourceSupported
                .into(),
        })
    }
}

#[derive(Debug)]
pub struct D3D12Options12 {
    pub ms_primitives_pipeline_statistic_includes_culled_primitives: D3D12_TRI_STATE,
    pub enhanced_barriers_supported: bool,
    pub relaxed_format_casting_supported: bool,
}

impl Feature for D3D12Options12 {
    fn check(device: &DeviceType) -> windows::core::Result<Self> {
        let tmp: D3D12_FEATURE_DATA_D3D12_OPTIONS12 =
            get_feature(device, D3D12_FEATURE_D3D12_OPTIONS12)?;
        Ok(Self {
            ms_primitives_pipeline_statistic_includes_culled_primitives: tmp
                .MSPrimitivesPipelineStatisticIncludesCulledPrimitives,
            enhanced_barriers_supported: tmp.EnhancedBarriersSupported.into(),
            relaxed_format_casting_supported: tmp.RelaxedFormatCastingSupported.into(),
        })
    }
}

#[derive(Debug)]
pub struct D3D12Options13 {
    pub unrestricted_buffer_texture_copy_pitch_supported: bool,
    pub unrestricted_vertex_element_alignment_supported: bool,
    pub inverted_viewport_height_flips_y_supported: bool,
    pub inverted_viewport_depth_flips_z_supported: bool,
    pub texture_copy_between_dimensions_supported: bool,
    pub alpha_blend_factor_supported: bool,
}

impl Feature for D3D12Options13 {
    fn check(device: &DeviceType) -> windows::core::Result<Self> {
        let tmp: D3D12_FEATURE_DATA_D3D12_OPTIONS13 =
            get_feature(device, D3D12_FEATURE_D3D12_OPTIONS13)?;
        Ok(Self {
            unrestricted_buffer_texture_copy_pitch_supported: tmp
                .UnrestrictedBufferTextureCopyPitchSupported
                .into(),
            unrestricted_vertex_element_alignment_supported: tmp
                .UnrestrictedVertexElementAlignmentSupported
                .into(),
            inverted_viewport_height_flips_y_supported: tmp
                .InvertedViewportHeightFlipsYSupported
                .into(),
            inverted_viewport_depth_flips_z_supported: tmp
                .InvertedViewportDepthFlipsZSupported
                .into(),
            texture_copy_between_dimensions_supported: tmp
                .TextureCopyBetweenDimensionsSupported
                .into(),
            alpha_blend_factor_supported: tmp.AlphaBlendFactorSupported.into(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_device() -> crate::Device {
        let adapter = crate::enum_warp_adapter().unwrap();
        crate::Device::new()
            .adapter(&adapter)
            .min_feature_level(D3D_FEATURE_LEVEL_12_1)
            .build()
            .unwrap()
    }

    #[test]
    fn d3d12_options() {
        let device = create_device();
        device.check_feature::<D3D12Options>().unwrap();
    }

    #[test]
    fn architecture() {
        let device = create_device();
        device.request_feature(Architecture::new(0)).unwrap();
    }

    #[test]
    fn feature_levels() {
        let device = create_device();
        device
            .request_feature(FeatureLevels::new(&[
                D3D_FEATURE_LEVEL_12_0,
                D3D_FEATURE_LEVEL_12_1,
            ]))
            .unwrap();
    }

    #[test]
    fn format_support() {
        let device = create_device();
        device
            .request_feature(FormatSupport::new(DXGI_FORMAT_R32_FLOAT))
            .unwrap();
    }

    #[test]
    fn multisample_quality_levels() {
        let device = create_device();
        device
            .request_feature(MultisampleQualityLevels::new(DXGI_FORMAT_R8G8B8A8_UNORM, 1))
            .unwrap();
    }

    #[test]
    fn format_info() {
        let device = create_device();
        device
            .request_feature(FormatInfo::new(DXGI_FORMAT_R8G8B8A8_UNORM))
            .unwrap();
    }

    #[test]
    fn gpu_virtual_address_support() {
        let device = create_device();
        device.check_feature::<GpuVirtualAddressSupport>().unwrap();
    }

    #[test]
    fn shader_model() {
        let device = create_device();
        device.check_feature::<ShaderModel>().unwrap();
    }

    #[test]
    fn d3d12_options1() {
        let device = create_device();
        device.check_feature::<D3D12Options1>().unwrap();
    }

    #[test]
    fn protected_resource_session_support() {
        let device = create_device();
        device
            .request_feature(ProtectedResourceSessionSupport::new(0))
            .unwrap();
    }

    #[test]
    fn root_signature() {
        let device = create_device();
        device.check_feature::<RootSignature>().unwrap();
    }

    #[test]
    fn d3d12_options2() {
        let device = create_device();
        device.check_feature::<D3D12Options2>().unwrap();
    }

    #[test]
    fn shader_cache() {
        let device = create_device();
        device.check_feature::<ShaderCache>().unwrap();
    }

    #[test]
    fn command_queue_priority() {
        let device = create_device();
        device
            .request_feature(CommandQueuePriority::new(D3D12_COMMAND_LIST_TYPE_DIRECT, 0))
            .unwrap();
    }

    #[test]
    fn d3d12_options3() {
        let device = create_device();
        device.check_feature::<D3D12Options3>().unwrap();
    }

    #[test]
    fn existing_heaps() {
        let device = create_device();
        device.check_feature::<ExistingHeaps>().unwrap();
    }

    #[test]
    fn d3d12_options4() {
        let device = create_device();
        device.check_feature::<D3D12Options4>().unwrap();
    }

    #[test]
    fn serialization() {
        let device = create_device();
        device.request_feature(Serialization::new(0)).unwrap();
    }

    #[test]
    fn cross_node() {
        let device = create_device();
        device.check_feature::<CrossNode>().unwrap();
    }

    #[test]
    fn d3d12_options5() {
        let device = create_device();
        device.check_feature::<D3D12Options5>().unwrap();
    }

    #[test]
    fn displayable() {
        let device = create_device();
        device.check_feature::<Displayable>().unwrap();
    }

    #[test]
    fn d3d12_options6() {
        let device = create_device();
        device.check_feature::<D3D12Options6>().unwrap();
    }

    #[test]
    fn d3d12_options7() {
        let device = create_device();
        device.check_feature::<D3D12Options7>().unwrap();
    }

    #[test]
    fn protected_resource_session_types() {
        let device = create_device();
        device
            .request_feature(ProtectedResourceSessionTypes::new(0))
            .unwrap();
    }

    #[test]
    fn d3d12_options8() {
        let device = create_device();
        device.check_feature::<D3D12Options8>().unwrap();
    }

    #[test]
    fn d3d12_options9() {
        let device = create_device();
        device.check_feature::<D3D12Options9>().unwrap();
    }

    #[test]
    fn d3d12_options10() {
        let device = create_device();
        device.check_feature::<D3D12Options10>().unwrap();
    }

    #[test]
    fn d3d12_options11() {
        let device = create_device();
        device.check_feature::<D3D12Options11>().unwrap();
    }

    #[test]
    fn d3d12_options12() {
        let device = create_device();
        device.check_feature::<D3D12Options12>().unwrap();
    }

    #[test]
    fn d3d12_options13() {
        let device = create_device();
        device.check_feature::<D3D12Options13>().unwrap();
    }
}
