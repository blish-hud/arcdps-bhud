mod combat;

use arcdps_bindings::*;
use std::io::prelude::*;
use std::net::TcpStream;

const CONNECTION_STRING: &'static str = "127.0.0.1:8214";

pub fn imgui(not_charsel_or_loading: bool) {
    if let Ok(mut stream) = TcpStream::connect(CONNECTION_STRING) {
        let _ = stream.write(&[1, not_charsel_or_loading as u8]);
    }
}

pub fn combat(
    ev: Option<&cbtevent>,
    src: Option<&Ag>,
    dst: Option<&Ag>,
    skillname: Option<&str>,
    id: u64,
    revision: u64,
) {
    combat::cbt(ev, src, dst, skillname, id, revision);
}
