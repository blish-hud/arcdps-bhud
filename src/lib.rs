mod arcdps;
mod device;
mod exports;

use winapi::shared::minwindef::LPVOID;

fn main() -> LPVOID {
    device::gen_device();
    arcdps::gen_arcdps()
}

fn release() {}
