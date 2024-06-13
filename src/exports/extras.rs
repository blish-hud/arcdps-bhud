use arcdps::{ChatMessageInfo, UserInfo, UserInfoIter};

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

pub fn ue_msg<'a>(msg: &'a ChatMessageInfo<'a>) {
    dispatch(msg).ok();
}

impl Message for ChatMessageInfo<'_> {
    const MESSAGE_ID: MessageId = MessageId::ChatMessage;
}
