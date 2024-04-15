use super::*;
use windows::Win32::Graphics::Direct3D::*;
use windows::Win32::Graphics::Direct3D12::*;

#[repr(transparent)]
pub struct DescriptorRange(D3D12_DESCRIPTOR_RANGE);

impl DescriptorRange {
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
    pub fn offset_in_descriptors_from_table_start(mut self, offset: u32) -> Self {
        self.0.OffsetInDescriptorsFromTableStart = offset;
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

#[repr(transparent)]
pub struct RootParameter<'a, T> {
    param: D3D12_ROOT_PARAMETER,
    _t: std::marker::PhantomData<&'a T>,
}

impl<'a, T> RootParameter<'a, T>
where
    T: RootParameterType,
{
    #[inline]
    pub fn new(_t: T) -> Self {
        Self {
            param: D3D12_ROOT_PARAMETER {
                ParameterType: T::VALUE,
                ShaderVisibility: D3D12_SHADER_VISIBILITY_ALL,
                ..Default::default()
            },
            _t: std::marker::PhantomData,
        }
    }

    #[inline]
    pub fn shader_visibility(mut self, visibility: D3D12_SHADER_VISIBILITY) -> Self {
        self.param.ShaderVisibility = visibility;
        self
    }
}

impl<'a> RootParameter<'a, root_parameter_type::DescriptorTable> {
    #[inline]
    pub fn ranges(
        self,
        ranges: &[DescriptorRange],
    ) -> RootParameter<root_parameter_type::DescriptorTable> {
        RootParameter {
            param: D3D12_ROOT_PARAMETER {
                ParameterType: self.param.ParameterType,
                ShaderVisibility: self.param.ShaderVisibility,
                Anonymous: D3D12_ROOT_PARAMETER_0 {
                    DescriptorTable: D3D12_ROOT_DESCRIPTOR_TABLE {
                        NumDescriptorRanges: ranges.len() as u32,
                        pDescriptorRanges: ranges.as_ptr() as *const D3D12_DESCRIPTOR_RANGE,
                    },
                },
            },
            _t: std::marker::PhantomData,
        }
    }
}

impl<'a> RootParameter<'a, root_parameter_type::Constants32bit> {
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

impl<'a> RootParameter<'a, root_parameter_type::Cbv> {
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

impl<'a> RootParameter<'a, root_parameter_type::Srv> {
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

impl<'a> RootParameter<'a, root_parameter_type::Uav> {
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

impl<'a, T> From<RootParameter<'a, T>> for RootParameter<'a, ()>
where
    T: RootParameterType,
{
    fn from(value: RootParameter<'a, T>) -> Self {
        Self {
            param: value.param,
            _t: std::marker::PhantomData,
        }
    }
}

pub struct StaticSamplerDesc(D3D12_STATIC_SAMPLER_DESC);

impl StaticSamplerDesc {
    #[inline]
    pub fn new() -> Self {
        Self(D3D12_STATIC_SAMPLER_DESC::default())
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
}

#[repr(transparent)]
pub struct RootSignatureDesc<'params, 'samplers> {
    desc: D3D12_ROOT_SIGNATURE_DESC,
    _params: std::marker::PhantomData<&'params ()>,
    _samplers: std::marker::PhantomData<&'samplers ()>,
}

impl<'params, 'samplers> RootSignatureDesc<'params, 'samplers> {
    #[inline]
    pub fn new() -> Self {
        Self {
            desc: D3D12_ROOT_SIGNATURE_DESC::default(),
            _params: std::marker::PhantomData,
            _samplers: std::marker::PhantomData,
        }
    }

    #[inline]
    pub fn parameters<'a>(
        mut self,
        params: &'a [RootParameter<'_, ()>],
    ) -> RootSignatureDesc<'a, 'samplers> {
        self.desc.pParameters = params.as_ptr() as *const D3D12_ROOT_PARAMETER;
        self.desc.NumParameters = params.len() as u32;
        RootSignatureDesc {
            desc: self.desc,
            _params: std::marker::PhantomData,
            _samplers: std::marker::PhantomData,
        }
    }

    #[inline]
    pub fn flags(mut self, flags: D3D12_ROOT_SIGNATURE_FLAGS) -> Self {
        self.desc.Flags = flags;
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
        let blob = unsafe {
            let mut blob: Option<ID3DBlob> = None;
            D3D12SerializeRootSignature(&desc.desc, D3D_ROOT_SIGNATURE_VERSION_1, &mut blob, None)
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
}
