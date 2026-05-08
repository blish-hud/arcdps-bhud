use crate::{exports::*, main, release};

arcdps::arcdps_export! {
    name: "BHUDrender",
    sig: 0x2_0804,
    init: main,
    release: release,
    raw_imgui: imgui,
    combat: combat,
    combat_local: combat_local,
    unofficial_extras_squad_update: squad,
    unofficial_extras_chat_message2: message,
}
