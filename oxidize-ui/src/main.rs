//! Oxidize — Saturn Asteroids mode.

mod asteroids;
mod api;
mod architect_quips;
mod components;
mod helpers;
mod signals;
mod timers;

use leptos::*;
use wasm_bindgen::JsCast;
use oxidize_engine::{PlayerState, UpgradeType};
use asteroids::AsteroidsArena;
use architect_quips::{quip_for_energy_milestone, quip_for_upgrade_purchase};
use components::{ArchitectToast, GameHeader, HowToPlayModal, NeonOrb, UpgradePanel};
use helpers::{get_player_uuid, load_state, save_state};
use signals::GameSignals;
use timers::setup_state_tick;

#[component]
pub fn App() -> impl IntoView {
    let _player_uuid = get_player_uuid();
    let mut initial_state = load_state();
    initial_state.last_synced_total_energy = initial_state.total_energy_generated;

    let signals = signals::create_game_signals(initial_state);
    let GameSignals { state, architect_message, show_how_to_play, .. } = signals;

    let (arch_read, arch_write) = (architect_message.0, architect_message.1);
    let (how_read, how_write) = (show_how_to_play.0, show_how_to_play.1);

    create_effect(move |_| {
        let eps = state.0.get().energy_per_second();
        if eps > 0.0 { arch_write.set(Some(quip_for_upgrade_purchase("start", 0))); }
    });

    setup_state_tick(state.1);

    let (prev_energy, set_prev_energy) = create_signal(state.0.get().energy);

    create_effect(move |_| {
        let current = state.0.get().energy;
        let prev = prev_energy.get();
        if current >= 1000.0 && prev < 1000.0 {
            arch_write.set(Some(quip_for_energy_milestone(current)));
        }
        set_prev_energy.set(current);
    });

    create_effect(move |_| {
        if let Some(window) = web_sys::window() {
            if let Some(doc) = window.document() {
                if let Some(el) = doc.document_element() {
                    let _ = el.set_attribute("class", "theme-neutral");
                }
            }
        }
    });

    let buy_callback = Callback::new(move |ut| {
        let current_time = js_sys::Date::now() as u64;
        state.1.update(|s| {
            if s.buy_upgrade(ut, current_time) {
                arch_write.set(Some(quip_for_upgrade_purchase(
                    match ut { oxidize_engine::UpgradeType::SolarSail => "SolarSail", _ => "upgrade" },
                    s.count_for_upgrade(ut),
                )));
            }
        });
        let s = state.0.get();
        save_state(&s);
    });

    view! {
        <div class="flex flex-col h-full bg-transparent text-app-text overflow-hidden font-mono">
            <GameHeader energy={move || state.0.get().energy} eps={move || state.0.get().energy_per_second()} />
            <div class="relative flex-1 overflow-hidden">
                <NeonOrb intensity={Some(state.0.get().energy_per_second() as i32)}/>
                <AsteroidsArena state_set={state.1} />
            </div>
            <div class="w-full flex flex-col items-center pb-4 shrink-0 relative z-10 pointer-events-auto">
                <UpgradePanel state={state.0} on_buy={buy_callback}/>
            </div>
            <Show when={move || arch_read.get().is_some()}>
                <ArchitectToast
                    message={arch_read.get().unwrap_or_default()}
                    on_close={Callback::from(move |_| arch_write.set(None))}
                />
            </Show>
            <Show when={move || how_read.get()}>
                <HowToPlayModal on_close={Callback::from(move |_| how_write.set(false))}/>
            </Show>
        </div>
    }
}

fn main() {
    console_error_panic_hook::set_once();
    if let Some(window) = web_sys::window() {
        if let Some(doc) = window.document() {
            if let Some(root) = doc.get_element_by_id("root") {
                leptos::mount_to(root.dyn_into::<web_sys::HtmlElement>().unwrap(), || view! { <App/> });
            }
        }
    }
}
