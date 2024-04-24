use windows::Win32::Graphics::Direct3D12::*;

pub trait CommandListType {
    const VALUE: D3D12_COMMAND_LIST_TYPE;
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Direct;

impl CommandListType for Direct {
    const VALUE: D3D12_COMMAND_LIST_TYPE = D3D12_COMMAND_LIST_TYPE_DIRECT;
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Bundle;

impl CommandListType for Bundle {
    const VALUE: D3D12_COMMAND_LIST_TYPE = D3D12_COMMAND_LIST_TYPE_BUNDLE;
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Compute;

impl CommandListType for Compute {
    const VALUE: D3D12_COMMAND_LIST_TYPE = D3D12_COMMAND_LIST_TYPE_COMPUTE;
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Copy;

impl CommandListType for Copy {
    const VALUE: D3D12_COMMAND_LIST_TYPE = D3D12_COMMAND_LIST_TYPE_COPY;
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct VideoDecode;

impl CommandListType for VideoDecode {
    const VALUE: D3D12_COMMAND_LIST_TYPE = D3D12_COMMAND_LIST_TYPE_VIDEO_DECODE;
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct VideoProcess;

impl CommandListType for VideoProcess {
    const VALUE: D3D12_COMMAND_LIST_TYPE = D3D12_COMMAND_LIST_TYPE_VIDEO_PROCESS;
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct VideoEncode;

impl CommandListType for VideoEncode {
    const VALUE: D3D12_COMMAND_LIST_TYPE = D3D12_COMMAND_LIST_TYPE_VIDEO_ENCODE;
}
