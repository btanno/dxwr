use std::sync::Arc;
use std::sync::OnceLock;
use std::time::Duration;
use windows::core::HSTRING;
use windows::Win32::Foundation::{CloseHandle, HANDLE, WAIT_OBJECT_0};
use windows::Win32::Graphics::Direct3D12::*;
use windows::Win32::Graphics::Dxgi::{Common::*, *};
use windows::Win32::System::Threading::{CreateEventW, WaitForSingleObject};

pub(crate) struct EventHandle(HANDLE);

impl EventHandle {
    #[inline]
    pub fn new() -> windows::core::Result<Self> {
        unsafe { Ok(Self(CreateEventW(None, true, false, None)?)) }
    }

    #[inline]
    pub fn wait_timeout(&self, d: Duration) -> windows::core::Result<bool> {
        let d = d.as_millis();
        assert!(d <= u32::MAX as u128);
        unsafe { Ok(WaitForSingleObject(self.0, d as u32) == WAIT_OBJECT_0) }
    }

    #[inline]
    pub fn handle(&self) -> HANDLE {
        self.0
    }
}

impl Drop for EventHandle {
    fn drop(&mut self) {
        unsafe {
            CloseHandle(self.0).ok();
        }
    }
}

#[derive(Clone, Debug)]
pub(crate) struct Name(Arc<String>);

impl Name {
    pub fn new<T>(object: &T, name: impl AsRef<str>) -> Self
    where
        T: Into<ID3D12Object> + Clone,
    {
        let object: ID3D12Object = object.clone().into();
        unsafe {
            object.SetName(&HSTRING::from(name.as_ref())).ok();
        }
        Self(Arc::new(name.as_ref().to_string()))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[inline]
pub fn dxgi_factory() -> &'static IDXGIFactory7 {
    static FACTORY: OnceLock<IDXGIFactory7> = OnceLock::new();
    FACTORY.get_or_init(|| unsafe { CreateDXGIFactory1().unwrap() })
}

#[derive(Clone, Copy, Debug)]
#[repr(transparent)]
pub struct SampleDesc(pub DXGI_SAMPLE_DESC);

impl SampleDesc {
    #[inline]
    pub fn new(count: u32, quality: u32) -> Self {
        Self(DXGI_SAMPLE_DESC {
            Count: count,
            Quality: quality,
        })
    }
}

impl Default for SampleDesc {
    #[inline]
    fn default() -> Self {
        Self::new(1, 0)
    }
}

#[derive(Clone, Copy, Debug)]
#[repr(transparent)]
pub struct GpuVirtualAddressRange(pub D3D12_GPU_VIRTUAL_ADDRESS_RANGE);

impl GpuVirtualAddressRange {
    #[inline]
    pub fn new() -> Self {
        Self(D3D12_GPU_VIRTUAL_ADDRESS_RANGE::default())
    }

    #[inline]
    pub fn start_address(mut self, addr: u64) -> Self {
        self.0.StartAddress = addr;
        self
    }

    #[inline]
    pub fn size_in_bytes(mut self, size: u64) -> Self {
        self.0.SizeInBytes = size;
        self
    }
}

#[derive(Clone, Copy, Debug)]
#[repr(transparent)]
pub struct GpuVirtualAddressRangeAndStride(pub D3D12_GPU_VIRTUAL_ADDRESS_RANGE_AND_STRIDE);

impl GpuVirtualAddressRangeAndStride {
    #[inline]
    pub fn new() -> Self {
        Self(D3D12_GPU_VIRTUAL_ADDRESS_RANGE_AND_STRIDE::default())
    }

    #[inline]
    pub fn start_address(mut self, addr: u64) -> Self {
        self.0.StartAddress = addr;
        self
    }

    #[inline]
    pub fn size_in_bytes(mut self, size: u64) -> Self {
        self.0.SizeInBytes = size;
        self
    }

    #[inline]
    pub fn stride_in_bytes(mut self, size: u64) -> Self {
        self.0.StrideInBytes = size;
        self
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct GpuVirtualAddress(pub u64);

#[derive(Clone, Copy, Debug)]
#[repr(transparent)]
pub struct GpuVirtualAddressAndStride(pub D3D12_GPU_VIRTUAL_ADDRESS_AND_STRIDE);

impl GpuVirtualAddressAndStride {
    #[inline]
    pub fn new() -> Self {
        Self(D3D12_GPU_VIRTUAL_ADDRESS_AND_STRIDE::default())
    }

    #[inline]
    pub fn start_address(mut self, addr: u64) -> Self {
        self.0.StartAddress = addr;
        self
    }

    #[inline]
    pub fn stride_in_bytes(mut self, stride: u64) -> Self {
        self.0.StrideInBytes = stride;
        self
    }
}

