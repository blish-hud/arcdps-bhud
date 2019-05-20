mod worker;

use arcdps_bindings;
use std::ptr::null;
use winapi::shared::{minwindef::LPVOID, ntdef::PCCHAR};
use worker::Device;

static mut ARCDPS: LPVOID = null::<isize>() as LPVOID;
static mut DEVICE: Option<Device> = None;

fn main() -> LPVOID {
    let arcdps = arcdps_bindings::arcdps_exports::new(
        0x020804,
        "BHUDrender",
        "0.1",
        None,
        None,
        Some(imgui as arcdps_bindings::ImguiCallback),
        None,
        None,
        None,
        None,
    );
    unsafe {
        DEVICE = Some(Device::new("BHUDrender"));
        ARCDPS = &arcdps as *const arcdps_bindings::arcdps_exports as LPVOID;
        std::mem::forget(arcdps);
        ARCDPS
    }
}

fn release() {
    unsafe {
        let arcdps: arcdps_bindings::arcdps_exports =
            *(ARCDPS as *const arcdps_bindings::arcdps_exports);
        drop(arcdps);
    };
}

fn imgui(not_charsel_or_loading: u32) -> usize {
    if not_charsel_or_loading > 0 {
        unsafe {
            if let Some(d) = &DEVICE {
                let _ = d.send();
            };
        }
    }
    0
}

#[no_mangle]
/* export -- arcdps looks for this exported function and calls the address it returns on client load */
pub extern "system" fn get_init_addr(
    _arcversionstr: PCCHAR,
    _mguicontext: LPVOID,
    _id3dd9: LPVOID,
) -> LPVOID {
    main as LPVOID
}

#[no_mangle]
/* export -- arcdps looks for this exported function and calls the address it returns on client exit */
pub extern "system" fn get_release_addr() -> LPVOID {
    release as LPVOID
}
