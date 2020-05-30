use once_cell::sync::Lazy;
use parking_lot::Mutex;
use piper::Sender;
use std::thread;

static SHUTDOWN: Lazy<Mutex<Option<Sender<()>>>> = Lazy::new(|| Mutex::new(None));

pub fn setup() {
    // A channel that sends the SHUTDOWN signal.
    let (s, r) = piper::chan::<()>(0);
    {
        let mut shutdown = SHUTDOWN.lock();
        *shutdown = Some(s);
    }

    // Create an executor thread pool.
    for _ in 0..3 {
        // Spawn an executor thread that waits for the SHUTDOWN signal.
        let r = r.clone();
        thread::spawn(move || smol::run(r.recv()));
    }
}

pub fn teardown() {
    SHUTDOWN.lock().take();
}
