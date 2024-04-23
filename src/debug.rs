use windows::Win32::Graphics::Direct3D12::*;
use windows::Win32::System::Diagnostics::Debug::*;
use windows::core::{PCSTR, PCWSTR};
use windows::Win32::Foundation::{DBG_PRINTEXCEPTION_C, DBG_PRINTEXCEPTION_WIDE_C};

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

pub fn output_debug_string_to_stderr() {
    unsafe extern "system" fn handler(info: *mut EXCEPTION_POINTERS) -> i32 {
        let info = info.as_ref().unwrap();
        let record = info.ExceptionRecord.as_ref().unwrap();
        match record.ExceptionCode {
            DBG_PRINTEXCEPTION_C => {
                let psz = PCSTR(record.ExceptionInformation[1] as *const u8);
                if let Ok(msg) = psz.to_string() {
                    eprintln!("{msg}");
                }
                EXCEPTION_CONTINUE_EXECUTION
            }
            DBG_PRINTEXCEPTION_WIDE_C => {
                let psz = PCWSTR(record.ExceptionInformation[1] as *const u16);
                if let Ok(msg) = psz.to_string() {
                    eprintln!("{msg}");
                }
                EXCEPTION_CONTINUE_EXECUTION
            }
            _ => EXCEPTION_CONTINUE_SEARCH
        }
    }

    unsafe {
        AddVectoredExceptionHandler(1, Some(handler));
    }
}
