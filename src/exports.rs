use crate::{device, pipeline};

pub fn imgui(not_charsel_or_loading: u32) -> usize {
    if not_charsel_or_loading > 0 {
        let _ = device::send_to_device(|| pipeline::Arc {
            msgtype: pipeline::Mtype::Imgui as i32,
            msg: Some(pipeline::arc::Msg::Imgui(true)),
        });
    }
    0
}
