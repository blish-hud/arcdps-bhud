mod arcdps;
mod executor;
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
