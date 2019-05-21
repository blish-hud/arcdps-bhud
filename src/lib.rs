mod arcdps;
mod device;
mod exports;
mod worker;

pub mod pipeline {
    include!(concat!(env!("OUT_DIR"), "/pipeline.rs"));
}

use winapi::shared::minwindef::LPVOID;

fn main() -> LPVOID {
    device::gen_device();
    arcdps::gen_arcdps()
}

fn release() {
    arcdps::drop_arcdps();
    device::drop_device();
}

#[cfg(test)]
mod test {
    use super::*;

    #[allow(unused)]
    #[test]
    fn test_loop() {
        device::gen_device();
        loop {
            device::send_to_device(|| pipeline::Arc {
                msgtype: pipeline::Mtype::Imgui as i32,
                msg: Some(pipeline::arc::Msg::Imgui(true)),
            });
        }
    }
}
