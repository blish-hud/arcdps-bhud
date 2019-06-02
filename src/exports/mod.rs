mod combat;

use crate::{device, pipeline};
use arcdps_bindings::*;
use std::ffi::CString;
use winapi::shared::ntdef::PCCHAR;

pub fn imgui(not_charsel_or_loading: u32) -> usize {
    if not_charsel_or_loading > 0 {
        let _ = device::send_to_device(|| pipeline::Arc {
            msgtype: pipeline::Mtype::Imgui as i32,
            msg: Some(pipeline::arc::Msg::Imgui(true)),
        });
    }
    0
}

pub fn combat(
    ev: *mut cbtevent,
    src: *mut ag,
    dst: *mut ag,
    skillname: PCCHAR,
    id: u64,
    revision: u64,
) -> usize {
    let s_ev: &cbtevent;
    let s_src: &ag;
    let s_dst: &ag;
    let s_skillname: String;
    unsafe {
        s_ev = &*ev;
        s_src = &*src;
        s_dst = &*dst;
        s_skillname = CString::from_raw(skillname)
            .into_string()
            .unwrap_or_default();
    }
    combat::cbt(&s_ev, &s_src, &s_dst, s_skillname, id, revision);
    0
}
