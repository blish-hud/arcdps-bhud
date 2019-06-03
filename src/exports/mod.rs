mod combat;

use crate::{device, pipeline};
use arcdps_bindings::*;

pub fn imgui(not_charsel_or_loading: bool) {
    if not_charsel_or_loading {
        let _ = device::send_to_device(|| pipeline::Arc {
            msgtype: pipeline::Mtype::Imgui as i32,
            msg: Some(pipeline::arc::Msg::Imgui(true)),
        });
    }
}

pub fn combat(
    ev: Option<&cbtevent>,
    src: Option<&Ag>,
    dst: Option<&Ag>,
    skillname: Option<&str>,
    id: u64,
    revision: u64,
) {
    combat::cbt(ev, src, dst, skillname, id, revision);
}
