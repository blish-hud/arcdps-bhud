mod arcdps;
mod executor;
mod exports;
mod pubsub;

fn main() -> Result<(), Box<(dyn std::error::Error + 'static)>> {
    executor::setup();
    pubsub::setup();
    Ok(())
}

fn release() {
    pubsub::teardown();
    executor::teardown();
}
