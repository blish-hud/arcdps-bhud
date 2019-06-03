use arcdps_bindings::*;
/*use std::fs::File;
use std::{io::Write, sync::Mutex};

static mut FILE: Option<Mutex<File>> = None;
*/
pub fn cbt(
    _ev: Option<&cbtevent>,
    _src: Option<&Ag>,
    _dst: Option<&Ag>,
    _skillname: Option<&str>,
    _id: u64,
    _revision: u64,
) {
    /*
    if let Some(ev) = _ev {
        let mut file = unsafe {
            if let Some(f) = &mut FILE {
                f.lock().unwrap()
            } else {
                return;
            }
        };
        let _ = writeln!(&mut file, "{}", ev.time);
    }*/
}

pub fn gen_combat() {
    /*let file = File::create("foo.txt").expect("creating file failed");
    unsafe {
        FILE = Some(Mutex::new(file));
    }*/
}

pub fn drop_combat() {}
