mod executor;
mod arcdps;
mod exports;
mod pubsub;

use winapi::shared::minwindef::LPVOID;

fn main() -> LPVOID {
    executor::setup();
    pubsub::setup();
    arcdps::gen_arcdps()
}

fn release() {
    pubsub::teardown();
    executor::teardown();
}
