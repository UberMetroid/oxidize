//! Lightweight game timers.

use gloo_timers::callback::Interval;
use leptos::*;
use oxidize_engine::PlayerState;

use crate::helpers::save_state;

/// Advances game state every 100ms.
pub fn setup_state_tick(state_set: WriteSignal<PlayerState>) {
    create_effect(move |_| {
        Interval::new(100, move || {
            let now = js_sys::Date::now() as u64;
            state_set.update(|s| {
                s.tick(0.1, now);
                s.last_sync_time = now;
                save_state(s);
            });
        }).forget();
    });
}
