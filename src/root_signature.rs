use super::*;
use windows::Win32::Graphics::Direct3D::*;
use windows::Win32::Graphics::Direct3D12::*;

#[derive(Clone, Debug)]
#[repr(transparent)]
pub struct DescriptorRange(D3D12_DESCRIPTOR_RANGE);

impl DescriptorRange {
    #[inline]
    pub fn new(t: D3D12_DESCRIPTOR_RANGE_TYPE) -> Self {
        Self(D3D12_DESCRIPTOR_RANGE {
            RangeType: t,
            OffsetInDescriptorsFromTableStart: D3D12_DESCRIPTOR_RANGE_OFFSET_APPEND,
            ..Default::default()
        })
    }

    #[inline]
    pub fn cbv() -> Self {
        Self(D3D12_DESCRIPTOR_RANGE {
            RangeType: D3D12_DESCRIPTOR_RANGE_TYPE_CBV,
            OffsetInDescriptorsFromTableStart: D3D12_DESCRIPTOR_RANGE_OFFSET_APPEND,
            ..Default::default()
        })
    }

    #[inline]
    pub fn srv() -> Self {
        Self(D3D12_DESCRIPTOR_RANGE {
            RangeType: D3D12_DESCRIPTOR_RANGE_TYPE_SRV,
            OffsetInDescriptorsFromTableStart: D3D12_DESCRIPTOR_RANGE_OFFSET_APPEND,
            ..Default::default()
        })
    }

    #[inline]
    pub fn uav() -> Self {
        Self(D3D12_DESCRIPTOR_RANGE {
            RangeType: D3D12_DESCRIPTOR_RANGE_TYPE_UAV,
            OffsetInDescriptorsFromTableStart: D3D12_DESCRIPTOR_RANGE_OFFSET_APPEND,
            ..Default::default()
        })
    }

    #[inline]
    pub fn sampler() -> Self {
        Self(D3D12_DESCRIPTOR_RANGE {
            RangeType: D3D12_DESCRIPTOR_RANGE_TYPE_SAMPLER,
            OffsetInDescriptorsFromTableStart: D3D12_DESCRIPTOR_RANGE_OFFSET_APPEND,
            ..Default::default()
        })
    }

    #[inline]
    pub fn num_descriptors(mut self, n: u32) -> Self {
        self.0.NumDescriptors = n;
        self
    }

    #[inline]
    pub fn base_shader_register(mut self, reg: u32) -> Self {
        self.0.BaseShaderRegister = reg;
        self
    }

    #[inline]
    pub fn register_space(mut self, space: u32) -> Self {
        self.0.RegisterSpace = space;
        self
    }

    #[inline]
    pub fn offset_in_descriptors_from_table_start(mut self, offset: Option<u32>) -> Self {
        self.0.OffsetInDescriptorsFromTableStart =
            offset.unwrap_or(D3D12_DESCRIPTOR_RANGE_OFFSET_APPEND);
        self
    }
}

pub trait RootParameterType {
    const VALUE: D3D12_ROOT_PARAMETER_TYPE;
}

pub mod root_parameter_type {
    use super::*;

    pub struct DescriptorTable;

    impl RootParameterType for DescriptorTable {
        const VALUE: D3D12_ROOT_PARAMETER_TYPE = D3D12_ROOT_PARAMETER_TYPE_DESCRIPTOR_TABLE;
    }

    pub struct Constants32bit;

    impl RootParameterType for Constants32bit {
        const VALUE: D3D12_ROOT_PARAMETER_TYPE = D3D12_ROOT_PARAMETER_TYPE_32BIT_CONSTANTS;
    }

    pub struct Cbv;

    impl RootParameterType for Cbv {
        const VALUE: D3D12_ROOT_PARAMETER_TYPE = D3D12_ROOT_PARAMETER_TYPE_CBV;
    }

    pub struct Srv;

    impl RootParameterType for Srv {
        const VALUE: D3D12_ROOT_PARAMETER_TYPE = D3D12_ROOT_PARAMETER_TYPE_SRV;
    }

    pub struct Uav;

    impl RootParameterType for Uav {
        const VALUE: D3D12_ROOT_PARAMETER_TYPE = D3D12_ROOT_PARAMETER_TYPE_UAV;
    }
}

#[derive(Clone)]
pub struct RootParameter<T = ()> {
    param: D3D12_ROOT_PARAMETER,
    ranges: Option<Vec<D3D12_DESCRIPTOR_RANGE>>,
    _t: std::marker::PhantomData<T>,
}

impl RootParameter<()> {
    #[inline]
    pub fn new<T>(_t: T) -> RootParameter<T>
    where
        T: RootParameterType,
    {
        RootParameter {
            param: D3D12_ROOT_PARAMETER {
                ParameterType: T::VALUE,
                ShaderVisibility: D3D12_SHADER_VISIBILITY_ALL,
                ..Default::default()
            },
            ranges: None,
            _t: std::marker::PhantomData,
        }
    }

    #[inline]
    pub fn cbv() -> RootParameter<root_parameter_type::Cbv> {
        Self::new(root_parameter_type::Cbv)
    }

    #[inline]
    pub fn srv() -> RootParameter<root_parameter_type::Srv> {
        Self::new(root_parameter_type::Srv)
    }

    #[inline]
    pub fn uav() -> RootParameter<root_parameter_type::Uav> {
        Self::new(root_parameter_type::Uav)
    }

    #[inline]
    pub fn constants_32bit() -> RootParameter<root_parameter_type::Constants32bit> {
        Self::new(root_parameter_type::Constants32bit)
    }

    #[inline]
    pub fn descriptor_table() -> RootParameter<root_parameter_type::DescriptorTable> {
        Self::new(root_parameter_type::DescriptorTable)
    }
}

impl<T> RootParameter<T>
where
    T: RootParameterType,
{
    #[inline]
    pub fn shader_visibility(mut self, visibility: D3D12_SHADER_VISIBILITY) -> Self {
        self.param.ShaderVisibility = visibility;
        self
    }
}

impl RootParameter<root_parameter_type::DescriptorTable> {
    #[inline]
    pub fn ranges(
        self,
        ranges: impl IntoIterator<Item = DescriptorRange>,
    ) -> RootParameter<root_parameter_type::DescriptorTable> {
        let ranges = ranges
            .into_iter()
            .map(|range| range.0)
            .collect::<Vec<D3D12_DESCRIPTOR_RANGE>>();
        RootParameter {
            param: D3D12_ROOT_PARAMETER {
                ParameterType: self.param.ParameterType,
                ShaderVisibility: self.param.ShaderVisibility,
                Anonymous: D3D12_ROOT_PARAMETER_0 {
                    DescriptorTable: D3D12_ROOT_DESCRIPTOR_TABLE {
                        NumDescriptorRanges: ranges.len() as u32,
                        pDescriptorRanges: ranges.as_ptr(),
                    },
                },
            },
            ranges: Some(ranges),
            _t: std::marker::PhantomData,
        }
    }
}

impl RootParameter<root_parameter_type::Constants32bit> {
    #[inline]
    pub fn shader_register(mut self, reg: u32) -> Self {
        self.param.Anonymous.Constants.ShaderRegister = reg;
        self
    }

    #[inline]
    pub fn register_space(mut self, space: u32) -> Self {
        self.param.Anonymous.Constants.RegisterSpace = space;
        self
    }

    #[inline]
    pub fn num_32bit_values(mut self, n: u32) -> Self {
        self.param.Anonymous.Constants.Num32BitValues = n;
        self
    }
}

impl RootParameter<root_parameter_type::Cbv> {
    #[inline]
    pub fn shader_register(mut self, reg: u32) -> Self {
        self.param.Anonymous.Descriptor.ShaderRegister = reg;
        self
    }

    #[inline]
    pub fn register_space(mut self, space: u32) -> Self {
        self.param.Anonymous.Descriptor.RegisterSpace = space;
        self
    }
}

impl RootParameter<root_parameter_type::Srv> {
    #[inline]
    pub fn shader_register(mut self, reg: u32) -> Self {
        self.param.Anonymous.Descriptor.ShaderRegister = reg;
        self
    }

    #[inline]
    pub fn register_space(mut self, space: u32) -> Self {
        self.param.Anonymous.Descriptor.RegisterSpace = space;
        self
    }
}

impl RootParameter<root_parameter_type::Uav> {
    #[inline]
    pub fn shader_register(mut self, reg: u32) -> Self {
        self.param.Anonymous.Descriptor.ShaderRegister = reg;
        self
    }

    #[inline]
    pub fn register_space(mut self, space: u32) -> Self {
        self.param.Anonymous.Descriptor.RegisterSpace = space;
        self
    }
}

impl<T> From<RootParameter<T>> for RootParameter<()>
where
    T: RootParameterType,
{
    fn from(value: RootParameter<T>) -> Self {
        Self {
            param: value.param,
            ranges: value.ranges,
            _t: std::marker::PhantomData,
        }
    }
}

impl<T> std::fmt::Debug for RootParameter<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "RootParameter {{ {:?} .. }}", self.ranges)
    }
}

#[derive(Clone, Debug)]
#[repr(transparent)]
pub struct StaticSamplerDesc(D3D12_STATIC_SAMPLER_DESC);

impl StaticSamplerDesc {
    #[inline]
    pub fn new() -> Self {
        Self(D3D12_STATIC_SAMPLER_DESC {
            Filter: D3D12_FILTER_ANISOTROPIC,
            AddressU: D3D12_TEXTURE_ADDRESS_MODE_WRAP,
            AddressV: D3D12_TEXTURE_ADDRESS_MODE_WRAP,
            AddressW: D3D12_TEXTURE_ADDRESS_MODE_WRAP,
            MaxAnisotropy: 16,
            ComparisonFunc: D3D12_COMPARISON_FUNC_LESS_EQUAL,
            BorderColor: D3D12_STATIC_BORDER_COLOR_OPAQUE_BLACK,
            MaxLOD: f32::MAX,
            ShaderVisibility: D3D12_SHADER_VISIBILITY_ALL,
            ..Default::default()
        })
    }

    #[inline]
    pub fn filter(mut self, filter: D3D12_FILTER) -> Self {
        self.0.Filter = filter;
        self
    }

    #[inline]
    pub fn address_u(mut self, mode: D3D12_TEXTURE_ADDRESS_MODE) -> Self {
        self.0.AddressU = mode;
        self
    }

    #[inline]
    pub fn address_v(mut self, mode: D3D12_TEXTURE_ADDRESS_MODE) -> Self {
        self.0.AddressV = mode;
        self
    }

    #[inline]
    pub fn address_w(mut self, mode: D3D12_TEXTURE_ADDRESS_MODE) -> Self {
        self.0.AddressW = mode;
        self
    }

    #[inline]
    pub fn mip_lod_bias(mut self, value: f32) -> Self {
        self.0.MipLODBias = value;
        self
    }

    #[inline]
    pub fn max_anisotropy(mut self, value: u32) -> Self {
        self.0.MaxAnisotropy = value;
        self
    }

    #[inline]
    pub fn comparison_func(mut self, func: D3D12_COMPARISON_FUNC) -> Self {
        self.0.ComparisonFunc = func;
        self
    }

    #[inline]
    pub fn border_color(mut self, color: D3D12_STATIC_BORDER_COLOR) -> Self {
        self.0.BorderColor = color;
        self
    }

    #[inline]
    pub fn min_lod(mut self, lod: f32) -> Self {
        self.0.MinLOD = lod;
        self
    }

    #[inline]
    pub fn max_lod(mut self, lod: f32) -> Self {
        self.0.MaxLOD = lod;
        self
    }

    #[inline]
    pub fn shader_register(mut self, reg: u32) -> Self {
        self.0.ShaderRegister = reg;
        self
    }

    #[inline]
    pub fn register_space(mut self, space: u32) -> Self {
        self.0.RegisterSpace = space;
        self
    }

    #[inline]
    pub fn shader_visibility(mut self, visibility: D3D12_SHADER_VISIBILITY) -> Self {
        self.0.ShaderVisibility = visibility;
        self
    }
}

impl Default for StaticSamplerDesc {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Clone)]
pub struct RootSignatureDesc<'params, 'samplers> {
    params: Option<&'params [RootParameter<()>]>,
    samplers: Option<&'samplers [StaticSamplerDesc]>,
    flags: D3D12_ROOT_SIGNATURE_FLAGS,
}

impl<'params, 'samplers> RootSignatureDesc<'params, 'samplers> {
    #[inline]
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self {
            params: None,
            samplers: None,
            flags: D3D12_ROOT_SIGNATURE_FLAG_NONE,
        }
    }

    #[inline]
    pub fn parameters<'a>(
        self,
        params: &'a [RootParameter<()>],
    ) -> RootSignatureDesc<'a, 'samplers> {
        RootSignatureDesc {
            params: Some(params),
            samplers: self.samplers,
            flags: self.flags,
        }
    }

    #[inline]
    pub fn static_samplers<'b>(
        self,
        samplers: &'b [StaticSamplerDesc],
    ) -> RootSignatureDesc<'params, 'b> {
        RootSignatureDesc {
            params: self.params,
            samplers: Some(samplers),
            flags: self.flags,
        }
    }

    #[inline]
    pub fn flags(mut self, flags: D3D12_ROOT_SIGNATURE_FLAGS) -> Self {
        self.flags = flags;
        self
    }
}

pub struct Builder {
    device: ID3D12Device,
    node_mask: u32,
    name: Option<String>,
}

impl Builder {
    fn new<T>(device: &T) -> Self
    where
        T: Into<ID3D12Device> + Clone,
    {
        let device: ID3D12Device = device.clone().into();
        Self {
            device,
            node_mask: 0,
            name: None,
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
    pub fn build_from_desc(self, desc: &RootSignatureDesc) -> windows::core::Result<RootSignature> {
        let parameters = desc
            .params
            .as_ref()
            .map(|params| params.iter().map(|p| p.param).collect::<Vec<_>>())
            .unwrap_or_default();
        let samplers = desc
            .samplers
            .as_ref()
            .map(|samplers| samplers.to_vec())
            .unwrap_or_default();
        let desc = D3D12_ROOT_SIGNATURE_DESC {
            NumParameters: parameters.len() as u32,
            pParameters: parameters.as_ptr(),
            NumStaticSamplers: samplers.len() as u32,
            pStaticSamplers: samplers.as_ptr() as *const D3D12_STATIC_SAMPLER_DESC,
            Flags: desc.flags,
        };
        let blob = unsafe {
            let mut blob: Option<ID3DBlob> = None;
            D3D12SerializeRootSignature(&desc, D3D_ROOT_SIGNATURE_VERSION_1, &mut blob, None)
                .map(|_| blob.unwrap())?
        };
        let handle = unsafe {
            let data = std::slice::from_raw_parts(
                blob.GetBufferPointer() as *const u8,
                blob.GetBufferSize(),
            );
            self.device.CreateRootSignature(self.node_mask, data)?
        };
        let name = self.name.as_ref().map(|n| Name::new(&handle, n));
        Ok(RootSignature { handle, name })
    }
}

#[derive(Clone, Debug)]
pub struct RootSignature {
    handle: ID3D12RootSignature,
    name: Option<Name>,
}

impl RootSignature {
    #[inline]
    #[allow(clippy::new_ret_no_self)]
    pub fn new(device: &Device) -> Builder {
        Builder::new(device.handle())
    }

    #[inline]
    pub fn handle(&self) -> &ID3D12RootSignature {
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
