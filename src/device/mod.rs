mod worker;

use crate::pipeline;
use worker::Device;

static mut DEVICE: Option<Device> = None;

pub fn gen_device() {
    let device = Device::new("BHUDrender");
    unsafe {
        DEVICE = Some(device);
    }
}

pub fn send_to_device(
    func: fn() -> pipeline::Arc,
) -> Result<(), std::sync::mpsc::TrySendError<fn() -> pipeline::Arc>> {
    unsafe {
        match &DEVICE {
            Some(d) => d.send(func),
            _ => Err(std::sync::mpsc::TrySendError::Disconnected(func)),
        }
    }
}
