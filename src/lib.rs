mod arcdps;
mod exports;

use winapi::shared::minwindef::LPVOID;

fn main() -> LPVOID {
    arcdps::gen_arcdps()
}

fn release() {}
