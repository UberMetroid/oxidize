//! Lightweight timers: sync, architect idle, viewport zoom, state tick.

use gloo_timers::callback::Interval;
use leptos::*;
use oxidize_engine::architect::{generate_quip, QuipTrigger};
use oxidize_engine::PlayerState;
use wasm_bindgen_futures::spawn_local;

use crate::api;
use crate::constants::*;
use crate::signals::GameSignals;
use crate::helpers::save_state;

/// Syncs state to server every 2000ms.
pub fn setup_sync_timer(signals: &GameSignals, player_uuid: String) {
    let state_read = signals.state.0;
    create_effect(move |_| {
        let uuid = player_uuid.clone();
        Interval::new(2000, move || {
            let mut s = state_read.get();
            s.last_synced_total_energy = s.total_energy_generated;
            save_state(&s);
            let u = uuid.clone(); let sc = s.clone();
            spawn_local(async move { let _ = api::sync_state(&u, &sc).await; });
        }).forget();
    });
}

/// Shows architect quip after 60s idle.
pub fn setup_architect_timer(signals: &GameSignals) {
    let last_purchase = signals.last_purchase_time.0;
    let arch_set = signals.architect_message.1;
    create_effect(move |_| {
        Interval::new(10000, move || {
            let t = last_purchase.get();
            if t == 0 { return; }
            let idle = (js_sys::Date::now() as f64 - t as f64) / 1000.0;
            if idle >= 60.0 { arch_set.set(Some(generate_quip(QuipTrigger::Idle))); }
        }).forget();
    });
}

/// Smoothly lerps zoom toward target planet and pans to center it.
pub fn setup_viewport_timer(
    signals: &GameSignals,
    _ox: WriteSignal<f64>, _oy: WriteSignal<f64>,
    set_ox: WriteSignal<f64>, set_oy: WriteSignal<f64>,
    zoom: ReadSignal<f64>, set_zoom: WriteSignal<f64>,
) {
    let tpi_r = signals.target_planet_idx.0;
    let pa_r = signals.planet_angles.0;
    create_effect(move |_| {
        let porbits = pa_r.get();
        let idx = tpi_r.get();
        let cz = zoom.get();
        let (px, py, tz) = if let Some(i) = idx {
            let a = PLANET_INITIAL_ANGLES[i] + porbits[i];
            let r = PLANET_DATA[i].0;
            (50.0 + r * a.cos(), 50.0 + r * a.sin(), PLANET_ZOOM[i])
        } else { (50.0, 50.0, 1.0) };
        let nz = cz + (tz - cz) * 0.08;
        set_zoom.set(nz);
        set_ox.set(50.0 - px);
        set_oy.set(50.0 - py);
    });
}

/// Advances game state every 100ms.
pub fn setup_state_tick(state_set: WriteSignal<PlayerState>) {
    create_effect(move |_| {
        Interval::new(100, move || {
            let now = js_sys::Date::now() as u64;
            state_set.update(|s| { s.tick(0.1, now); s.last_sync_time = now; });
        }).forget();
    });
}
