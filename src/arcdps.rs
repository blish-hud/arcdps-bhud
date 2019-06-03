use crate::{exports::*, main, release};
use arcdps_bindings;
use std::ptr::null;
use winapi::shared::{minwindef::LPVOID, ntdef::PCCHAR};

static mut ARCDPS: LPVOID = null::<isize>() as LPVOID;

pub fn gen_arcdps() -> LPVOID {
    let arcdps = arcdps_bindings::arcdps_exports::new(0x0002_0804, "BHUDrender", "0.1")
        .imgui(imgui as arcdps_bindings::SafeImguiCallback)
        .combat(combat as arcdps_bindings::SafeCombatCallback);

    unsafe {
        ARCDPS = &arcdps as *const arcdps_bindings::arcdps_exports as LPVOID;
        ARCDPS
    }
}

#[no_mangle]
/* export -- arcdps looks for this exported function and calls the address it returns on client load */
pub extern "system" fn get_init_addr(
    _arcversionstr: PCCHAR,
    _imguicontext: LPVOID,
    _id3dd9: LPVOID,
) -> LPVOID {
    main as LPVOID
}

#[no_mangle]
/* export -- arcdps looks for this exported function and calls the address it returns on client exit */
pub extern "system" fn get_release_addr() -> LPVOID {
    release as LPVOID
}
