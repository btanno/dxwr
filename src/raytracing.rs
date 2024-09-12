use super::*;
use windows::Win32::Graphics::Direct3D12::*;
use windows::Win32::Graphics::Dxgi::Common::DXGI_FORMAT;

#[repr(transparent)]
pub struct RaytracingGeometryTrianglesDesc(D3D12_RAYTRACING_GEOMETRY_DESC);

impl RaytracingGeometryTrianglesDesc {
    #[inline]
    pub fn flags(mut self, flags: D3D12_RAYTRACING_GEOMETRY_FLAGS) -> Self {
        self.0.Flags = flags;
        self
    }

    #[inline]
    pub fn vertex_buffer(
        mut self,
        addr_and_stride: GpuVirtualAddressAndStride,
        count: u32,
        format: DXGI_FORMAT,
    ) -> Self {
        self.0.Anonymous.Triangles.VertexBuffer = addr_and_stride.0;
        self.0.Anonymous.Triangles.VertexCount = count;
        self.0.Anonymous.Triangles.VertexFormat = format;
        self
    }

    #[inline]
    pub fn index_buffer(
        mut self,
        addr: GpuVirtualAddress,
        count: u32,
        format: DXGI_FORMAT,
    ) -> Self {
        self.0.Anonymous.Triangles.IndexBuffer = addr.0;
        self.0.Anonymous.Triangles.IndexCount = count;
        self.0.Anonymous.Triangles.IndexFormat = format;
        self
    }

    #[inline]
    pub fn transform3x4(mut self, addr: GpuVirtualAddress) -> Self {
        self.0.Anonymous.Triangles.Transform3x4 = addr.0;
        self
    }
}

impl From<RaytracingGeometryTrianglesDesc> for RaytracingGeometryDesc {
    fn from(value: RaytracingGeometryTrianglesDesc) -> Self {
        Self(value.0)
    }
}

#[repr(transparent)]
pub struct RaytracingGeometryAABBsDesc(D3D12_RAYTRACING_GEOMETRY_DESC);

impl RaytracingGeometryAABBsDesc {
    #[inline]
    pub fn flags(mut self, flags: D3D12_RAYTRACING_GEOMETRY_FLAGS) -> Self {
        self.0.Flags = flags;
        self
    }

    #[inline]
    pub fn aabb_count(mut self, count: u64) -> Self {
        self.0.Anonymous.AABBs.AABBCount = count;
        self
    }

    #[inline]
    pub fn aabbs(mut self, aabbs: GpuVirtualAddressAndStride) -> Self {
        self.0.Anonymous.AABBs.AABBs = aabbs.0;
        self
    }
}

impl From<RaytracingGeometryAABBsDesc> for RaytracingGeometryDesc {
    fn from(value: RaytracingGeometryAABBsDesc) -> Self {
        Self(value.0)
    }
}

#[repr(transparent)]
pub struct RaytracingGeometryDesc(D3D12_RAYTRACING_GEOMETRY_DESC);

impl RaytracingGeometryDesc {
    #[inline]
    pub fn triangles() -> RaytracingGeometryTrianglesDesc {
        RaytracingGeometryTrianglesDesc(D3D12_RAYTRACING_GEOMETRY_DESC {
            Type: D3D12_RAYTRACING_GEOMETRY_TYPE_TRIANGLES,
            Flags: D3D12_RAYTRACING_GEOMETRY_FLAG_NONE,
            Anonymous: D3D12_RAYTRACING_GEOMETRY_DESC_0 {
                Triangles: D3D12_RAYTRACING_GEOMETRY_TRIANGLES_DESC::default(),
            },
        })
    }

    #[inline]
    pub fn aabbs() -> RaytracingGeometryAABBsDesc {
        RaytracingGeometryAABBsDesc(D3D12_RAYTRACING_GEOMETRY_DESC {
            Type: D3D12_RAYTRACING_GEOMETRY_TYPE_PROCEDURAL_PRIMITIVE_AABBS,
            Flags: D3D12_RAYTRACING_GEOMETRY_FLAG_NONE,
            Anonymous: D3D12_RAYTRACING_GEOMETRY_DESC_0 {
                AABBs: D3D12_RAYTRACING_GEOMETRY_AABBS_DESC::default(),
            },
        })
    }
}

pub trait BuildRaytracingAccelerationStructureInputsType {
    fn get(&self) -> &D3D12_BUILD_RAYTRACING_ACCELERATION_STRUCTURE_INPUTS;
}

#[derive(Clone)]
#[repr(transparent)]
pub struct TopLevel(D3D12_BUILD_RAYTRACING_ACCELERATION_STRUCTURE_INPUTS);

impl TopLevel {
    #[inline]
    pub fn flags(mut self, flags: D3D12_RAYTRACING_ACCELERATION_STRUCTURE_BUILD_FLAGS) -> Self {
        self.0.Flags = flags;
        self
    }

    #[inline]
    pub fn num_descs(mut self, num: u32) -> Self {
        self.0.NumDescs = num;
        self
    }

    #[inline]
    pub fn instance_descs(mut self, descs: GpuVirtualAddress) -> Self {
        self.0.Anonymous.InstanceDescs = descs.0;
        self
    }
}

impl BuildRaytracingAccelerationStructureInputsType for TopLevel {
    fn get(&self) -> &D3D12_BUILD_RAYTRACING_ACCELERATION_STRUCTURE_INPUTS {
        &self.0
    }
}

impl From<TopLevel> for BuildRaytracingAccelerationStructureInputs {
    fn from(value: TopLevel) -> Self {
        Self(value.0)
    }
}

#[derive(Clone)]
#[repr(transparent)]
pub struct BottomLevel(D3D12_BUILD_RAYTRACING_ACCELERATION_STRUCTURE_INPUTS);

impl BottomLevel {
    #[inline]
    pub fn flags(mut self, flags: D3D12_RAYTRACING_ACCELERATION_STRUCTURE_BUILD_FLAGS) -> Self {
        self.0.Flags = flags;
        self
    }

    #[inline]
    pub fn geometry_descs(mut self, descs: &[RaytracingGeometryDesc]) -> Self {
        self.0.NumDescs = descs.len() as u32;
        self.0.Anonymous.pGeometryDescs = descs.as_ptr() as *const D3D12_RAYTRACING_GEOMETRY_DESC;
        self
    }
}

impl BuildRaytracingAccelerationStructureInputsType for BottomLevel {
    fn get(&self) -> &D3D12_BUILD_RAYTRACING_ACCELERATION_STRUCTURE_INPUTS {
        &self.0
    }
}

impl From<BottomLevel> for BuildRaytracingAccelerationStructureInputs {
    fn from(value: BottomLevel) -> Self {
        Self(value.0)
    }
}

#[repr(transparent)]
pub struct BuildRaytracingAccelerationStructureInputs(
    D3D12_BUILD_RAYTRACING_ACCELERATION_STRUCTURE_INPUTS,
);

impl BuildRaytracingAccelerationStructureInputs {
    #[inline]
    pub fn top_level() -> TopLevel {
        TopLevel(D3D12_BUILD_RAYTRACING_ACCELERATION_STRUCTURE_INPUTS {
            Type: D3D12_RAYTRACING_ACCELERATION_STRUCTURE_TYPE_TOP_LEVEL,
            DescsLayout: D3D12_ELEMENTS_LAYOUT_ARRAY,
            ..Default::default()
        })
    }

    #[inline]
    pub fn bottom_level() -> BottomLevel {
        BottomLevel(D3D12_BUILD_RAYTRACING_ACCELERATION_STRUCTURE_INPUTS {
            Type: D3D12_RAYTRACING_ACCELERATION_STRUCTURE_TYPE_BOTTOM_LEVEL,
            DescsLayout: D3D12_ELEMENTS_LAYOUT_ARRAY,
            ..Default::default()
        })
    }
}

#[derive(Clone, Copy, Debug)]
pub struct RaytracingAccelerationStructurePrebuildInfo {
    pub result_data_size_in_bytes: u64,
    pub scratch_data_size_in_bytes: u64,
    pub update_scratch_data_size_in_bytes: u64,
}

impl From<D3D12_RAYTRACING_ACCELERATION_STRUCTURE_PREBUILD_INFO>
    for RaytracingAccelerationStructurePrebuildInfo
{
    fn from(value: D3D12_RAYTRACING_ACCELERATION_STRUCTURE_PREBUILD_INFO) -> Self {
        Self {
            result_data_size_in_bytes: value.ResultDataMaxSizeInBytes,
            scratch_data_size_in_bytes: value.ScratchDataSizeInBytes,
            update_scratch_data_size_in_bytes: value.UpdateScratchDataSizeInBytes,
        }
    }
}

#[repr(transparent)]
pub struct RaytracingInstanceDesc(D3D12_RAYTRACING_INSTANCE_DESC);

impl RaytracingInstanceDesc {
    #[inline]
    pub fn new() -> Self {
        let this = Self(D3D12_RAYTRACING_INSTANCE_DESC {
            Transform: [1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0],
            ..Default::default()
        });
        this.instance_mask(1)
    }

    #[inline]
    pub fn transform(mut self, m: [f32; 12]) -> Self {
        self.0.Transform = m;
        self
    }

    #[inline]
    pub fn instance_id(mut self, id: u32) -> Self {
        self.0._bitfield1 = (self.0._bitfield1 & 0xff000000) | (id & 0x00ffffff);
        self
    }

    #[inline]
    pub fn instance_mask(mut self, mask: u8) -> Self {
        self.0._bitfield1 = ((mask as u32) << 24) | (self.0._bitfield1 & 0x00ffffff);
        self
    }

    #[inline]
    pub fn instance_contribution_to_hit_group_index(mut self, index: u32) -> Self {
        self.0._bitfield2 = (self.0._bitfield2 & 0xff000000) | (index & 0x00ffffff);
        self
    }

    #[inline]
    pub fn flags(mut self, flags: D3D12_RAYTRACING_INSTANCE_FLAGS) -> Self {
        self.0._bitfield2 = ((flags.0 as u32) << 24) | (self.0._bitfield2 & 0x00ffffff);
        self
    }

    #[inline]
    pub fn accelration_structure(mut self, addr: GpuVirtualAddress) -> Self {
        self.0.AccelerationStructure = addr.0;
        self
    }
}

impl Default for RaytracingInstanceDesc {
    fn default() -> Self {
        Self::new()
    }
}

#[repr(transparent)]
pub struct BuildRaytracingAccelerationStructureDesc(
    pub(crate) D3D12_BUILD_RAYTRACING_ACCELERATION_STRUCTURE_DESC,
);

impl BuildRaytracingAccelerationStructureDesc {
    #[inline]
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self(D3D12_BUILD_RAYTRACING_ACCELERATION_STRUCTURE_DESC::default())
    }

    #[inline]
    pub fn dest_acceleration_structure_data(mut self, addr: GpuVirtualAddress) -> Self {
        self.0.DestAccelerationStructureData = addr.0;
        self
    }

    #[inline]
    pub fn inputs<T>(mut self, inputs: &T) -> Self
    where
        T: BuildRaytracingAccelerationStructureInputsType,
    {
        self.0.Inputs = *inputs.get();
        self
    }

    #[inline]
    pub fn source_acceleration_structure_data(mut self, addr: GpuVirtualAddress) -> Self {
        self.0.SourceAccelerationStructureData = addr.0;
        self
    }

    #[inline]
    pub fn scratch_acceleration_structure_data(mut self, addr: GpuVirtualAddress) -> Self {
        self.0.ScratchAccelerationStructureData = addr.0;
        self
    }
}
