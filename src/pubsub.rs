use std::net::{TcpListener, TcpStream};
use smol::{Async, Task};
use futures::select;
use piper::Sender;
use futures::FutureExt;
use futures::lock::Mutex;
use once_cell::sync::Lazy;

static SHUTDOWN: Lazy<Mutex<Option<Sender<()>>>> = Lazy::new(|| Mutex::new(None));
static STREAMS: Lazy<Mutex<Vec<Async<TcpStream>>>> = Lazy::new(|| Mutex::new(Vec::new()));

pub fn setup() {
    Task::spawn(setup_pubsub()).unwrap().detach();
}

pub async fn dispatch(data: Vec<u8>) {
    let data = &data[..];
    let mut streams = STREAMS.lock().await;
    let mut workload = Vec::with_capacity(streams.len());
    for stream in streams.iter_mut() {
        workload.push(futures::io::copy(data, stream));
    }

    for task in workload.drain(..) {
        let _ = task.await;
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
    let listener = Async::<TcpListener>::bind(format!("127.0.0.1:{}", get_port(std::process::id())))?;

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
