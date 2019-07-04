mod worker;

use std::collections::HashMap;
use std::sync::mpsc::Receiver;
use std::sync::Mutex;
pub use worker::ChannelType;
use worker::Device;

static mut DEVICE: Option<Mutex<HashMap<&'static str, Device>>> = None;

pub fn gen_device<A>(device_name: &'static str, action: A)
where
    A: Fn(Receiver<ChannelType>) + Sized + Send + 'static,
{
    let device = Device::new(action);
    unsafe {
        if DEVICE.is_none() {
            DEVICE = Some(Mutex::new(HashMap::with_capacity(1)));
        }
        if let Some(dev) = &mut DEVICE {
            dev.lock().unwrap().insert(device_name, device);
        }
    }
}

pub fn send_to_device(device_name: &'static str, func: ChannelType) {
    unsafe {
        if let Some(h) = &DEVICE {
            if let Some(d) = h.lock().unwrap().get(device_name) {
                let _ = d.send(func);
            }
        }
    }
}
