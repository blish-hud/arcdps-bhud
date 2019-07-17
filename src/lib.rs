mod arcdps;
mod exports;
mod worker;

#[macro_use]
extern crate lazy_static;

use winapi::shared::minwindef::LPVOID;

fn main() -> LPVOID {
    worker::socket::new();
    worker::log::new();
    arcdps::gen_arcdps()
}

fn release() {}
