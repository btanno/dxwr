use super::*;
use crate::resources::ShareableHandle;
use std::sync::{
    Arc,
    atomic::{self, AtomicU64},
};
use std::time::Duration;
use windows::Win32::Foundation::HANDLE;
use windows::Win32::Graphics::Direct3D12::*;
use windows::core::Interface;

pub struct Builder {
    device: ID3D12Device,
    flags: D3D12_FENCE_FLAGS,
    name: Option<String>,
}

impl Builder {
    fn new<T>(device: &T) -> Self
    where
        T: Into<ID3D12Device> + Clone,
    {
        Self {
            device: device.clone().into(),
            flags: D3D12_FENCE_FLAG_NONE,
            name: None,
        }
    }

    #[inline]
    pub fn flags(mut self, flags: D3D12_FENCE_FLAGS) -> Self {
        self.flags = flags;
        self
    }

    #[inline]
    pub fn name(mut self, name: impl AsRef<str>) -> Self {
        self.name = Some(name.as_ref().to_string());
        self
    }

    #[inline]
    pub fn build(self) -> windows::core::Result<Fence> {
        let handle: ID3D12Fence = unsafe { self.device.CreateFence(0, self.flags)? };
        let next_value = Arc::new(AtomicU64::new(1));
        let name = self.name.as_ref().map(|n| Name::new(&handle, n));
        Ok(Fence {
            handle,
            next_value,
            name,
        })
    }
}

#[derive(Clone, Debug)]
pub struct Fence {
    handle: ID3D12Fence,
    next_value: Arc<AtomicU64>,
    name: Option<Name>,
}

impl Fence {
    #[inline]
    #[allow(clippy::new_ret_no_self)]
    pub fn new(device: &Device) -> Builder {
        Builder::new(device.handle())
    }

    #[inline]
    pub fn signal(&self, signal: &Signal) -> windows::core::Result<()> {
        unsafe {
            if self.handle.GetCompletedValue() < signal.value {
                self.handle.Signal(signal.value)
            } else {
                Ok(())
            }
        }
    }

    #[inline]
    pub fn handle(&self) -> &ID3D12Fence {
        &self.handle
    }

    #[inline]
    pub fn name(&self) -> Option<&str> {
        self.name.as_ref().map(|n| n.as_str())
    }
}

impl PartialEq for Fence {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.handle == other.handle
    }
}

impl Eq for Fence {}

impl ShareableHandle for Fence {
    #[inline]
    fn as_device_child(&self) -> ID3D12DeviceChild {
        self.handle.cast().unwrap()
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Signal {
    fence: Fence,
    value: u64,
}

impl Signal {
    #[inline]
    pub fn new(fence: &Fence) -> Self {
        let value = fence.next_value.fetch_add(1, atomic::Ordering::SeqCst);
        Self {
            fence: fence.clone(),
            value,
        }
    }

    #[inline]
    pub fn wait(&self) -> windows::core::Result<()> {
        unsafe {
            if self.fence.handle.GetCompletedValue() < self.value {
                self.fence
                    .handle
                    .SetEventOnCompletion(self.value, HANDLE::default())?;
            }
        }
        Ok(())
    }

    #[inline]
    pub fn wait_timeout(&self, d: Duration) -> windows::core::Result<bool> {
        unsafe {
            if self.fence.handle.GetCompletedValue() < self.value {
                let event = EventHandle::new()?;
                self.fence
                    .handle
                    .SetEventOnCompletion(self.value, event.handle())?;
                return event.wait_timeout(d);
            }
        }
        Ok(true)
    }

    #[inline]
    pub fn is_completed(&self) -> bool {
        unsafe { self.fence.handle.GetCompletedValue() >= self.value }
    }

    #[inline]
    pub fn fence(&self) -> &Fence {
        &self.fence
    }

    #[inline]
    pub fn value(&self) -> u64 {
        self.value
    }
}
