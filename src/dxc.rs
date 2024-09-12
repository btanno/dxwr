use super::*;
use std::path::Path;
use windows::core::{Interface, HSTRING, PCWSTR};
use windows::Win32::Foundation::{E_ABORT, E_POINTER};

pub use windows::Win32::Graphics::Direct3D::Dxc::*;

#[derive(Clone, PartialEq, Eq, Debug)]
#[repr(transparent)]
pub struct Utils(IDxcUtils);

impl Utils {
    #[inline]
    pub fn new() -> windows::core::Result<Self> {
        unsafe { Ok(Self(DxcCreateInstance(&CLSID_DxcUtils)?)) }
    }

    #[inline]
    pub fn handle(&self) -> &IDxcUtils {
        &self.0
    }
}

pub trait BlobType: Sized {
    fn handle(&self) -> IDxcBlob;

    fn as_ptr(&self) -> *const std::ffi::c_void {
        unsafe { self.handle().GetBufferPointer() }
    }

    fn is_empty(&self) -> bool {
        self.len() == 0
    }

    fn len(&self) -> usize {
        unsafe { self.handle().GetBufferSize() }
    }

    fn as_slice(&self) -> &[u8] {
        unsafe { std::slice::from_raw_parts(self.as_ptr() as *const u8, self.len()) }
    }

    unsafe fn as_ref<T>(&self) -> &T {
        unsafe { (self.as_ptr() as *const T).as_ref().unwrap() }
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
#[repr(transparent)]
pub struct RefBlob<'a> {
    blob: IDxcBlob,
    _a: std::marker::PhantomData<&'a ()>,
}

impl<'a> RefBlob<'a> {
    #[inline]
    pub fn new(util: &Utils, data: &[u8], encoding: DXC_CP) -> windows::core::Result<Self> {
        unsafe {
            Ok(Self {
                blob: util
                    .0
                    .CreateBlobFromPinned(
                        data.as_ptr() as *const std::ffi::c_void,
                        data.len() as u32,
                        encoding,
                    )?
                    .cast()?,
                _a: std::marker::PhantomData,
            })
        }
    }
}

impl<'a> BlobType for RefBlob<'a> {
    fn handle(&self) -> IDxcBlob {
        self.blob.clone()
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
#[repr(transparent)]
pub struct Blob(IDxcBlob);

impl BlobType for Blob {
    fn handle(&self) -> IDxcBlob {
        self.0.clone()
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
#[repr(C)]
pub struct ShaderHash {
    pub flags: u32,
    pub digest: [u8; 16],
}

impl std::hash::Hash for ShaderHash {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        state.write(&self.digest)
    }
}

#[repr(transparent)]
pub struct CompileResult(IDxcResult);

impl CompileResult {
    fn get_output<T>(&self, kind: DXC_OUT_KIND) -> windows::core::Result<T>
    where
        T: windows::core::Interface,
    {
        if !self.has_output(kind) {
            return Err(E_ABORT.into());
        }
        let output = unsafe {
            let mut p: Option<T> = None;
            self.0
                .GetOutput(kind, std::ptr::null_mut(), &mut p)
                .map(|_| p.unwrap())?
        };
        Ok(output)
    }

    pub(crate) fn get_blob(&self, kind: DXC_OUT_KIND) -> windows::core::Result<RefBlob> {
        let blob = self.get_output::<IDxcBlob>(kind)?;
        unsafe {
            if blob.GetBufferPointer().is_null() {
                return Err(E_POINTER.into());
            }
        }
        Ok(RefBlob {
            blob,
            _a: std::marker::PhantomData,
        })
    }

    fn get_string(&self, kind: DXC_OUT_KIND) -> windows::core::Result<String> {
        let blob = self.get_output::<IDxcBlobUtf8>(kind)?;
        unsafe {
            if blob.GetBufferPointer().is_null() {
                return Ok(String::new());
            }
            Ok(String::from_utf8_lossy(std::slice::from_raw_parts(
                blob.GetBufferPointer() as *const u8,
                blob.GetBufferSize(),
            ))
            .to_string())
        }
    }

    #[inline]
    pub fn has_output(&self, kind: DXC_OUT_KIND) -> bool {
        unsafe { self.0.HasOutput(kind).as_bool() }
    }

    #[inline]
    pub fn object(&self) -> windows::core::Result<RefBlob> {
        self.get_blob(DXC_OUT_OBJECT)
    }

    #[inline]
    pub fn errors(&self) -> windows::core::Result<String> {
        self.get_string(DXC_OUT_ERRORS)
    }

    #[inline]
    pub fn pdb(&self) -> windows::core::Result<RefBlob> {
        self.get_blob(DXC_OUT_PDB)
    }

    #[inline]
    pub fn shader_hash(&self) -> windows::core::Result<ShaderHash> {
        self.get_blob(DXC_OUT_SHADER_HASH)
            .map(|blob| unsafe { blob.as_ref::<ShaderHash>().clone() })
    }

    #[inline]
    pub fn disassembly(&self) -> windows::core::Result<String> {
        self.get_string(DXC_OUT_DISASSEMBLY)
    }

    #[inline]
    pub fn hlsl(&self) -> windows::core::Result<String> {
        self.get_string(DXC_OUT_HLSL)
    }

    #[inline]
    pub fn text(&self) -> windows::core::Result<String> {
        self.get_string(DXC_OUT_TEXT)
    }

    #[inline]
    pub fn reflection(&self, utils: &Utils) -> windows::core::Result<ShaderReflection> {
        ShaderReflection::from_compile_result(utils, self)
    }

    #[inline]
    pub fn root_signature(&self) -> windows::core::Result<RefBlob> {
        self.get_blob(DXC_OUT_ROOT_SIGNATURE)
    }

    #[inline]
    pub fn result(&self) -> &IDxcResult {
        &self.0
    }
}

#[derive(Debug)]
pub struct Arguments {
    source_name: Option<HSTRING>,
    target: Option<HSTRING>,
    entry_point: Option<HSTRING>,
    include_dirs: Vec<HSTRING>,
    defines: Vec<HSTRING>,
    optimization: Option<u32>,
    strict_mode: bool,
    no_legacy_cbuf_layout: bool,
    no_warnings: bool,
    validation: bool,
    debug_info: bool,
    hash_considering_only_binary: bool,
    hash_considering_source_info: bool,
    embed_debug: bool,
    warnings_as_errors: bool,
    hlsl_version: HSTRING,
    extra: Vec<HSTRING>,
}

impl Arguments {
    #[inline]
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self {
            source_name: None,
            target: None,
            entry_point: None,
            include_dirs: vec![],
            defines: vec![],
            optimization: Some(3),
            strict_mode: false,
            no_legacy_cbuf_layout: false,
            no_warnings: false,
            validation: true,
            debug_info: false,
            hash_considering_only_binary: false,
            hash_considering_source_info: false,
            embed_debug: false,
            warnings_as_errors: false,
            hlsl_version: "2021".into(),
            extra: vec![],
        }
    }

    #[inline]
    pub fn source_name(mut self, name: impl AsRef<str>) -> Self {
        self.source_name = Some(name.as_ref().into());
        self
    }

    #[inline]
    pub fn target(mut self, target: impl AsRef<str>) -> Self {
        self.target = Some(target.as_ref().into());
        self
    }

    #[inline]
    pub fn entry_point(mut self, entry_point: impl AsRef<str>) -> Self {
        self.entry_point = Some(entry_point.as_ref().into());
        self
    }

    #[inline]
    pub fn include_dir(mut self, path: impl AsRef<Path>) -> Self {
        self.include_dirs
            .push(path.as_ref().to_string_lossy().to_string().into());
        self
    }

    #[inline]
    pub fn define(mut self, define: impl AsRef<str>) -> Self {
        self.defines.push(define.as_ref().into());
        self
    }

    #[inline]
    pub fn optimization(mut self, level: Option<u32>) -> Self {
        self.optimization = level;
        self
    }

    #[inline]
    pub fn strict_mode(mut self, flag: bool) -> Self {
        self.strict_mode = flag;
        self
    }

    #[inline]
    pub fn no_legacy_cbuf_layout(mut self, flag: bool) -> Self {
        self.no_legacy_cbuf_layout = flag;
        self
    }

    #[inline]
    pub fn no_warnings(mut self, flag: bool) -> Self {
        self.no_warnings = flag;
        self
    }

    #[inline]
    pub fn validation(mut self, flag: bool) -> Self {
        self.validation = flag;
        self
    }

    #[inline]
    pub fn debug_info(mut self, flag: bool) -> Self {
        self.debug_info = flag;
        self
    }

    #[inline]
    pub fn hash_considering_only_binary(mut self, flag: bool) -> Self {
        self.hash_considering_only_binary = flag;
        self
    }

    #[inline]
    pub fn hash_considering_source_info(mut self, flag: bool) -> Self {
        self.hash_considering_source_info = flag;
        self
    }

    #[inline]
    pub fn embed_debug(mut self, flag: bool) -> Self {
        self.embed_debug = flag;
        self
    }

    #[inline]
    pub fn warnings_as_errors(mut self, flag: bool) -> Self {
        self.warnings_as_errors = flag;
        self
    }

    #[inline]
    pub fn hlsl_version(mut self, version: impl AsRef<str>) -> Self {
        self.hlsl_version = version.as_ref().into();
        self
    }

    #[inline]
    pub fn extra<T>(mut self, args: &[T]) -> Self
    where
        T: AsRef<str>,
    {
        self.extra = args
            .iter()
            .map(|arg| arg.as_ref().to_string().into())
            .collect();
        self
    }

    fn build(&self) -> (Vec<HSTRING>, Vec<PCWSTR>) {
        let mut args = vec![];
        self.source_name
            .as_ref()
            .inspect(|&source_name| args.push(source_name.clone()));
        self.target.as_ref().inspect(|&target| {
            args.push("-T".into());
            args.push(target.clone());
        });
        self.entry_point.as_ref().inspect(|&entry_point| {
            args.push("-E".into());
            args.push(entry_point.clone())
        });
        self.include_dirs.iter().for_each(|dir| {
            args.push("-I".into());
            args.push(dir.clone());
        });
        self.defines.iter().for_each(|d| {
            args.push("-D".into());
            args.push(d.clone());
        });
        args.push(
            match self.optimization {
                None => "-Od",
                Some(0) => "-O0",
                Some(1) => "-O1",
                Some(2) => "-O2",
                Some(3) => "-O3",
                _ => unimplemented!(),
            }
            .into(),
        );
        self.strict_mode.then(|| args.push("-Ges".into()));
        self.no_legacy_cbuf_layout
            .then(|| args.push("-no-legacy-cbuf-layout".into()));
        self.no_warnings.then(|| args.push("-no-warnings".into()));
        (!self.validation).then(|| args.push("-Vd".into()));
        self.debug_info.then(|| args.push("-Zi".into()));
        self.hash_considering_only_binary
            .then(|| args.push("-Zsb".into()));
        self.hash_considering_source_info
            .then(|| args.push("-Zss".into()));
        self.embed_debug.then(|| args.push("-Qembed_debug".into()));
        self.warnings_as_errors.then(|| args.push("-WX".into()));
        args.push("-HV".into());
        args.push(self.hlsl_version.clone());
        self.extra.iter().for_each(|arg| args.push(arg.clone()));
        let ptrs = args.iter().map(|arg| PCWSTR(arg.as_ptr())).collect();
        (args, ptrs)
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Compiler {
    utils: Utils,
    compiler: IDxcCompiler3,
}

impl Compiler {
    #[inline]
    pub fn new(utils: &Utils) -> windows::core::Result<Self> {
        unsafe {
            Ok(Self {
                utils: utils.clone(),
                compiler: DxcCreateInstance(&CLSID_DxcCompiler)?,
            })
        }
    }

    #[inline]
    pub fn compile(&self, src: &[u8], args: &Arguments) -> windows::core::Result<CompileResult> {
        let buffer = DxcBuffer {
            Ptr: src.as_ptr() as *const std::ffi::c_void,
            Size: src.len(),
            Encoding: DXC_CP_UTF8.0,
        };
        let (_a, args) = args.build();
        let result = unsafe {
            let include = self.utils.0.CreateDefaultIncludeHandler()?;
            self.compiler
                .Compile(&buffer, Some(&args), Some(&include))?
        };
        Ok(CompileResult(result))
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct PartKind(u32);

impl PartKind {
    #[inline]
    pub fn new(ch: [char; 4]) -> Self {
        assert!(ch.iter().all(|c| c.is_ascii_alphanumeric()));
        Self(ch[0] as u32 | (ch[1] as u32) << 8 | (ch[2] as u32) << 16 | (ch[3] as u32) << 24)
    }

    #[inline]
    pub const fn fourcc(ch0: u8, ch1: u8, ch2: u8, ch3: u8) -> Self {
        Self(ch0 as u32 | (ch1 as u32) << 8 | (ch2 as u32) << 16 | (ch3 as u32) << 24)
    }

    #[inline]
    pub fn to_chars(&self) -> [char; 4] {
        [
            char::from_u32(self.0 & 0xff).unwrap(),
            char::from_u32((self.0 >> 8) & 0xff).unwrap(),
            char::from_u32((self.0 >> 16) & 0xff).unwrap(),
            char::from_u32((self.0 >> 24) & 0xff).unwrap(),
        ]
    }

    pub const DXBC: Self = Self::fourcc(b'D', b'X', b'B', b'C');
    pub const RESOURCE_DEF: Self = Self::fourcc(b'R', b'D', b'E', b'F');
    pub const INPUT_SIGNATURE: Self = Self::fourcc(b'I', b'S', b'G', b'1');
    pub const OUTPUT_SIGNATURE: Self = Self::fourcc(b'O', b'S', b'G', b'1');
    pub const PATCH_CONSTANT_SIGNATURE: Self = Self::fourcc(b'P', b'S', b'G', b'1');
    pub const SHADER_STATISTICS: Self = Self::fourcc(b'S', b'T', b'A', b'T');
    pub const SHADER_DEBUG_INFO: Self = Self::fourcc(b'I', b'L', b'D', b'B');
    pub const SHADER_DEBUG_NAME: Self = Self::fourcc(b'I', b'L', b'D', b'N');
    pub const FEATURE_INFO: Self = Self::fourcc(b'S', b'F', b'I', b'0');
    pub const PRIVATE_DATA: Self = Self::fourcc(b'P', b'R', b'I', b'V');
    pub const ROOT_SIGNATURE: Self = Self::fourcc(b'R', b'T', b'S', b'0');
    pub const DXIL: Self = Self::fourcc(b'D', b'X', b'I', b'L');
    pub const PIPELINE_STATE_VALIDATION: Self = Self::fourcc(b'P', b'S', b'V', b'0');
    pub const RUNTIME_DATA: Self = Self::fourcc(b'R', b'D', b'A', b'T');
    pub const SHADER_HASH: Self = Self::fourcc(b'H', b'A', b'S', b'H');
    pub const SHADER_SOURCE_INFO: Self = Self::fourcc(b'S', b'R', b'C', b'I');
    pub const SHADER_PDB_INFO: Self = Self::fourcc(b'P', b'D', b'B', b'I');
    pub const COMPILER_VERSION: Self = Self::fourcc(b'V', b'E', b'R', b'S');
}

impl std::fmt::Debug for PartKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.to_chars())
    }
}

#[derive(Clone, Debug)]
pub struct Part {
    index: u32,
    container: IDxcContainerReflection,
    pub kind: PartKind,
    pub content: Blob,
}

impl Part {
    #[inline]
    pub fn reflection<T>(&self) -> windows::core::Result<T>
    where
        T: ReflectionType,
    {
        let obj = unsafe {
            let mut p: Option<T::Type> = None;
            self.container
                .GetPartReflection(self.index, &T::Type::IID, &mut p as *mut _ as *mut _)
                .map(|_| p.unwrap())?
        };
        Ok(T::new(obj))
    }
}

#[derive(Clone, Debug)]
pub struct ContainerReflection(IDxcContainerReflection);

impl ContainerReflection {
    #[inline]
    pub fn new(blob: &impl BlobType) -> windows::core::Result<Self> {
        unsafe {
            let reflection: IDxcContainerReflection =
                DxcCreateInstance(&CLSID_DxcContainerReflection)?;
            reflection.Load(&blob.handle())?;
            Ok(Self(reflection))
        }
    }

    #[inline]
    pub fn parts(&self) -> impl Iterator<Item = Part> + '_ {
        unsafe {
            let len = self.0.GetPartCount().unwrap();
            (0..len)
                .map(|i| -> windows::core::Result<Part> {
                    let kind = self.0.GetPartKind(i)?;
                    let content = self.0.GetPartContent(i)?;
                    Ok(Part {
                        index: i,
                        container: self.0.clone(),
                        kind: PartKind(kind),
                        content: Blob(content),
                    })
                })
                .filter_map(|part| part.ok())
        }
    }
}
