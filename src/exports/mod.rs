mod combat;

use crate::device::send_to_device;
use arcdps_bindings::*;

pub fn imgui(not_charsel_or_loading: bool) {
    send_to_device([1, not_charsel_or_loading as u8].to_vec());
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn loop_imgui() {
        crate::device::gen_device();
        let sleep = std::time::Duration::from_millis(500);
        let mut param = false;
        loop {
            imgui(param);
            param = !param;
            std::thread::sleep(sleep);
        }
    }
}
