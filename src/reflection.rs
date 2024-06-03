use super::dxc::BlobType;
use super::*;
use windows::core::Interface;
use windows::Win32::Graphics::{Direct3D::Dxc::*, Direct3D::*, Direct3D12::*};

#[derive(Clone, Debug)]
pub struct SignatureParameterDesc {
    pub semantic_name: String,
    pub semantic_index: u32,
    pub system_value_type: D3D_NAME,
    pub component_type: D3D_REGISTER_COMPONENT_TYPE,
    pub mask: u8,
    pub read_write_mask: u8,
    pub stream: u32,
    pub min_precision: D3D_MIN_PRECISION,
}

#[derive(Clone, Debug)]
pub struct ShaderInputBindDesc {
    pub name: String,
    pub ty: D3D_SHADER_INPUT_TYPE,
    pub bind_point: u32,
    pub bind_count: u32,
    pub flags: D3D_SHADER_INPUT_FLAGS,
    pub return_type: D3D_RESOURCE_RETURN_TYPE,
    pub dimension: D3D_SRV_DIMENSION,
    pub num_samples: u32,
    pub space: u32,
    pub id: u32,
}

impl From<D3D12_SHADER_INPUT_BIND_DESC> for ShaderInputBindDesc {
    fn from(value: D3D12_SHADER_INPUT_BIND_DESC) -> Self {
        ShaderInputBindDesc {
            name: unsafe { value.Name.to_string().unwrap_or_else(|_| String::new()) },
            ty: value.Type,
            bind_count: value.BindCount,
            bind_point: value.BindPoint,
            flags: D3D_SHADER_INPUT_FLAGS(value.uFlags as i32),
            return_type: value.ReturnType,
            dimension: value.Dimension,
            num_samples: value.NumSamples,
            space: value.Space,
            id: value.uID,
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum MatrixMajor {
    Columns,
    Rows,
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum Type {
    Void,
    Bool,
    Int,
    Uint,
    Float,
    String,
    Texture,
    Texture1D,
    Texture2D,
    Texture3D,
    TextureCube,
    Sampler,
    Sampler1D,
    Sampler2D,
    Sampler3D,
    SamplerCube,
    Min8Float,
    Min10Float,
    Min16Int,
    Min12Int,
    Min16Uint,
    Vector(Vector),
    Matrix(Matrix),
    Struct(Struct),
    Unsupported,
}

impl Type {
    fn svt(t: D3D_SHADER_VARIABLE_TYPE) -> Self {
        match t {
            D3D_SVT_VOID => Self::Void,
            D3D_SVT_BOOL => Self::Bool,
            D3D_SVT_INT => Self::Int,
            D3D_SVT_UINT => Self::Uint,
            D3D_SVT_FLOAT => Self::Float,
            D3D_SVT_STRING => Self::String,
            D3D_SVT_MIN8FLOAT => Self::Min8Float,
            D3D_SVT_MIN10FLOAT => Self::Min10Float,
            D3D_SVT_MIN16INT => Self::Min16Int,
            D3D_SVT_MIN12INT => Self::Min12Int,
            D3D_SVT_MIN16UINT => Self::Min16Uint,
            _ => Self::Unsupported,
        }
    }

    fn from_reflection(t: &ID3D12ShaderReflectionType) -> windows::core::Result<Self> {
        unsafe {
            let mut desc = D3D12_SHADER_TYPE_DESC::default();
            t.GetDesc(&mut desc)?;
            let ret = match desc.Class {
                D3D_SVC_SCALAR => Self::svt(desc.Type),
                D3D_SVC_VECTOR => Self::Vector(Vector {
                    ty: Box::new(Self::svt(desc.Type)),
                    len: desc.Columns as usize,
                }),
                D3D_SVC_MATRIX_COLUMNS => Self::Matrix(Matrix {
                    ty: Box::new(Self::svt(desc.Type)),
                    columns: desc.Columns as usize,
                    rows: desc.Rows as usize,
                    major: MatrixMajor::Columns,
                }),
                D3D_SVC_MATRIX_ROWS => Self::Matrix(Matrix {
                    ty: Box::new(Self::svt(desc.Type)),
                    columns: desc.Columns as usize,
                    rows: desc.Rows as usize,
                    major: MatrixMajor::Rows,
                }),
                D3D_SVC_STRUCT => Self::Struct(Struct {
                    name: desc.Name.to_string()?,
                    members: (0..desc.Members)
                        .map(|i| -> windows::core::Result<Member> {
                            let u = t.GetMemberTypeByIndex(i).unwrap();
                            let ty = Self::from_reflection(&u)?;
                            let name = t.GetMemberTypeName(i).to_string()?;
                            let mut d = D3D12_SHADER_TYPE_DESC::default();
                            u.GetDesc(&mut d)?;
                            Ok(Member {
                                ty,
                                name,
                                offset: d.Offset as usize,
                            })
                        })
                        .collect::<windows::core::Result<Vec<Member>>>()?,
                }),
                _ => Self::Unsupported,
            };
            Ok(ret)
        }
    }

    fn from_parameter_desc(desc: &D3D12_PARAMETER_DESC) -> windows::core::Result<Self> {
        unsafe {
            let ret = match desc.Class {
                D3D_SVC_SCALAR => Self::svt(desc.Type),
                D3D_SVC_VECTOR => Self::Vector(Vector {
                    ty: Box::new(Self::svt(desc.Type)),
                    len: desc.Columns as usize,
                }),
                D3D_SVC_MATRIX_COLUMNS => Self::Matrix(Matrix {
                    ty: Box::new(Self::svt(desc.Type)),
                    columns: desc.Columns as usize,
                    rows: desc.Rows as usize,
                    major: MatrixMajor::Columns,
                }),
                D3D_SVC_MATRIX_ROWS => Self::Matrix(Matrix {
                    ty: Box::new(Self::svt(desc.Type)),
                    columns: desc.Columns as usize,
                    rows: desc.Rows as usize,
                    major: MatrixMajor::Rows,
                }),
                D3D_SVC_STRUCT => Self::Struct(Struct {
                    name: "".into(),
                    members: vec![],
                }),
                _ => Self::Unsupported,
            };
            Ok(ret)
        }
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Vector {
    pub ty: Box<Type>,
    pub len: usize,
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Matrix {
    pub ty: Box<Type>,
    pub columns: usize,
    pub rows: usize,
    pub major: MatrixMajor,
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Member {
    pub name: String,
    pub ty: Type,
    pub offset: usize,
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Struct {
    pub name: String,
    pub members: Vec<Member>,
}

#[derive(Clone, Debug)]
pub struct ShaderBufferDesc {
    pub name: String,
    pub members: Vec<Type>,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct ThreadGroupSize {
    pub x: u32,
    pub y: u32,
    pub z: u32,
}

impl ThreadGroupSize {
    #[inline]
    pub fn total_size(&self) -> u32 {
        self.x * self.y * self.z
    }
}

pub trait ReflectionType {
    type Type: windows::core::Interface;

    fn new(this: Self::Type) -> Self;
}

#[derive(Clone, PartialEq, Eq, Debug)]
#[repr(transparent)]
pub struct ShaderReflection(pub(crate) ID3D12ShaderReflection);

impl ShaderReflection {
    #[inline]
    pub fn new(utils: &dxc::Utils, data: &[u8], encoding: DXC_CP) -> windows::core::Result<Self> {
        let buffer = DxcBuffer {
            Ptr: data.as_ptr() as *const std::ffi::c_void,
            Size: data.len(),
            Encoding: encoding.0,
        };
        Ok(Self(unsafe {
            let mut p: Option<ID3D12ShaderReflection> = None;
            utils
                .handle()
                .CreateReflection(
                    &buffer,
                    &ID3D12ShaderReflection::IID,
                    &mut p as *mut _ as *mut _,
                )
                .map(|_| p.unwrap())?
        }))
    }

    #[inline]
    pub fn from_compile_result(
        utils: &dxc::Utils,
        result: &dxc::CompileResult,
    ) -> windows::core::Result<Self> {
        result
            .get_blob(DXC_OUT_REFLECTION)
            .and_then(|blob| Self::new(utils, blob.as_slice(), DXC_CP_UTF8))
    }

    fn get_desc(&self) -> windows::core::Result<D3D12_SHADER_DESC> {
        unsafe {
            let mut desc = D3D12_SHADER_DESC::default();
            self.0.GetDesc(&mut desc)?;
            Ok(desc)
        }
    }

    #[inline]
    pub fn min_feature_level(&self) -> windows::core::Result<D3D_FEATURE_LEVEL> {
        unsafe { self.0.GetMinFeatureLevel() }
    }

    #[inline]
    pub fn thread_group_size(&self) -> ThreadGroupSize {
        let mut ret = ThreadGroupSize { x: 0, y: 0, z: 0 };
        unsafe {
            self.0
                .GetThreadGroupSize(Some(&mut ret.x), Some(&mut ret.y), Some(&mut ret.z));
        }
        ret
    }

    #[inline]
    pub fn input_parameters(&self) -> windows::core::Result<Vec<SignatureParameterDesc>> {
        let desc = self.get_desc()?;
        unsafe {
            (0..desc.InputParameters)
                .map(|i| -> windows::core::Result<SignatureParameterDesc> {
                    let mut ipd = D3D12_SIGNATURE_PARAMETER_DESC::default();
                    self.0.GetInputParameterDesc(i, &mut ipd)?;
                    Ok(SignatureParameterDesc {
                        semantic_name: ipd.SemanticName.to_string()?,
                        semantic_index: ipd.SemanticIndex,
                        system_value_type: ipd.SystemValueType,
                        component_type: ipd.ComponentType,
                        mask: ipd.Mask,
                        read_write_mask: ipd.ReadWriteMask,
                        stream: ipd.Stream,
                        min_precision: ipd.MinPrecision,
                    })
                })
                .collect()
        }
    }

    #[inline]
    pub fn resource_bind_desc(&self) -> windows::core::Result<Vec<ShaderInputBindDesc>> {
        let desc = self.get_desc()?;
        unsafe {
            (0..desc.BoundResources)
                .map(|i| -> windows::core::Result<ShaderInputBindDesc> {
                    let mut sibd = D3D12_SHADER_INPUT_BIND_DESC::default();
                    self.0.GetResourceBindingDesc(i, &mut sibd)?;
                    Ok(sibd.into())
                })
                .collect()
        }
    }

    #[inline]
    pub fn constant_buffer_descs(&self) -> windows::core::Result<Vec<ShaderBufferDesc>> {
        let desc = self.get_desc()?;
        unsafe {
            (0..desc.ConstantBuffers)
                .map(|i| -> windows::core::Result<ShaderBufferDesc> {
                    let cb = self.0.GetConstantBufferByIndex(i).unwrap();
                    let mut cb_desc = D3D12_SHADER_BUFFER_DESC::default();
                    cb.GetDesc(&mut cb_desc)?;
                    Ok(ShaderBufferDesc {
                        name: cb_desc.Name.to_string()?,
                        members: (0..cb_desc.Variables)
                            .map(|i| -> windows::core::Result<Type> {
                                let v = cb.GetVariableByIndex(i).unwrap();
                                let mut vd = D3D12_SHADER_VARIABLE_DESC::default();
                                v.GetDesc(&mut vd)?;
                                let t = v.GetType().unwrap();
                                Type::from_reflection(&t)
                            })
                            .collect::<windows::core::Result<Vec<Type>>>()?,
                    })
                })
                .collect()
        }
    }

    #[inline]
    pub fn output_parameter_descs(&self) -> windows::core::Result<Vec<SignatureParameterDesc>> {
        let desc = self.get_desc()?;
        unsafe {
            (0..desc.OutputParameters)
                .map(|i| -> windows::core::Result<SignatureParameterDesc> {
                    let mut spd = D3D12_SIGNATURE_PARAMETER_DESC::default();
                    self.0.GetOutputParameterDesc(i, &mut spd)?;
                    Ok(SignatureParameterDesc {
                        semantic_name: spd.SemanticName.to_string()?,
                        semantic_index: spd.SemanticIndex,
                        system_value_type: spd.SystemValueType,
                        component_type: spd.ComponentType,
                        mask: spd.Mask,
                        read_write_mask: spd.ReadWriteMask,
                        stream: spd.Stream,
                        min_precision: spd.MinPrecision,
                    })
                })
                .collect()
        }
    }
}

impl ReflectionType for ShaderReflection {
    type Type = ID3D12ShaderReflection;

    fn new(this: Self::Type) -> Self {
        Self(this)
    }
}

fn demangle(src: &str) -> &str {
    let s = src.strip_prefix("\u{1}?").unwrap();
    let end = s.find("@").unwrap();
    &s[..end]
}

#[derive(Clone, Debug)]
pub struct FunctionParameter {
    pub name: String,
    pub semantic_name: String,
    pub ty: Type,
}

#[derive(Clone, Debug)]
pub struct Function {
    object: ID3D12FunctionReflection,
    name: String,
}

impl Function {
    #[inline]
    pub fn name(&self) -> &str {
        &self.name
    }

    fn get_desc(&self) -> D3D12_FUNCTION_DESC {
        unsafe {
            let mut desc = D3D12_FUNCTION_DESC::default();
            self.object.GetDesc(&mut desc).unwrap();
            desc
        }
    }

    #[inline]
    pub fn parameters(&self) -> impl Iterator<Item = FunctionParameter> + '_ {
        let desc = self.get_desc();
        unsafe {
            (0..desc.FunctionParameterCount)
                .map(|i| -> windows::core::Result<FunctionParameter> {
                    let param = self.object.GetFunctionParameter(i).unwrap();
                    let mut desc = D3D12_PARAMETER_DESC::default();
                    param.GetDesc(&mut desc)?;
                    Ok(FunctionParameter {
                        name: desc.Name.to_string()?,
                        semantic_name: desc.SemanticName.to_string()?,
                        ty: Type::from_parameter_desc(&desc)?,
                    })
                })
                .filter_map(|param| param.ok())
        }
    }

    #[inline]
    pub fn resource_bind_descs(&self) -> impl Iterator<Item = ShaderInputBindDesc> + '_ {
        let desc = self.get_desc();
        unsafe {
            (0..desc.BoundResources)
                .map(|i| -> windows::core::Result<ShaderInputBindDesc> {
                    let mut sibd = D3D12_SHADER_INPUT_BIND_DESC::default();
                    self.object.GetResourceBindingDesc(i, &mut sibd)?;
                    Ok(sibd.into())
                })
                .filter_map(|param| param.ok())
        }
    }

    #[inline]
    pub fn constant_buffer_descs(&self) -> impl Iterator<Item = ShaderBufferDesc> + '_ {
        let desc = self.get_desc();
        unsafe {
            (0..desc.ConstantBuffers)
                .map(|i| -> windows::core::Result<ShaderBufferDesc> {
                    let cb = self.object.GetConstantBufferByIndex(i).unwrap();
                    let mut cb_desc = D3D12_SHADER_BUFFER_DESC::default();
                    cb.GetDesc(&mut cb_desc).unwrap();
                    Ok(ShaderBufferDesc {
                        name: cb_desc.Name.to_string()?,
                        members: (0..cb_desc.Variables)
                            .map(|i| -> windows::core::Result<Type> {
                                let v = cb.GetVariableByIndex(i).unwrap();
                                let mut vd = D3D12_SHADER_VARIABLE_DESC::default();
                                v.GetDesc(&mut vd)?;
                                let t = v.GetType().unwrap();
                                Type::from_reflection(&t)
                            })
                            .collect::<windows::core::Result<Vec<Type>>>()?,
                    })
                })
                .filter_map(|desc| desc.ok())
        }
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
#[repr(transparent)]
pub struct LibraryReflection(pub(crate) ID3D12LibraryReflection);

impl LibraryReflection {
    #[inline]
    pub fn new(utils: &dxc::Utils, data: &[u8], encoding: DXC_CP) -> windows::core::Result<Self> {
        let buffer = DxcBuffer {
            Ptr: data.as_ptr() as *const std::ffi::c_void,
            Size: data.len(),
            Encoding: encoding.0,
        };
        Ok(Self(unsafe {
            let mut p: Option<ID3D12LibraryReflection> = None;
            utils
                .handle()
                .CreateReflection(
                    &buffer,
                    &ID3D12ShaderReflection::IID,
                    &mut p as *mut _ as *mut _,
                )
                .map(|_| p.unwrap())?
        }))
    }

    fn get_desc(&self) -> windows::core::Result<D3D12_LIBRARY_DESC> {
        unsafe { self.0.GetDesc() }
    }

    #[inline]
    pub fn functions(&self) -> impl Iterator<Item = Function> + '_ {
        let desc = self.get_desc().unwrap();
        unsafe {
            (0..desc.FunctionCount)
                .map(|i| -> windows::core::Result<Function> {
                    let f = self.0.GetFunctionByIndex(i as i32).unwrap();
                    let desc = {
                        let mut desc = D3D12_FUNCTION_DESC::default();
                        f.GetDesc(&mut desc)?;
                        desc
                    };
                    Ok(Function {
                        object: f,
                        name: demangle(&desc.Name.to_string().unwrap()).to_string(),
                    })
                })
                .filter_map(|f| f.ok())
        }
    }
}

impl ReflectionType for LibraryReflection {
    type Type = ID3D12LibraryReflection;

    fn new(this: Self::Type) -> Self {
        Self(this)
    }
}
