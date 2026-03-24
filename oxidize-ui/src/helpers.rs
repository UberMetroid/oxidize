//! Helper functions

use oxidize_engine::PlayerState;

pub fn get_player_uuid() -> String {
    if let Some(window) = web_sys::window() {
        if let Ok(Some(storage)) = window.local_storage() {
            if let Ok(Some(uuid)) = storage.get_item("player-uuid") {
                return uuid;
            }
            let new_uuid = uuid::Uuid::new_v4().to_string();
            let _ = storage.set_item("player-uuid", &new_uuid);
            return new_uuid;
        }
    }
    uuid::Uuid::new_v4().to_string()
}

pub fn load_state() -> PlayerState {
    if let Some(window) = web_sys::window() {
        if let Ok(Some(storage)) = window.local_storage() {
            if let Ok(Some(s)) = storage.get_item("player-state") {
                if let Ok(state) = serde_json::from_str::<PlayerState>(&s) {
                    let mut state = state;
                    state.calculate_offline_progress(js_sys::Date::now() as u64);
                    return state;
                }
            }
        }
    }
    PlayerState::new()
}

pub fn save_state(state: &PlayerState) {
    if let Some(window) = web_sys::window() {
        if let Ok(Some(storage)) = window.local_storage() {
            if let Ok(serialized) = serde_json::to_string(state) {
                let _ = storage.set_item("player-state", &serialized);
            }
        }
    }
}
