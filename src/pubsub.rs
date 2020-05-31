use futures::lock::Mutex;
use futures::select;
use futures::FutureExt;
use once_cell::sync::Lazy;
use piper::Sender;
use smol::{Async, Task};
use std::net::{TcpListener, TcpStream};

static SHUTDOWN: Lazy<Mutex<Option<Sender<()>>>> = Lazy::new(|| Mutex::new(None));
static STREAMS: Lazy<Mutex<Vec<Async<TcpStream>>>> = Lazy::new(|| Mutex::new(Vec::new()));

pub fn setup() {
    Task::spawn(setup_pubsub()).unwrap().detach();
}

pub async fn dispatch(data: Vec<u8>) {
    let data = [&data.len().to_le_bytes(), &data[..]].concat();
    let data = &data[..];
    let mut streams = STREAMS.lock().await;
    let mut to_remove = Vec::new();
    for (i, stream) in streams.iter_mut().enumerate() {
        if let Err(_) = futures::io::copy(data, stream).await {
            to_remove.push(i);
        }
    }

    for i in to_remove.drain(..).rev() {
        streams.remove(i);
    }
}

pub fn teardown() {
    smol::block_on(async_teardown());
}

async fn async_teardown() {
    STREAMS.lock().await.clear();
    SHUTDOWN.lock().await.take();
}

async fn setup_pubsub() -> Result<(), std::io::Error> {
    // A channel that sends the SHUTDOWN signal.
    let (s, r) = piper::chan::<()>(0);
    {
        let mut shutdown = SHUTDOWN.lock().await;
        *shutdown = Some(s);
    }

    // Create a listener.
    let listener =
        Async::<TcpListener>::bind(format!("127.0.0.1:{}", get_port(std::process::id())))?;

    // Accept clients in a loop.
    loop {
        select! {
            _ = r.recv().fuse() => break,
            opt = listener.accept().fuse() => {
                if let Ok((stream, _)) = opt {
                    let mut streams = STREAMS.lock().await;
                    streams.push(stream);
                }
            },
        }
    }
    Ok(())
}

fn get_port(pid: u32) -> u16 {
    pid as u16 | 1 << 14 | 1 << 15
}
