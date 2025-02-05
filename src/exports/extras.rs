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

pub fn ue_msg<'a>(msg: &ChatMessageInfo2<'a>) {
    match msg {
        ChatMessageInfo2::Squad(squad_info) => {
            dispatch(squad_info).ok();
        }
        ChatMessageInfo2::Npc(npc_info) => {
            dispatch(npc_info).ok();
        }
        _ => {}
    };
}

impl Message for SquadMessageInfo<'_> {
    const MESSAGE_ID: MessageId = MessageId::SquadMessage;
}

impl Message for NpcMessageInfo<'_> {
    const MESSAGE_ID: MessageId = MessageId::NpcMessage;
}
