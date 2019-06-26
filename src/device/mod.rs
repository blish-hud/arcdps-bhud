mod worker;

use worker::{ChannelType, Device};

static mut DEVICE: Option<Device> = None;

pub fn gen_device() {
    let device = Device::new("127.0.0.1:8214");
    unsafe {
        DEVICE = Some(device);
    }
}

pub fn send_to_device(func: ChannelType) {
    unsafe {
        if let Some(d) = &DEVICE {
            let _ = d.send(func);
        }
    }
}
