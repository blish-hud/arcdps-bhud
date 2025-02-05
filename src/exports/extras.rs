use arcdps::{ChatMessageInfo2, NpcMessageInfo, SquadMessageInfo, UserInfo, UserInfoIter};

use crate::{
    exports::{Message, MessageId},
    pubsub::dispatch,
};

pub fn ue_user(users: UserInfoIter) {
    for user in users {
        let user: UserInfo = user;
        // https://discord.com/channels/456611641526845473/953659301162004591/1005618923921735721
        // Elite Insights Discord, mentions that callbacks are async,
        // we just block receiving the event for other addons
        dispatch(&user).ok();
    }
}

impl Message for UserInfo<'_> {
    const MESSAGE_ID: MessageId = MessageId::UserInfo;
}

pub fn ue_msg(msg: &ChatMessageInfo2) {
    match msg {
        ChatMessageInfo2::Squad(squad_info) => {
            dispatch(squad_info).ok();
        }
        ChatMessageInfo2::Npc(npc_info) => {
            dispatch(&NpcMsg::from(npc_info)).ok();
        }
        _ => {}
    };
}

#[derive(Debug, serde::Serialize)]
struct NpcMsg<'a> {
    /// Null terminated character name of the NPC or the player character.
    /// The string is only valid for the duration of the call.
    pub character_name: &'a str,

    /// Null terminated string of the message said by an npc or the user
    /// character. The string is only valid for the duration of the call.
    pub message: &'a str,

    /// Time since epoch UTC in nanoseconds.
    /// This can be used to sort messages, when they are out of order.
    pub timestamp: u64,
}

impl<'a> From<&'a NpcMessageInfo<'a>> for NpcMsg<'a> {
    fn from(msg: &NpcMessageInfo<'a>) -> Self {
        Self {
            character_name: msg.character_name,
            message: msg.character_name,
            timestamp: msg.timestamp.timestamp_nanos_opt().unwrap() as _,
        }
    }
}

impl Message for SquadMessageInfo<'_> {
    const MESSAGE_ID: MessageId = MessageId::SquadMessage;
}

impl Message for NpcMsg<'_> {
    const MESSAGE_ID: MessageId = MessageId::NpcMessage;
}
