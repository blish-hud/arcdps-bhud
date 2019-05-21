mod arcdps;
mod device;
mod exports;
mod pipeline;
mod worker;

use winapi::shared::minwindef::LPVOID;

fn main() -> LPVOID {
    device::gen_device();
    arcdps::gen_arcdps()
}

fn release() {
    arcdps::drop_arcdps();
    device::drop_device();
}
