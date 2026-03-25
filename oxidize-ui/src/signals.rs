//! Reactive game state signals.

use leptos::*;
use oxidize_engine::PlayerState;

pub struct GameSignals {
    pub state: (ReadSignal<PlayerState>, WriteSignal<PlayerState>),
    pub architect_message: (ReadSignal<Option<String>>, WriteSignal<Option<String>>),
    pub show_how_to_play: (ReadSignal<bool>, WriteSignal<bool>),
}

pub fn create_game_signals(initial_state: PlayerState) -> GameSignals {
    GameSignals {
        state: create_signal(initial_state),
        architect_message: create_signal(None),
        show_how_to_play: create_signal(false),
    }
}
