use std::{ffi::c_void, ptr::NonNull};

mod arcdps;
mod exports;
mod pubsub;

fn main(_: Option<NonNull<c_void>>) -> Result<(), Box<(dyn std::error::Error + 'static)>> {
    pubsub::setup();
    Ok(())
}

fn release() {
    pubsub::teardown();
}
