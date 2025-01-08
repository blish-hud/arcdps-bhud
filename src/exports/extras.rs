use arcdps::{ChatMessageInfo2, ChatMessageType, NpcMessageInfo, SquadMessageInfo, UserInfo, UserInfoIter};

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

pub fn ue_msg<'a>(msg_type: ChatMessageType, msg: ChatMessageInfo2<'a>) {
    let _ = msg_type;
    match msg_type {
        ChatMessageType::Squad => {
            if let Some(squad_info) = msg.squad_message_info {
                dispatch(&squad_info).ok();
            }
        }
        ChatMessageType::NPC => {
            if let Some(npc_info) = msg.npc_message_info {
                dispatch(&npc_info).ok();
            }
        }
    };
}

impl Message for SquadMessageInfo<'_> {
    const MESSAGE_ID: MessageId = MessageId::ChatMessage;
}

impl Message for NpcMessageInfo<'_> {
    const MESSAGE_ID: MessageId = MessageId::ChatMessage;
}