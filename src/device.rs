use super::raytracing::BuildRaytracingAccelerationStructureInputsType;
use super::*;
use windows::core::IUnknown;
use windows::Win32::Foundation::LUID;
use windows::Win32::Graphics::Direct3D::*;
use windows::Win32::Graphics::Direct3D12::*;

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

#[derive(Clone, Debug)]
pub struct Device {
    handle: ID3D12Device8,
    name: Option<Name>,
}

impl Device {
    #[inline]
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
