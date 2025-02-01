mod combat;
mod extras;

use arcdps::imgui;
pub use combat::{cbt as combat, cbt_local as combat_local};
pub use extras::{ue_msg as message, ue_user as squad};
use serde::Serialize;

use crate::pubsub::dispatch;

pub trait Message: Serialize {
    const MESSAGE_ID: MessageId;
    /// block the sending thread when the message queue is full
    const BLOCK: bool = true;
}

#[repr(u8)]
#[derive(Serialize)]
pub enum MessageId {
    ImguiCallback = 1,
    Combat,
    CombatLocal,
    UserInfo,
    SquadMessage,
    NpcMessage
}

pub fn imgui(_ui: &imgui::Ui, not_charsel_or_loading: bool) {
    dispatch(&Ui {
        not_charsel_or_loading,
    })
    .ok();
}

#[derive(Serialize)]
pub struct Ui {
    pub not_charsel_or_loading: bool,
}

impl Message for Ui {
    const BLOCK: bool = false;
    const MESSAGE_ID: MessageId = MessageId::ImguiCallback;
}
