use windows::Win32::Graphics::Direct3D12::*;

#[inline]
pub fn enable_debug_layer() -> windows::core::Result<()> {
    unsafe {
        let debug: ID3D12Debug = {
            let mut p: Option<ID3D12Debug> = None;
            D3D12GetDebugInterface(&mut p).map(|_| p.unwrap())?
        };
        debug.EnableDebugLayer();
    }
    Ok(())
}
