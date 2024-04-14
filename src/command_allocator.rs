use super::command_list_type::*;
use super::*;
use windows::Win32::Graphics::Direct3D12::*;

pub struct Builder<T> {
    device: ID3D12Device,
    name: Option<String>,
    _t: std::marker::PhantomData<T>,
}

impl<T> Builder<T>
where
    T: Type,
{
    fn new<U>(device: &U) -> Self
    where
        U: Into<ID3D12Device> + Clone,
    {
        Self {
            device: device.clone().into(),
            name: None,
            _t: std::marker::PhantomData,
        }
    }

    #[inline]
    pub fn name(mut self, name: impl AsRef<str>) -> Self {
        self.name = Some(name.as_ref().to_string());
        self
    }

    #[inline]
    pub fn build(self) -> windows::core::Result<CommandAllocator<T>> {
        let device: ID3D12Device = self.device.clone();
        unsafe {
            let handle = device.CreateCommandAllocator(T::VALUE)?;
            let name = self.name.as_ref().map(|n| Name::new(&handle, n));
            Ok(CommandAllocator {
                handle,
                name,
                _t: std::marker::PhantomData,
            })
        }
    }
}

#[derive(Clone, Debug)]
pub struct CommandAllocator<T> {
    handle: ID3D12CommandAllocator,
    name: Option<Name>,
    _t: std::marker::PhantomData<T>,
}

impl<T> CommandAllocator<T>
where
    T: Type,
{
    #[inline]
    pub fn new(device: &Device, _ty: T) -> Builder<T> {
        Builder::new(device.handle())
    }

    #[inline]
    pub fn handle(&self) -> &ID3D12CommandAllocator {
        &self.handle
    }

    #[inline]
    pub fn name(&self) -> Option<&str> {
        self.name.as_ref().map(|n| n.as_str())
    }
}
