use crate::{pipeline, worker::Device};

static mut DEVICE: Option<Device> = None;

pub fn gen_device() {
    let device = Device::new("BHUDrender");
    unsafe {
        DEVICE = Some(device);
    }

    hello();
}

pub fn drop_device() {
    bye();
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

fn hello() {
    let _ = send_to_device(|| pipeline::Arc {
        msgtype: pipeline::Mtype::Greeting as i32,
        msg: Some(pipeline::arc::Msg::Greeting(true)),
    });
}

fn bye() {
    let _ = send_to_device(|| pipeline::Arc {
        msgtype: pipeline::Mtype::Greeting as i32,
        msg: Some(pipeline::arc::Msg::Greeting(false)),
    });
}
