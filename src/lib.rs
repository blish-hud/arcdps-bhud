mod arcdps;
mod exports;
mod worker;

use winapi::shared::minwindef::LPVOID;

fn main() -> LPVOID {
    worker::socket::new();
    worker::log::new();
    arcdps::gen_arcdps()
}

fn release() {}
