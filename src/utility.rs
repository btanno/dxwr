use std::sync::Arc;
use std::sync::OnceLock;
use std::time::Duration;
use windows::core::HSTRING;
use windows::Win32::Foundation::{CloseHandle, HANDLE, WAIT_OBJECT_0};
use windows::Win32::Graphics::Direct3D12::*;
use windows::Win32::Graphics::Dxgi::{Common::*, *};
use windows::Win32::System::Threading::{CreateEventW, WaitForSingleObject, INFINITE};

#[derive(Clone, Copy, Debug)]
#[repr(transparent)]
pub struct Rect(windows::Win32::Foundation::RECT);

impl Rect {
    #[inline]
    pub fn new() -> Self {
        Self(Default::default())
    }

    #[inline]
    pub fn left(mut self, v: i32) -> Self {
        self.0.left = v;
        self
    }

    #[inline]
    pub fn top(mut self, v: i32) -> Self {
        self.0.top = v;
        self
    }

    #[inline]
    pub fn right(mut self, v: i32) -> Self {
        self.0.right = v;
        self
    }

    #[inline]
    pub fn bottom(mut self, v: i32) -> Self {
        self.0.bottom = v;
        self
    }
}

pub(crate) fn as_rect_slice(src: &[Rect]) -> &[windows::Win32::Foundation::RECT] {
    unsafe { std::slice::from_raw_parts(src.as_ptr() as *const _, src.len()) }
}

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

#[derive(Clone, PartialEq, Eq, Debug)]
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

#[derive(Clone, Copy, PartialEq, Default, Debug)]
#[repr(transparent)]
pub struct GpuVirtualAddressRange(pub D3D12_GPU_VIRTUAL_ADDRESS_RANGE);

impl GpuVirtualAddressRange {
    #[inline]
    pub fn new() -> Self {
        Self(D3D12_GPU_VIRTUAL_ADDRESS_RANGE::default())
    }

    #[inline]
    pub fn start_address(mut self, addr: GpuVirtualAddress) -> Self {
        self.0.StartAddress = addr.0;
        self
    }

    #[inline]
    pub fn size_in_bytes(mut self, size: u64) -> Self {
        self.0.SizeInBytes = size;
        self
    }
}

#[derive(Clone, Copy, PartialEq, Default, Debug)]
#[repr(transparent)]
pub struct GpuVirtualAddressRangeAndStride(pub D3D12_GPU_VIRTUAL_ADDRESS_RANGE_AND_STRIDE);

impl GpuVirtualAddressRangeAndStride {
    #[inline]
    pub fn new() -> Self {
        Self(D3D12_GPU_VIRTUAL_ADDRESS_RANGE_AND_STRIDE::default())
    }

    #[inline]
    pub fn start_address(mut self, addr: GpuVirtualAddress) -> Self {
        self.0.StartAddress = addr.0;
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

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default, Debug)]
pub struct GpuVirtualAddress(pub u64);

impl GpuVirtualAddress {
    #[inline]
    pub fn offset(self, v: i64) -> Self {
        if v < 0 {
            Self(self.0 - (-v as u64))
        } else {
            Self(self.0 + v as u64)
        }
    }
}

impl From<u64> for GpuVirtualAddress {
    #[inline]
    fn from(value: u64) -> Self {
        Self(value)
    }
}

#[derive(Clone, Copy, PartialEq, Default, Debug)]
#[repr(transparent)]
pub struct GpuVirtualAddressAndStride(pub D3D12_GPU_VIRTUAL_ADDRESS_AND_STRIDE);

impl GpuVirtualAddressAndStride {
    #[inline]
    pub fn new() -> Self {
        Self(D3D12_GPU_VIRTUAL_ADDRESS_AND_STRIDE::default())
    }

    #[inline]
    pub fn start_address(mut self, addr: GpuVirtualAddress) -> Self {
        self.0.StartAddress = addr.0;
        self
    }

    #[inline]
    pub fn stride_in_bytes(mut self, stride: u64) -> Self {
        self.0.StrideInBytes = stride;
        self
    }
}

pub struct Handle(HANDLE);

impl Handle {
    #[inline]
    pub fn new(handle: HANDLE) -> Self {
        Self(handle)
    }

    #[inline]
    pub fn wait(&self) {
        unsafe {
            WaitForSingleObject(self.0, INFINITE);
        }
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

impl Drop for Handle {
    fn drop(&mut self) {
        unsafe {
            CloseHandle(self.0).ok();
        }
    }
}

#[inline]
pub fn align_size(size: u64, align: u64) -> u64 {
    (size + (align - 1)) & !(align - 1)
}

#[inline]
pub fn align_size_for_constant_buffer(size: u64) -> u64 {
    align_size(size, D3D12_CONSTANT_BUFFER_DATA_PLACEMENT_ALIGNMENT as u64)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn align_size_test() {
        assert!(align_size(1, 256) == 256);
        assert!(align_size(256, 256) == 256);
        assert!(align_size(257, 256) == 256 * 2);
        assert!(align_size(256 * 2 + 1, 256) == 256 * 3);
    }
}
