use crate::{device, pipeline};
use arcdps_bindings::*;
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
    _ev: *mut cbtevent,
    _src: *mut ag,
    _dst: *mut ag,
    _skillname: PCCHAR,
    _id: u64,
    _revision: u64,
) -> usize {
    0
}
