use std::{
    error::Error,
    net::Ipv4Addr,
    sync::{Arc, OnceLock},
    thread,
    time::Duration,
};

use async_broadcast::{InactiveReceiver, Receiver, Sender};
use bincode::Options;
use futures::{select, AsyncWriteExt, FutureExt};
use smol::{
    net::{TcpListener, TcpStream},
    Executor, Timer,
};

use crate::exports::Message;

pub fn setup() {
    thread::spawn(setup_pubsub);
}

pub fn teardown() {
    MsgChannel::tx().close();
    smol::block_on(wait_for_channel_close());
}

pub fn dispatch<T: Message>(msg: &T) -> Result<(), Box<dyn Error>> {
    let mut data = Vec::with_capacity(1_024);
    let msg = (T::MESSAGE_ID as u8, msg);
    let serializer = bincode::DefaultOptions::new().with_varint_encoding();
    data.extend((serializer.serialized_size(&msg)? as u32).to_le_bytes());
    serializer.serialize_into(&mut data, &msg)?;

    let tx = MsgChannel::tx();
    if T::BLOCK {
        // every event except imgui is sent in its own thread, we want to block here
        smol::block_on(tx.broadcast_direct(Arc::new(data)))?;
    } else {
        // if the queue is full, we simply drop the message
        tx.try_broadcast(Arc::new(data)).ok();
    }
    Ok(())
}

fn setup_pubsub() {
    let ex = Arc::new(Executor::new());

    // worker threads sending data
    for _ in 0..2 {
        let ex = ex.clone();
        thread::spawn(move || smol::block_on(ex.run(wait_for_channel_close())));
    }

    smol::block_on(async {
        // create a listener
        // we are inside a fire and forget thread,
        // crashing on error is equivalent to returning the error
        let listener =
            TcpListener::bind((Ipv4Addr::new(127, 0, 0, 1), get_port(std::process::id())))
                .await
                .unwrap();

        // accept clients in a loop
        loop {
            select! {
                _ = wait_for_channel_close().fuse() => {
                    break;
                },
                opt = listener.accept().fuse() => {
                    if let Ok((stream, _)) = opt {
                        // stream events
                        ex.spawn(stream_data(stream)).detach();
                    }
                },
            }
        }
    });
}

async fn wait_for_channel_close() {
    let mut rx = MsgChannel::rx();
    while rx.recv().await.is_ok() {}
}

async fn stream_data(mut stream: TcpStream) -> std::io::Result<()> {
    let mut rx = MsgChannel::rx();

    while let Ok(msg) = rx.recv().await {
        // The channel waits for the last receiver before freeing messages.
        // A broken connection could block receiving events for all connections.
        // We kill the connection if it doesn't receive the event in time.
        // We do not care about malicious connections as they could game the system
        // by opening many connections and blocking all of them.
        select! {
            res = stream.write_all(msg.as_slice()).fuse() => {
                res?;
            }
            _ = Timer::after(Duration::from_secs(5)).fuse() => {
                break;
            }
        }
    }
    Ok(())
}

fn get_port(pid: u32) -> u16 {
    (pid as u16 + 1) | 1 << 14 | 1 << 15
}

pub type MsgChannelType = Arc<Vec<u8>>;
pub type MsgChannel = StaticChannel;

pub struct StaticChannel {
    tx: Sender<MsgChannelType>,
    rx: InactiveReceiver<MsgChannelType>,
}

impl StaticChannel {
    pub fn tx() -> Sender<MsgChannelType> {
        Self::get().tx.clone()
    }

    pub fn rx() -> Receiver<MsgChannelType> {
        Self::get().rx.clone().activate()
    }

    fn get() -> &'static Self {
        static THIS: OnceLock<StaticChannel> = OnceLock::new();

        THIS.get_or_init(|| {
            let (tx, rx) = async_broadcast::broadcast(1_000);
            Self {
                tx,
                rx: rx.deactivate(),
            }
        })
    }
}
