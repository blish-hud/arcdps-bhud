use std::sync::atomic::Ordering::Acquire;
use std::sync::{
    atomic::AtomicBool,
    mpsc::{self, Receiver},
    Arc,
};

pub type ChannelType = Vec<u8>;

pub struct Device {
    active: Arc<AtomicBool>,
    sender: mpsc::SyncSender<ChannelType>,
}

impl Device {
    pub fn new<A>(action: A) -> Device
    where
        A: Fn(Arc<AtomicBool>, Receiver<ChannelType>) + Sized + Send + 'static,
    {
        let (tx, rx) = mpsc::sync_channel(1000);
        let device = Device {
            active: Arc::new(AtomicBool::new(false)),
            sender: tx,
        };
        let active = device.active.clone();
        std::thread::spawn(move || action(active, rx));

        device
    }

    pub fn send(&self, func: ChannelType) -> Result<(), mpsc::TrySendError<ChannelType>> {
        if self.active.load(Acquire) {
            self.sender.try_send(func)
        } else {
            Ok(())
        }
    }
}
