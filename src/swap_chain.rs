use super::*;
use windows::core::Interface;
use windows::Win32::Foundation::HWND;
use windows::Win32::Graphics::Dxgi::{Common::*, *};

pub struct Builder<Q = ()> {
    cmd_queue: Q,
    desc: DXGI_SWAP_CHAIN_DESC1,
}

impl Builder<()> {
    fn new() -> Self {
        Self {
            cmd_queue: (),
            desc: DXGI_SWAP_CHAIN_DESC1 {
                Scaling: DXGI_SCALING_STRETCH,
                SampleDesc: DXGI_SAMPLE_DESC {
                    Count: 1,
                    Quality: 0,
                },
                ..Default::default()
            },
        }
    }
}

impl<Q> Builder<Q> {
    #[inline]
    pub fn command_queue(
        self,
        cmd_queue: &CommandQueue<command_list_type::Direct>,
    ) -> Builder<CommandQueue<command_list_type::Direct>> {
        Builder {
            cmd_queue: cmd_queue.clone(),
            desc: DXGI_SWAP_CHAIN_DESC1 { ..self.desc },
        }
    }

    #[inline]
    pub fn width(mut self, width: u32) -> Self {
        self.desc.Width = width;
        self
    }

    #[inline]
    pub fn height(mut self, height: u32) -> Self {
        self.desc.Height = height;
        self
    }

    #[inline]
    pub fn stereo(mut self, stereo: bool) -> Self {
        self.desc.Stereo = stereo.into();
        self
    }

    #[inline]
    pub fn format(mut self, format: DXGI_FORMAT) -> Self {
        self.desc.Format = format;
        self
    }

    #[inline]
    pub fn sample_desc(mut self, desc: DXGI_SAMPLE_DESC) -> Self {
        self.desc.SampleDesc = desc;
        self
    }

    #[inline]
    pub fn buffer_usage(mut self, usage: DXGI_USAGE) -> Self {
        self.desc.BufferUsage = usage;
        self
    }

    #[inline]
    pub fn buffer_count(mut self, count: u32) -> Self {
        self.desc.BufferCount = count;
        self
    }

    #[inline]
    pub fn scaling(mut self, scaling: DXGI_SCALING) -> Self {
        self.desc.Scaling = scaling;
        self
    }

    #[inline]
    pub fn swap_effect(mut self, effect: DXGI_SWAP_EFFECT) -> Self {
        self.desc.SwapEffect = effect;
        self
    }

    #[inline]
    pub fn alpha_mode(mut self, mode: DXGI_ALPHA_MODE) -> Self {
        self.desc.AlphaMode = mode;
        self
    }

    #[inline]
    pub fn flags(mut self, flags: u32) -> Self {
        self.desc.Flags = flags;
        self
    }
}

impl Builder<CommandQueue<command_list_type::Direct>> {
    #[inline]
    pub fn build_for_composition(self) -> windows::core::Result<SwapChain> {
        let factory = dxgi_factory();
        let handle: IDXGISwapChain4 = unsafe {
            factory
                .CreateSwapChainForComposition(self.cmd_queue.handle(), &self.desc, None)?
                .cast()
                .unwrap()
        };

        Ok(SwapChain {
            handle,
            cmd_queue: self.cmd_queue,
        })
    }

    #[inline]
    pub fn build_for_hwnd(self, hwnd: isize) -> windows::core::Result<SwapChain> {
        let factory = dxgi_factory();
        let handle: IDXGISwapChain4 = unsafe {
            factory
                .CreateSwapChainForHwnd(
                    self.cmd_queue.handle(),
                    HWND(hwnd),
                    &self.desc,
                    None,
                    None,
                )?
                .cast()
                .unwrap()
        };
        Ok(SwapChain {
            handle,
            cmd_queue: self.cmd_queue,
        })
    }
}

#[derive(Clone)]
pub struct SwapChain {
    handle: IDXGISwapChain4,
    cmd_queue: CommandQueue<command_list_type::Direct>,
}

impl SwapChain {
    #[inline]
    pub fn new() -> Builder {
        Builder::new()
    }

    #[inline]
    pub fn get_buffer(&self, index: usize) -> windows::core::Result<Resource> {
        unsafe { Ok(Resource::from_raw(self.handle.GetBuffer(index as u32)?)) }
    }

    #[inline]
    pub fn get_current_back_buffer_index(&self) -> usize {
        unsafe { self.handle.GetCurrentBackBufferIndex() as usize }
    }

    #[inline]
    pub fn present(
        &self,
        fence: &Fence,
        interval: u32,
        flags: u32,
    ) -> windows::core::Result<Signal> {
        unsafe {
            self.handle.Present(interval, flags).ok()?;
        }
        Ok(self.cmd_queue.signal(fence)?)
    }
}
