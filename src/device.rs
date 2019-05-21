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
    func: fn() -> pipeline::arc,
) -> Result<(), std::sync::mpsc::TrySendError<fn() -> pipeline::arc>> {
    unsafe {
        if let Some(d) = &DEVICE {
            d.send(func)
        } else {
            Err(std::sync::mpsc::TrySendError::Disconnected(func))
        }
    }
}

fn hello() {
    for _ in 0..5 {
        if let Ok(_) = send_to_device(|| pipeline::arc {
            msgtype: pipeline::mtype::Greeting,
            msg: pipeline::mod_arc::OneOfmsg::greeting(true),
        }) {
            break;
        }
    }
}

fn bye() {
    for _ in 0..5 {
        if let Ok(_) = send_to_device(|| pipeline::arc {
            msgtype: pipeline::mtype::Greeting,
            msg: pipeline::mod_arc::OneOfmsg::greeting(false),
        }) {
            break;
        }
    }
}
