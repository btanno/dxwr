use super::command_list_type::*;
use super::*;
use windows::Win32::Graphics::Direct3D12::*;

pub struct Builder<T> {
    device: ID3D12Device,
    desc: D3D12_COMMAND_QUEUE_DESC,
    name: Option<String>,
    _t: std::marker::PhantomData<T>,
}

impl<T> Builder<T>
where
    T: CommandListType,
{
    fn new<U>(device: &U) -> Self
    where
        U: Into<ID3D12Device> + Clone,
    {
        let device = device.clone().into();
        Self {
            device,
            desc: D3D12_COMMAND_QUEUE_DESC {
                Type: T::VALUE,
                ..Default::default()
            },
            name: None,
            _t: std::marker::PhantomData,
        }
    }

    #[inline]
    pub fn priority(mut self, priority: i32) -> Self {
        self.desc.Priority = priority;
        self
    }

    #[inline]
    pub fn flags(mut self, flags: D3D12_COMMAND_QUEUE_FLAGS) -> Self {
        self.desc.Flags = flags;
        self
    }

    #[inline]
    pub fn node_mask(mut self, mask: u32) -> Self {
        self.desc.NodeMask = mask;
        self
    }

    #[inline]
    pub fn name(mut self, name: impl AsRef<str>) -> Self {
        self.name = Some(name.as_ref().to_string());
        self
    }

    #[inline]
    pub fn build(self) -> windows::core::Result<CommandQueue<T>> {
        unsafe {
            let handle = self.device.CreateCommandQueue(&self.desc)?;
            let name = self.name.as_ref().map(|n| Name::new(&handle, n));
            Ok(CommandQueue {
                handle,
                name,
                _t: std::marker::PhantomData,
            })
        }
    }
}

#[derive(Clone, Debug)]
pub struct CommandQueue<T = ()> {
    handle: ID3D12CommandQueue,
    name: Option<Name>,
    _t: std::marker::PhantomData<T>,
}

impl CommandQueue<()> {
    #[inline]
    pub fn new_direct(device: &Device) -> Builder<Direct> {
        Builder::new(device.handle())
    }

    #[inline]
    pub fn new_compute(device: &Device) -> Builder<Compute> {
        Builder::new(device.handle())
    }

    #[inline]
    pub fn new_copy(device: &Device) -> Builder<Copy> {
        Builder::new(device.handle())
    }

    #[inline]
    pub fn new_video_decode(device: &Device) -> Builder<VideoDecode> {
        Builder::new(device.handle())
    }

    #[inline]
    pub fn new_video_encode(device: &Device) -> Builder<VideoEncode> {
        Builder::new(device.handle())
    }

    #[inline]
    pub fn new_video_process(device: &Device) -> Builder<VideoProcess> {
        Builder::new(device.handle())
    }
}

impl<T> CommandQueue<T>
where
    T: CommandListType,
{
    #[inline]
    pub fn execute_command_lists(&self, cmd_lists: &[&impl CommandList<T>]) {
        let cmd_lists = cmd_lists
            .iter()
            .map(|l| Some(l.to_raw()))
            .collect::<Vec<Option<ID3D12CommandList>>>();
        unsafe {
            self.handle.ExecuteCommandLists(&cmd_lists);
        }
    }

    #[inline]
    pub fn signal(&self, fence: &Fence) -> windows::core::Result<Signal> {
        unsafe {
            let signal = Signal::new(fence);
            self.handle
                .Signal(signal.fence().handle(), signal.value())?;
            Ok(signal)
        }
    }

    #[inline]
    pub fn wait(&self, signal: &Signal) -> windows::core::Result<()> {
        unsafe { self.handle.Wait(signal.fence().handle(), signal.value()) }
    }

    #[inline]
    pub fn handle(&self) -> &ID3D12CommandQueue {
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

impl CommandQueue<command_list_type::Direct> {
    #[inline]
    #[allow(clippy::new_ret_no_self)]
    pub fn new(device: &Device) -> Builder<command_list_type::Direct> {
        Builder::new(device.handle())
    }
}

impl CommandQueue<command_list_type::Compute> {
    #[inline]
    #[allow(clippy::new_ret_no_self)]
    pub fn new(device: &Device) -> Builder<command_list_type::Compute> {
        Builder::new(device.handle())
    }
}

impl CommandQueue<command_list_type::Copy> {
    #[inline]
    #[allow(clippy::new_ret_no_self)]
    pub fn new(device: &Device) -> Builder<command_list_type::Copy> {
        Builder::new(device.handle())
    }
}

impl CommandQueue<command_list_type::VideoDecode> {
    #[inline]
    #[allow(clippy::new_ret_no_self)]
    pub fn new(device: &Device) -> Builder<command_list_type::VideoDecode> {
        Builder::new(device.handle())
    }
}

impl CommandQueue<command_list_type::VideoEncode> {
    #[inline]
    #[allow(clippy::new_ret_no_self)]
    pub fn new(device: &Device) -> Builder<command_list_type::VideoEncode> {
        Builder::new(device.handle())
    }
}

impl CommandQueue<command_list_type::VideoProcess> {
    #[inline]
    #[allow(clippy::new_ret_no_self)]
    pub fn new(device: &Device) -> Builder<command_list_type::VideoProcess> {
        Builder::new(device.handle())
    }
}

pub type DirectCommandQueue = CommandQueue<command_list_type::Direct>;
pub type ComputeCommandQueue = CommandQueue<command_list_type::Compute>;
pub type CopyCommandQueue = CommandQueue<command_list_type::Copy>;
pub type VideoDecodeCommandQueue = CommandQueue<command_list_type::VideoDecode>;
pub type VideoEncodeCommandQueue = CommandQueue<command_list_type::VideoEncode>;
pub type VideoProcessCommandQueue = CommandQueue<command_list_type::VideoProcess>;
