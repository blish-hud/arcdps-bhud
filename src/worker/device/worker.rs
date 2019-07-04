use std::sync::mpsc;
use std::sync::mpsc::Receiver;

pub type ChannelType = Vec<u8>;

pub struct Device {
    sender: mpsc::Sender<ChannelType>,
}

impl Device {
    pub fn new<A>(action: A) -> Device
    where
        A: Fn(Receiver<ChannelType>) + Sized + Send + 'static,
    {
        let (tx, rx) = mpsc::channel();
        let device = Device { sender: tx };
        std::thread::spawn(move || action(rx));

        device
    }

    pub fn send(&self, func: ChannelType) -> Result<(), mpsc::SendError<ChannelType>> {
        self.sender.send(func)
    }
}
