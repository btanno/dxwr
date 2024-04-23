use super::raytracing::BuildRaytracingAccelerationStructureInputsType;
use super::*;
use windows::core::IUnknown;
use windows::Win32::Foundation::LUID;
use windows::Win32::Graphics::Direct3D::*;
use windows::Win32::Graphics::Direct3D12::*;

#[derive(Clone, Debug)]
pub struct Device {
    handle: ID3D12Device8,
    name: Option<Name>,
}

impl Device {
    #[inline]
    pub fn new(
        adapter: Option<&Adapter>,
        min_feature_level: D3D_FEATURE_LEVEL,
        name: Option<&str>,
    ) -> windows::core::Result<Self> {
        unsafe {
            let handle: ID3D12Device8 = {
                let mut p: Option<ID3D12Device8> = None;
                let adapter: Option<IUnknown> = adapter.map(|a| a.handle().clone().into());
                D3D12CreateDevice(adapter.as_ref(), min_feature_level, &mut p)
                    .map(|_| p.unwrap())?
            };
            let name = name.map(|n| Name::new(&handle, n));
            Ok(Self { handle, name })
        }
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
    pub fn handle(&self) -> &ID3D12Device8 {
        &self.handle
    }

    #[inline]
    pub fn name(&self) -> Option<&str> {
        self.name.as_ref().map(|n| n.as_str())
    }
}

impl PartialEq for Device {
    fn eq(&self, other: &Self) -> bool {
        self.handle == other.handle
    }
}

impl Eq for Device {}
