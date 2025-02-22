use std::sync::{LazyLock, Mutex, OnceLock};
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

fn call_dbg_handlers(msg: &str) {
    let Some(handlers) = DBG_HANDLERS.lock().ok() else {
        return;
    };
    for handler in handlers.iter() {
        handler(msg);
    }
}

unsafe extern "system" fn exception_handler_proc(pointers: *mut EXCEPTION_POINTERS) -> i32 {
    unsafe {
        let Some(pointers) = pointers.as_ref() else {
            return EXCEPTION_CONTINUE_EXECUTION;
        };
        let Some(record) = pointers.ExceptionRecord.as_ref() else {
            return EXCEPTION_CONTINUE_EXECUTION;
        };
        if record.NumberParameters < 2 {
            return EXCEPTION_CONTINUE_SEARCH;
        }
        match record.ExceptionCode {
            DBG_PRINTEXCEPTION_C => {
                let len = record.ExceptionInformation[0];
                if len >= 2 {
                    let data = record.ExceptionInformation[1] as *const u8;
                    let s = std::slice::from_raw_parts(data, len - 1);
                    if let Ok(msg) = std::str::from_utf8(s) {
                        call_dbg_handlers(msg);
                    }
                }
            }
            DBG_PRINTEXCEPTION_WIDE_C => {
                let len = record.ExceptionInformation[0];
                if len >= 2 {
                    let data = record.ExceptionInformation[1] as *const u16;
                    let s = std::slice::from_raw_parts(data, len - 1);
                    if let Ok(msg) = String::from_utf16(s) {
                        call_dbg_handlers(&msg);
                    }
                }
            }
            _ => {}
        }
        EXCEPTION_CONTINUE_SEARCH
    }
}

type DebugHandler = Box<(dyn Fn(&str) + Send + Sync + 'static)>;
static DBG_HANDLERS: LazyLock<Mutex<Vec<DebugHandler>>> = LazyLock::new(|| {
    unsafe {
        AddVectoredExceptionHandler(0, Some(exception_handler_proc));
    }
    Mutex::new(vec![])
});

pub fn register_output_debug_string_handler(f: impl Fn(&str) + Send + Sync + 'static) {
    let mut handlers = DBG_HANDLERS.lock().unwrap();
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

#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
pub enum DredEnablement {
    SystemControlled,
    ForceOn,
    ForceOff,
}

impl Default for DredEnablement {
    #[inline]
    fn default() -> Self {
        Self::SystemControlled
    }
}

impl From<DredEnablement> for D3D12_DRED_ENABLEMENT {
    #[inline]
    fn from(value: DredEnablement) -> Self {
        match value {
            DredEnablement::SystemControlled => D3D12_DRED_ENABLEMENT_SYSTEM_CONTROLLED,
            DredEnablement::ForceOn => D3D12_DRED_ENABLEMENT_FORCED_ON,
            DredEnablement::ForceOff => D3D12_DRED_ENABLEMENT_FORCED_OFF,
        }
    }
}

pub struct DeviceRemovedExtendedDataSettings(ID3D12DeviceRemovedExtendedDataSettings1);

impl DeviceRemovedExtendedDataSettings {
    #[inline]
    pub fn new() -> windows::core::Result<Self> {
        let settings: ID3D12DeviceRemovedExtendedDataSettings1 = unsafe {
            let mut p: Option<_> = None;
            D3D12GetDebugInterface(&mut p).map(|_| p.unwrap())?
        };
        Ok(Self(settings))
    }

    #[inline]
    pub fn set_auto_breadcrumbs_enablement(self, enablement: DredEnablement) -> Self {
        unsafe {
            self.0.SetAutoBreadcrumbsEnablement(enablement.into());
        }
        self
    }

    #[inline]
    pub fn set_page_fault_enablement(self, enablement: DredEnablement) -> Self {
        unsafe {
            self.0.SetPageFaultEnablement(enablement.into());
        }
        self
    }

    #[inline]
    pub fn set_watson_dump_enablement(self, enablement: DredEnablement) -> Self {
        unsafe {
            self.0.SetWatsonDumpEnablement(enablement.into());
        }
        self
    }

    #[inline]
    pub fn set_breadcrumb_context_enablement(self, enablement: DredEnablement) -> Self {
        unsafe {
            self.0.SetBreadcrumbContextEnablement(enablement.into());
        }
        self
    }
}
