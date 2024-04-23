use super::*;
use std::mem::ManuallyDrop;
use windows::core::Interface;
use windows::core::{HSTRING, PCWSTR};
use windows::Win32::Graphics::Direct3D12::*;

pub trait StateSubobject {
    fn type_value(&self) -> D3D12_STATE_SUBOBJECT_TYPE;

    fn desc(&self) -> *const std::ffi::c_void {
        self as *const Self as *const _
    }
}

#[derive(Clone, Debug)]
#[repr(transparent)]
pub struct StateObjectConfig(D3D12_STATE_OBJECT_CONFIG);

impl StateObjectConfig {
    #[inline]
    pub fn new(flags: D3D12_STATE_OBJECT_FLAGS) -> Self {
        Self(D3D12_STATE_OBJECT_CONFIG { Flags: flags })
    }
}

impl StateSubobject for StateObjectConfig {
    fn type_value(&self) -> D3D12_STATE_SUBOBJECT_TYPE {
        D3D12_STATE_SUBOBJECT_TYPE_STATE_OBJECT_CONFIG
    }
}

#[derive(Clone, Debug)]
#[repr(transparent)]
pub struct GlobalRootSignature(D3D12_GLOBAL_ROOT_SIGNATURE);

impl GlobalRootSignature {
    #[inline]
    pub fn new(root_sig: &RootSignature) -> Self {
        Self(D3D12_GLOBAL_ROOT_SIGNATURE {
            pGlobalRootSignature: ManuallyDrop::new(Some(root_sig.handle().clone())),
        })
    }
}

impl StateSubobject for GlobalRootSignature {
    fn type_value(&self) -> D3D12_STATE_SUBOBJECT_TYPE {
        D3D12_STATE_SUBOBJECT_TYPE_GLOBAL_ROOT_SIGNATURE
    }
}

impl Drop for GlobalRootSignature {
    fn drop(&mut self) {
        unsafe {
            ManuallyDrop::drop(&mut self.0.pGlobalRootSignature);
        }
    }
}

#[derive(Clone, Debug)]
#[repr(transparent)]
pub struct LocalRootSignature(D3D12_LOCAL_ROOT_SIGNATURE);

impl LocalRootSignature {
    #[inline]
    pub fn new(root_sig: &RootSignature) -> Self {
        Self(D3D12_LOCAL_ROOT_SIGNATURE {
            pLocalRootSignature: ManuallyDrop::new(Some(root_sig.handle().clone())),
        })
    }
}

impl StateSubobject for LocalRootSignature {
    fn type_value(&self) -> D3D12_STATE_SUBOBJECT_TYPE {
        D3D12_STATE_SUBOBJECT_TYPE_LOCAL_ROOT_SIGNATURE
    }
}

impl Drop for LocalRootSignature {
    fn drop(&mut self) {
        unsafe {
            ManuallyDrop::drop(&mut self.0.pLocalRootSignature);
        }
    }
}

#[derive(Clone, Debug)]
#[repr(transparent)]
pub struct NodeMask(D3D12_NODE_MASK);

impl NodeMask {
    #[inline]
    pub fn new(node_mask: u32) -> Self {
        Self(D3D12_NODE_MASK {
            NodeMask: node_mask,
        })
    }
}

impl StateSubobject for NodeMask {
    fn type_value(&self) -> D3D12_STATE_SUBOBJECT_TYPE {
        D3D12_STATE_SUBOBJECT_TYPE_NODE_MASK
    }
}

#[derive(Clone)]
pub struct ExportDesc {
    name: HSTRING,
    export_to_rename: Option<HSTRING>,
    flags: D3D12_EXPORT_FLAGS,
}

impl ExportDesc {
    #[inline]
    pub fn new() -> Self {
        Self {
            name: HSTRING::new(),
            export_to_rename: None,
            flags: D3D12_EXPORT_FLAG_NONE,
        }
    }

    #[inline]
    pub fn name(mut self, name: impl AsRef<str>) -> Self {
        self.name = HSTRING::from(name.as_ref());
        self
    }

    #[inline]
    pub fn export_to_rename(mut self, name: Option<&str>) -> Self {
        self.export_to_rename = name.map(|n| HSTRING::from(n));
        self
    }
}

pub struct DxilLibraryDesc<'a> {
    _dxil_library: ShaderBytecode<'a>,
    exports: Vec<ExportDesc>,
    export_descs: Vec<D3D12_EXPORT_DESC>,
    desc: D3D12_DXIL_LIBRARY_DESC,
}

impl<'a> DxilLibraryDesc<'a> {
    #[inline]
    pub fn new(dxil_library: ShaderBytecode<'a>) -> Self {
        let desc = D3D12_DXIL_LIBRARY_DESC {
            DXILLibrary: dxil_library.desc,
            ..Default::default()
        };
        Self {
            _dxil_library: dxil_library,
            exports: vec![],
            export_descs: vec![],
            desc,
        }
    }

    #[inline]
    pub fn exports(mut self, exports: &[ExportDesc]) -> Self {
        self.exports = exports.to_vec();
        self.export_descs = self
            .exports
            .iter()
            .map(|export| D3D12_EXPORT_DESC {
                Name: PCWSTR(export.name.as_ptr()),
                ExportToRename: export
                    .export_to_rename
                    .as_ref()
                    .map_or(PCWSTR::null(), |n| PCWSTR(n.as_ptr())),
                Flags: export.flags,
            })
            .collect::<Vec<_>>();
        self.desc.pExports = self.export_descs.as_ptr();
        self.desc.NumExports = self.export_descs.len() as u32;
        self
    }
}

impl<'a> StateSubobject for DxilLibraryDesc<'a> {
    fn type_value(&self) -> D3D12_STATE_SUBOBJECT_TYPE {
        D3D12_STATE_SUBOBJECT_TYPE_DXIL_LIBRARY
    }

    fn desc(&self) -> *const std::ffi::c_void {
        &self.desc as *const D3D12_DXIL_LIBRARY_DESC as *const _
    }
}

pub struct ExistingCollectionDesc {
    desc: D3D12_EXISTING_COLLECTION_DESC,
    exports: Vec<ExportDesc>,
    export_descs: Vec<D3D12_EXPORT_DESC>,
}

impl ExistingCollectionDesc {
    #[inline]
    pub fn new(state_object: &StateObject) -> Self {
        Self {
            desc: D3D12_EXISTING_COLLECTION_DESC {
                pExistingCollection: ManuallyDrop::new(Some(state_object.handle().clone())),
                NumExports: 0,
                pExports: std::ptr::null(),
            },
            exports: vec![],
            export_descs: vec![],
        }
    }

    #[inline]
    pub fn exports(mut self, exports: &[ExportDesc]) -> Self {
        self.exports = exports.to_vec();
        self.export_descs = self
            .exports
            .iter()
            .map(|export| D3D12_EXPORT_DESC {
                Name: PCWSTR(export.name.as_ptr()),
                ExportToRename: PCWSTR(
                    export
                        .export_to_rename
                        .as_ref()
                        .map_or(std::ptr::null(), |n| n.as_ptr()),
                ),
                Flags: export.flags,
            })
            .collect::<Vec<_>>();
        self.desc.pExports = self.export_descs.as_ptr();
        self.desc.NumExports = self.export_descs.len() as u32;
        self
    }
}

impl StateSubobject for ExistingCollectionDesc {
    fn type_value(&self) -> D3D12_STATE_SUBOBJECT_TYPE {
        D3D12_STATE_SUBOBJECT_TYPE_EXISTING_COLLECTION
    }

    fn desc(&self) -> *const std::ffi::c_void {
        &self.desc as *const D3D12_EXISTING_COLLECTION_DESC as *const _
    }
}

impl Drop for ExistingCollectionDesc {
    fn drop(&mut self) {
        unsafe {
            ManuallyDrop::drop(&mut self.desc.pExistingCollection);
        }
    }
}

pub struct SubobjectToExportsAssociation<'a> {
    assoc: &'a dyn StateSubobject,
    exports: Vec<HSTRING>,
    export_ptrs: Vec<PCWSTR>,
}

impl<'a> SubobjectToExportsAssociation<'a> {
    #[inline]
    pub fn new(assoc: &'a dyn StateSubobject) -> Self {
        Self {
            assoc,
            exports: vec![],
            export_ptrs: vec![],
        }
    }

    #[inline]
    pub fn exports(mut self, exports: &[&str]) -> Self {
        let exports = exports
            .iter()
            .map(|export| HSTRING::from(*export))
            .collect::<Vec<_>>();
        let export_ptrs = exports
            .iter()
            .map(|export| PCWSTR(export.as_ptr()))
            .collect::<Vec<_>>();
        self.exports = exports;
        self.export_ptrs = export_ptrs;
        self
    }
}

pub struct DxilSubobjectToExportsAssociation {
    desc: D3D12_DXIL_SUBOBJECT_TO_EXPORTS_ASSOCIATION,
    _assoc: HSTRING,
    exports: Vec<HSTRING>,
    export_ptrs: Vec<PCWSTR>,
}

impl DxilSubobjectToExportsAssociation {
    #[inline]
    pub fn new(assoc: impl AsRef<str>) -> Self {
        let assoc = HSTRING::from(assoc.as_ref());
        Self {
            desc: D3D12_DXIL_SUBOBJECT_TO_EXPORTS_ASSOCIATION {
                SubobjectToAssociate: PCWSTR(assoc.as_ptr()),
                NumExports: 0,
                pExports: std::ptr::null(),
            },
            _assoc: assoc,
            exports: vec![],
            export_ptrs: vec![],
        }
    }

    #[inline]
    pub fn exports(mut self, exports: &[&str]) -> Self {
        let exports = exports
            .iter()
            .map(|export| HSTRING::from(*export))
            .collect::<Vec<_>>();
        let export_ptrs = exports
            .iter()
            .map(|export| PCWSTR(export.as_ptr()))
            .collect::<Vec<_>>();
        self.exports = exports;
        self.export_ptrs = export_ptrs;
        self.desc.NumExports = self.export_ptrs.len() as u32;
        self.desc.pExports = self.export_ptrs.as_ptr();
        self
    }
}

impl StateSubobject for DxilSubobjectToExportsAssociation {
    fn type_value(&self) -> D3D12_STATE_SUBOBJECT_TYPE {
        D3D12_STATE_SUBOBJECT_TYPE_DXIL_SUBOBJECT_TO_EXPORTS_ASSOCIATION
    }

    fn desc(&self) -> *const std::ffi::c_void {
        &self.desc as *const D3D12_DXIL_SUBOBJECT_TO_EXPORTS_ASSOCIATION as *const _
    }
}

#[derive(Clone, Debug)]
#[repr(transparent)]
pub struct RaytracingShaderConfig(D3D12_RAYTRACING_SHADER_CONFIG);

impl RaytracingShaderConfig {
    #[inline]
    pub fn new() -> Self {
        Self(D3D12_RAYTRACING_SHADER_CONFIG::default())
    }

    #[inline]
    pub fn max_payload_size_in_bytes(mut self, size: u32) -> Self {
        self.0.MaxPayloadSizeInBytes = size;
        self
    }

    #[inline]
    pub fn max_attribute_size_in_bytes(mut self, size: u32) -> Self {
        self.0.MaxAttributeSizeInBytes = size;
        self
    }
}

impl StateSubobject for RaytracingShaderConfig {
    fn type_value(&self) -> D3D12_STATE_SUBOBJECT_TYPE {
        D3D12_STATE_SUBOBJECT_TYPE_RAYTRACING_SHADER_CONFIG
    }
}

#[derive(Clone, Debug)]
#[repr(transparent)]
pub struct RaytracingPipelineConfig(D3D12_RAYTRACING_PIPELINE_CONFIG1);

impl RaytracingPipelineConfig {
    #[inline]
    pub fn new() -> Self {
        Self(D3D12_RAYTRACING_PIPELINE_CONFIG1::default())
    }

    #[inline]
    pub fn max_trace_recursion_depth(mut self, depth: u32) -> Self {
        self.0.MaxTraceRecursionDepth = depth;
        self
    }

    #[inline]
    pub fn flags(mut self, flags: D3D12_RAYTRACING_PIPELINE_FLAGS) -> Self {
        self.0.Flags = flags;
        self
    }
}

impl StateSubobject for RaytracingPipelineConfig {
    fn type_value(&self) -> D3D12_STATE_SUBOBJECT_TYPE {
        D3D12_STATE_SUBOBJECT_TYPE_RAYTRACING_PIPELINE_CONFIG1
    }
}

pub struct HitGroupDesc {
    desc: D3D12_HIT_GROUP_DESC,
    export: HSTRING,
    any_hit_shader_import: Option<HSTRING>,
    closest_hit_shader_import: Option<HSTRING>,
    intersection_shader_import: Option<HSTRING>,
}

impl HitGroupDesc {
    #[inline]
    pub fn new() -> Self {
        Self {
            desc: Default::default(),
            export: HSTRING::new(),
            any_hit_shader_import: None,
            closest_hit_shader_import: None,
            intersection_shader_import: None,
        }
    }

    #[inline]
    pub fn hit_group_type(mut self, ty: D3D12_HIT_GROUP_TYPE) -> Self {
        self.desc.Type = ty;
        self
    }

    #[inline]
    pub fn export(mut self, export: impl AsRef<str>) -> Self {
        self.export = export.as_ref().into();
        self.desc.HitGroupExport = PCWSTR(self.export.as_ptr());
        self
    }

    #[inline]
    pub fn any_hit_shader_import(mut self, import: impl AsRef<str>) -> Self {
        self.any_hit_shader_import = Some(import.as_ref().into());
        self.desc.AnyHitShaderImport = self
            .any_hit_shader_import
            .as_ref()
            .map_or(PCWSTR::null(), |import| PCWSTR(import.as_ptr()));
        self
    }

    #[inline]
    pub fn closest_hit_shader_import(mut self, import: impl AsRef<str>) -> Self {
        self.closest_hit_shader_import = Some(import.as_ref().into());
        self.desc.ClosestHitShaderImport = self
            .closest_hit_shader_import
            .as_ref()
            .map_or(PCWSTR::null(), |import| PCWSTR(import.as_ptr()));
        self
    }

    #[inline]
    pub fn intersection_shader_import(mut self, import: impl AsRef<str>) -> Self {
        self.intersection_shader_import = Some(import.as_ref().into());
        self.desc.IntersectionShaderImport = self
            .intersection_shader_import
            .as_ref()
            .map_or(PCWSTR::null(), |import| PCWSTR(import.as_ptr()));
        self
    }
}

impl StateSubobject for HitGroupDesc {
    fn type_value(&self) -> D3D12_STATE_SUBOBJECT_TYPE {
        D3D12_STATE_SUBOBJECT_TYPE_HIT_GROUP
    }

    fn desc(&self) -> *const std::ffi::c_void {
        &self.desc as *const D3D12_HIT_GROUP_DESC as *const _
    }
}

pub struct Builder<'a> {
    device: ID3D12Device5,
    ty: D3D12_STATE_OBJECT_TYPE,
    subobjects: Vec<&'a dyn StateSubobject>,
    associations: Vec<&'a SubobjectToExportsAssociation<'a>>,
    name: Option<String>,
}

impl<'a> Builder<'a> {
    fn new<T>(device: &T, ty: D3D12_STATE_OBJECT_TYPE) -> Self
    where
        T: Into<ID3D12Device5> + Clone,
    {
        Self {
            device: device.clone().into(),
            ty,
            subobjects: vec![],
            associations: vec![],
            name: None,
        }
    }

    #[inline]
    pub fn subobject(mut self, subobject: &'a impl StateSubobject) -> Self {
        self.subobjects.push(subobject);
        self
    }

    #[inline]
    pub fn associate(mut self, assoc: &'a SubobjectToExportsAssociation<'a>) -> Self {
        self.associations.push(assoc);
        self
    }

    #[inline]
    pub fn name(mut self, name: impl AsRef<str>) -> Self {
        self.name = Some(name.as_ref().to_string());
        self
    }

    #[inline]
    pub fn build(self) -> windows::core::Result<StateObject> {
        let mut subobjects: Vec<D3D12_STATE_SUBOBJECT> =
            Vec::with_capacity(self.subobjects.len() + self.associations.len());
        for subobject in self.subobjects.iter() {
            subobjects.push(D3D12_STATE_SUBOBJECT {
                Type: subobject.type_value(),
                pDesc: subobject.desc(),
            });
        }
        let associations = self
            .associations
            .iter()
            .map(|i| {
                let index = self
                    .subobjects
                    .iter()
                    .position(|sub| sub.desc() == i.assoc.desc())
                    .unwrap();
                D3D12_SUBOBJECT_TO_EXPORTS_ASSOCIATION {
                    pSubobjectToAssociate: &subobjects[index],
                    NumExports: i.export_ptrs.len() as u32,
                    pExports: i.export_ptrs.as_ptr(),
                }
            })
            .collect::<Vec<_>>();
        for a in associations.iter() {
            subobjects.push(D3D12_STATE_SUBOBJECT {
                Type: D3D12_STATE_SUBOBJECT_TYPE_SUBOBJECT_TO_EXPORTS_ASSOCIATION,
                pDesc: a as *const D3D12_SUBOBJECT_TO_EXPORTS_ASSOCIATION as *const _,
            });
        }
        let handle: ID3D12StateObject = unsafe {
            self.device.CreateStateObject(&D3D12_STATE_OBJECT_DESC {
                Type: self.ty,
                NumSubobjects: subobjects.len() as u32,
                pSubobjects: subobjects.as_ptr(),
            })?
        };
        let name = self.name.as_ref().map(|n| Name::new(&handle, n));
        Ok(StateObject { handle, name })
    }
}

#[derive(Clone, Debug)]
pub struct StateObject {
    handle: ID3D12StateObject,
    name: Option<Name>,
}

impl StateObject {
    #[inline]
    pub fn new(device: &Device, ty: D3D12_STATE_OBJECT_TYPE) -> Builder {
        Builder::new(device.handle(), ty)
    }

    #[inline]
    pub fn handle(&self) -> &ID3D12StateObject {
        &self.handle
    }

    #[inline]
    pub fn name(&self) -> Option<&str> {
        self.name.as_ref().map(|n| n.as_str())
    }
}

impl super::command_list::PipelineStateType for StateObject {
    fn call(&self, cmd_list: &ID3D12GraphicsCommandList7) {
        unsafe {
            cmd_list.SetPipelineState1(&self.handle);
        }
    }
}

#[derive(Clone, Debug)]
pub struct StateObjectProperties {
    handle: ID3D12StateObjectProperties,
}

impl StateObjectProperties {
    #[inline]
    pub fn new(state_object: &StateObject) -> Self {
        Self {
            handle: state_object.handle.cast().unwrap(),
        }
    }

    #[inline]
    pub fn get_pipeline_stack_size(&self) -> u64 {
        unsafe { self.handle.GetPipelineStackSize() }
    }

    #[inline]
    pub fn set_pipeline_stack_size(&self, size: u64) {
        unsafe {
            self.handle.SetPipelineStackSize(size);
        }
    }

    #[inline]
    pub fn get_shader_identifier(&self, export_name: impl AsRef<str>) -> *mut std::ffi::c_void {
        unsafe {
            self.handle
                .GetShaderIdentifier(&HSTRING::from(export_name.as_ref()))
        }
    }

    #[inline]
    pub fn get_shader_size(&self, export_name: impl AsRef<str>) -> u64 {
        unsafe {
            self.handle
                .GetShaderStackSize(&HSTRING::from(export_name.as_ref()))
        }
    }

    #[inline]
    pub fn handle(&self) -> &ID3D12StateObjectProperties {
        &self.handle
    }
}
