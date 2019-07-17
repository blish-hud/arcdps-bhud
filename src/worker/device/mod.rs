mod worker;

use std::collections::HashMap;
use std::sync::atomic::AtomicBool;
use std::sync::mpsc::Receiver;
use std::sync::{Arc, RwLock};
pub use worker::ChannelType;
use worker::Device;

lazy_static! {
    static ref DEVICE: RwLock<HashMap<&'static str, Device>> = { RwLock::new(HashMap::new()) };
}

pub fn gen_device<A>(device_name: &'static str, action: A)
where
    A: Fn(Arc<AtomicBool>, Receiver<ChannelType>) + Sized + Send + 'static,
{
    let device = Device::new(action);
    let mut dev = DEVICE.write().unwrap();
    dev.insert(device_name, device);
}

pub fn send_to_device(device_name: &'static str, func: ChannelType) {
    let dev = DEVICE.read().unwrap();
    if let Some(d) = dev.get(device_name) {
        let _ = d.send(func);
    }
}
