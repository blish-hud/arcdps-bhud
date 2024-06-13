use arcdps::{Agent, CombatEvent};
use serde::Serialize;

use crate::{
    exports::{Message, MessageId},
    pubsub::dispatch,
};

pub fn cbt<'a>(
    ev: Option<&'a CombatEvent>,
    src: Option<Agent>,
    dst: Option<Agent>,
    skillname: Option<&'a str>,
    id: u64,
    revision: u64,
) {
    dispatch(&CombatMessage(CombatMessageInner {
        ev,
        src,
        dst,
        skillname,
        id,
        revision,
    }))
    .ok();
}

pub fn cbt_local<'a>(
    ev: Option<&'a CombatEvent>,
    src: Option<Agent>,
    dst: Option<Agent>,
    skillname: Option<&'a str>,
    id: u64,
    revision: u64,
) {
    dispatch(&CombatMessageLocal(CombatMessageInner {
        ev,
        src,
        dst,
        skillname,
        id,
        revision,
    }))
    .ok();
}

#[derive(Serialize)]
#[repr(transparent)]
pub struct CombatMessage<'a>(pub CombatMessageInner<'a>);

impl Message for CombatMessage<'_> {
    const MESSAGE_ID: MessageId = MessageId::Combat;
}

#[derive(Serialize)]
#[repr(transparent)]
pub struct CombatMessageLocal<'a>(pub CombatMessageInner<'a>);

impl Message for CombatMessageLocal<'_> {
    const MESSAGE_ID: MessageId = MessageId::CombatLocal;
}

#[derive(Serialize)]
pub struct CombatMessageInner<'a> {
    pub ev: Option<&'a CombatEvent>,
    pub src: Option<Agent<'a>>,
    pub dst: Option<Agent<'a>>,
    pub skillname: Option<&'a str>,
    pub id: u64,
    pub revision: u64,
}
