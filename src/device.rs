use super::raytracing::BuildRaytracingAccelerationStructureInputsType;
use super::*;
use windows::core::IUnknown;
use windows::Win32::Foundation::LUID;
use windows::Win32::Graphics::Direct3D::*;
use windows::Win32::Graphics::Direct3D12::*;
use windows::Win32::Graphics::Dxgi::Common::DXGI_FORMAT;

pub struct Builder<Level = ()> {
    adapter: Option<Adapter>,
    min_feature_level: Level,
    name: Option<String>,
}

impl Builder<()> {
    fn new() -> Self {
        Self {
            adapter: None,
            min_feature_level: (),
            name: None,
        }
    }
}

impl<Level> Builder<Level> {
    #[inline]
    pub fn adapter(mut self, adapter: &Adapter) -> Self {
        self.adapter = Some(adapter.clone());
        self
    }

    #[inline]
    pub fn min_feature_level(
        self,
        min_feature_level: D3D_FEATURE_LEVEL,
    ) -> Builder<D3D_FEATURE_LEVEL> {
        Builder {
            adapter: self.adapter,
            min_feature_level,
            name: self.name,
        }
    }

    #[inline]
    pub fn name(mut self, name: impl AsRef<str>) -> Self {
        self.name = Some(name.as_ref().to_string());
        self
    }
}

impl Builder<D3D_FEATURE_LEVEL> {
    #[inline]
    pub fn build(self) -> windows::core::Result<Device> {
        unsafe {
            let handle: ID3D12Device8 = {
                let mut p: Option<ID3D12Device8> = None;
                let adapter: Option<IUnknown> = self.adapter.map(|a| a.handle().clone().into());
                D3D12CreateDevice(adapter.as_ref(), self.min_feature_level, &mut p)
                    .map(|_| p.unwrap())?
            };
            let name = self.name.map(|n| Name::new(&handle, n));
            Ok(Device { handle, name })
        }
    }
}

#[derive(Clone, Debug, Default)]
#[repr(C)]
pub struct SubresourceFootprint {
    pub format: DXGI_FORMAT,
    pub width: u32,
    pub height: u32,
    pub depth: u32,
    pub row_pitch: u32,
}

#[derive(Clone, Debug, Default)]
#[repr(C)]
pub struct PlacedSubresourceFootprint {
    pub offset: u64,
    pub footprint: SubresourceFootprint,
}

impl From<SubresourceFootprint> for D3D12_SUBRESOURCE_FOOTPRINT {
    fn from(value: SubresourceFootprint) -> Self {
        Self {
            Format: value.format,
            Width: value.width,
            Height: value.height,
            Depth: value.depth,
            RowPitch: value.row_pitch,
        }
    }
}

impl From<PlacedSubresourceFootprint> for D3D12_PLACED_SUBRESOURCE_FOOTPRINT {
    fn from(value: PlacedSubresourceFootprint) -> Self {
        Self {
            Offset: value.offset,
            Footprint: value.footprint.into(),
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct ResourceAllocationInfo {
    pub size_in_bytes: u64,
    pub alignment: u64,
}

pub(crate) type DeviceType = ID3D12Device8;

#[derive(Clone, Debug)]
pub struct Device {
    handle: DeviceType,
    name: Option<Name>,
}

impl Device {
    #[inline]
    #[allow(clippy::new_ret_no_self)]
    pub fn new() -> Builder {
        Builder::new()
    }

    #[inline]
    pub fn get_adapter_luid(&self) -> LUID {
        unsafe { self.handle.GetAdapterLuid() }
    }

    #[inline]
    pub fn get_raytracing_acceleration_structure_prebuild_info(
        &self,
        desc: &impl BuildRaytracingAccelerationStructureInputsType,
    ) -> RaytracingAccelerationStructurePrebuildInfo {
        unsafe {
            let mut info = Default::default();
            self.handle
                .GetRaytracingAccelerationStructurePrebuildInfo(desc.get(), &mut info);
            info.into()
        }
    }

    #[inline]
    pub fn check_feature<T: Feature>(&self) -> windows::core::Result<T> {
        T::check(self.handle())
    }

    #[inline]
    pub fn request_feature<T: RequestFeature>(&self, feature: T) -> windows::core::Result<T> {
        T::check(self.handle(), feature)
    }

    #[inline]
    #[allow(clippy::too_many_arguments)]
    pub fn get_copyable_footprints<T>(
        &self,
        resource_desc: &ResourceDesc<T>,
        first_subresource: u32,
        num_subresource: u32,
        base_offset: u64,
        layouts: Option<&mut [PlacedSubresourceFootprint]>,
        num_rows: Option<&mut u32>,
        row_size_in_bytes: Option<&mut u64>,
        total_size: Option<&mut u64>,
    ) {
        assert!(layouts
            .as_ref()
            .map_or(true, |layouts| layouts.len() >= num_subresource as usize));
        unsafe {
            self.handle.GetCopyableFootprints(
                &resource_desc.desc,
                first_subresource,
                num_subresource,
                base_offset,
                layouts
                    .map(|layouts| layouts.as_mut_ptr() as *mut D3D12_PLACED_SUBRESOURCE_FOOTPRINT),
                num_rows.map(|num_rows| num_rows as *mut u32),
                row_size_in_bytes.map(|row_size_in_bytes| row_size_in_bytes as *mut u64),
                total_size.map(|total_size| total_size as *mut u64),
            );
        }
    }

    #[inline]
    pub fn get_resource_allocation_info(
        &self,
        visible_mask: u32,
        descs: &[ResourceDesc],
    ) -> ResourceAllocationInfo {
        unsafe {
            let descs = std::slice::from_raw_parts(
                descs.as_ptr() as *const D3D12_RESOURCE_DESC,
                descs.len(),
            );
            let info = self.handle.GetResourceAllocationInfo(visible_mask, descs);
            ResourceAllocationInfo {
                size_in_bytes: info.SizeInBytes,
                alignment: info.Alignment,
            }
        }
    }

    #[inline]
    pub fn handle(&self) -> &ID3D12Device8 {
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

impl PartialEq for Device {
    fn eq(&self, other: &Self) -> bool {
        self.handle == other.handle
    }
}

impl Eq for Device {}
