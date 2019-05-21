// Automatically generated rust module for 'pipeline.proto' file

#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(unused_imports)]
#![allow(unknown_lints)]
#![cfg_attr(rustfmt, rustfmt_skip)]


use std::io::Write;
use quick_protobuf::{MessageRead, MessageWrite, BytesReader, Writer, Result};
use quick_protobuf::sizeofs::*;
use super::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum mtype {
    NoMsg = 0,
    Imgui = 1,
    Greeting = 2,
}

impl Default for mtype {
    fn default() -> Self {
        mtype::NoMsg
    }
}

impl From<i32> for mtype {
    fn from(i: i32) -> Self {
        match i {
            0 => mtype::NoMsg,
            1 => mtype::Imgui,
            2 => mtype::Greeting,
            _ => Self::default(),
        }
    }
}

impl<'a> From<&'a str> for mtype {
    fn from(s: &'a str) -> Self {
        match s {
            "NoMsg" => mtype::NoMsg,
            "Imgui" => mtype::Imgui,
            "Greeting" => mtype::Greeting,
            _ => Self::default(),
        }
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct arc {
    pub msgtype: mtype,
    pub msg: mod_arc::OneOfmsg,
}

impl<'a> MessageRead<'a> for arc {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(16) => msg.msgtype = r.read_enum(bytes)?,
                Ok(128) => msg.msg = mod_arc::OneOfmsg::greeting(r.read_bool(bytes)?),
                Ok(8) => msg.msg = mod_arc::OneOfmsg::imgui(r.read_bool(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for arc {
    fn get_size(&self) -> usize {
        0
        + if self.msgtype == pipeline::mtype::NoMsg { 0 } else { 1 + sizeof_varint(*(&self.msgtype) as u64) }
        + match self.msg {
            mod_arc::OneOfmsg::greeting(ref m) => 2 + sizeof_varint(*(m) as u64),
            mod_arc::OneOfmsg::imgui(ref m) => 1 + sizeof_varint(*(m) as u64),
            mod_arc::OneOfmsg::None => 0,
    }    }

    fn write_message<W: Write>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.msgtype != pipeline::mtype::NoMsg { w.write_with_tag(16, |w| w.write_enum(*&self.msgtype as i32))?; }
        match self.msg {            mod_arc::OneOfmsg::greeting(ref m) => { w.write_with_tag(128, |w| w.write_bool(*m))? },
            mod_arc::OneOfmsg::imgui(ref m) => { w.write_with_tag(8, |w| w.write_bool(*m))? },
            mod_arc::OneOfmsg::None => {},
    }        Ok(())
    }
}

pub mod mod_arc {

use super::*;

#[derive(Debug, PartialEq, Clone)]
pub enum OneOfmsg {
    greeting(bool),
    imgui(bool),
    None,
}

impl Default for OneOfmsg {
    fn default() -> Self {
        OneOfmsg::None
    }
}

}
