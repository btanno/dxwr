use super::utility::dxgi_factory;
use std::sync::Arc;
use windows::core::Interface;
use windows::Win32::Foundation::LUID;
use windows::Win32::Graphics::Dxgi::*;

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct AdapterId {
    pub vendor_id: u32,
    pub device_id: u32,
    pub sub_sys_id: u32,
    pub revision: u32,
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct AdapterMemoryInfo {
    pub dedicate_video_memory: usize,
    pub dedicate_system_memory: usize,
    pub shared_system_memory: usize,
}

#[derive(Clone, Debug)]
pub struct Adapter {
    handle: IDXGIAdapter4,
    desc: Arc<DXGI_ADAPTER_DESC3>,
}

impl Adapter {
    #[inline]
    pub fn description(&self) -> String {
        let len = self
            .desc
            .Description
            .iter()
            .position(|d| *d == 0)
            .unwrap_or(self.desc.Description.len());
        String::from_utf16_lossy(&self.desc.Description[..len])
    }

    #[inline]
    pub fn id(&self) -> AdapterId {
        AdapterId {
            vendor_id: self.desc.VendorId,
            device_id: self.desc.DeviceId,
            sub_sys_id: self.desc.SubSysId,
            revision: self.desc.Revision,
        }
    }

    #[inline]
    pub fn memory_info(&self) -> AdapterMemoryInfo {
        AdapterMemoryInfo {
            dedicate_video_memory: self.desc.DedicatedVideoMemory,
            dedicate_system_memory: self.desc.DedicatedSystemMemory,
            shared_system_memory: self.desc.SharedSystemMemory,
        }
    }

    #[inline]
    pub fn flags(&self) -> DXGI_ADAPTER_FLAG3 {
        self.desc.Flags
    }

    #[inline]
    pub fn luid(&self) -> LUID {
        self.desc.AdapterLuid
    }

    #[inline]
    pub fn graphics_preemption_granularity(&self) -> DXGI_GRAPHICS_PREEMPTION_GRANULARITY {
        self.desc.GraphicsPreemptionGranularity
    }

    #[inline]
    pub fn compute_preemption_granularity(&self) -> DXGI_COMPUTE_PREEMPTION_GRANULARITY {
        self.desc.ComputePreemptionGranularity
    }

    #[inline]
    pub fn handle(&self) -> &IDXGIAdapter4 {
        &self.handle
    }
}

pub fn enum_adapters() -> windows::core::Result<Vec<Adapter>> {
    let factory = dxgi_factory();
    let mut adapters = vec![];
    let mut index = 0;
    unsafe {
        loop {
            let ret = factory.EnumAdapters1(index);
            match ret {
                Ok(adapter) => {
                    let handle = adapter.cast::<IDXGIAdapter4>().unwrap();
                    let desc = handle.GetDesc3()?;
                    adapters.push(Adapter {
                        handle,
                        desc: Arc::new(desc),
                    });
                    index += 1;
                }
                Err(e) if e.code() == DXGI_ERROR_NOT_FOUND => break,
                Err(e) => return Err(e),
            }
        }
    }
    Ok(adapters)
}

#[inline]
pub fn enum_warp_adapter() -> windows::core::Result<Adapter> {
    let factory = dxgi_factory();
    let handle: IDXGIAdapter4 = unsafe { factory.EnumWarpAdapter()? };
    let desc = unsafe { handle.GetDesc3()? };
    Ok(Adapter {
        handle,
        desc: Arc::new(desc),
    })
}
