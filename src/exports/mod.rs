mod combat;

use crate::pubsub::dispatch;
use smol::Task;
use crate::protos::eventdata::{Event, ImGuiEvent, UiState};

pub fn imgui(not_charsel_or_loading: bool) {
    let mut event = Event::new();
    let mut imgui_event = ImGuiEvent::new();
    if not_charsel_or_loading {
        imgui_event.set_UiState(UiState::Default);
    } else {
        imgui_event.set_UiState(UiState::CharacterSelectionOrLoading);
    }

    event.set_imgui_event(imgui_event);
    Task::spawn(dispatch(protobuf::Message::write_to_bytes(&event).unwrap())).detach();
}

pub use combat::cbt as combat;
pub use combat::cbt_local as combat_local;
