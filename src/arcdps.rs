use crate::{exports::*, main, release};

arcdps::arcdps_export! {
    name: "BHUDrender",
    sig: 0x2_0804,
    init: main,
    release: release,
    imgui: imgui,
    combat: combat,
    combat_local: combat_local,
    unofficial_extras_squad_update: squad,
    unofficial_extras_chat_message: message,
}
