use crate::{device, pipeline};

pub fn imgui(not_charsel_or_loading: u32) -> usize {
    if not_charsel_or_loading > 0 {
        let _ = device::send_to_device(|| pipeline::arc {
            msgtype: pipeline::mtype::Imgui,
            msg: pipeline::mod_arc::OneOfmsg::imgui(true),
        });
    }
    0
}
