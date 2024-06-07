use std::sync::{Mutex, OnceLock};
use windows::core::{PCSTR, PCWSTR};
use windows::Win32::Foundation::{DBG_PRINTEXCEPTION_C, DBG_PRINTEXCEPTION_WIDE_C};
use windows::Win32::Graphics::Direct3D12::*;
use windows::Win32::System::Diagnostics::Debug::*;

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

static DBG_HANDLERS: OnceLock<Mutex<Vec<Box<(dyn Fn(&str) + Send + Sync + 'static)>>>> =
    OnceLock::new();

fn call_dbg_handlers(msg: &str) {
    let Some(handlers) = DBG_HANDLERS.get().and_then(|handlers| handlers.lock().ok()) else {
        return;
    };
    for handler in handlers.iter() {
        handler(msg);
    }
}

pub fn register_output_debug_string_handler(f: impl Fn(&str) + Send + Sync + 'static) {
    unsafe extern "system" fn handler_proc(info: *mut EXCEPTION_POINTERS) -> i32 {
        let info = info.as_ref().unwrap();
        let record = info.ExceptionRecord.as_ref().unwrap();
        match record.ExceptionCode {
            DBG_PRINTEXCEPTION_C => {
                let psz = PCSTR(record.ExceptionInformation[1] as *const u8);
                if let Ok(msg) = psz.to_string() {
                    call_dbg_handlers(&msg);
                }
                EXCEPTION_CONTINUE_EXECUTION
            }
            DBG_PRINTEXCEPTION_WIDE_C => {
                let psz = PCWSTR(record.ExceptionInformation[1] as *const u16);
                if let Ok(msg) = psz.to_string() {
                    call_dbg_handlers(&msg);
                }
                EXCEPTION_CONTINUE_EXECUTION
            }
            _ => EXCEPTION_CONTINUE_SEARCH,
        }
    }
    let handlers = DBG_HANDLERS.get_or_init(|| {
        unsafe {
            AddVectoredExceptionHandler(1, Some(handler_proc));
        }
        Mutex::new(vec![])
    });
    let mut handlers = handlers.lock().unwrap();
    handlers.push(Box::new(f));
}

#[inline]
pub fn output_debug_string_to_stderr() {
    static LOCK: OnceLock<()> = OnceLock::new();
    LOCK.get_or_init(|| {
        register_output_debug_string_handler(|msg| {
            eprintln!("{msg}");
        });
    });
}
