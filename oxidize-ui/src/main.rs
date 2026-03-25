//! Oxidize UI entry point.

mod api;
mod architect_quips;
mod components;
mod constants;
mod game_loop;
mod timers;
mod handlers;
mod helpers;
mod physics;
mod signals;
mod types;

use leptos::*;
use oxidize_engine::{PlayerState, UpgradeType};
use wasm_bindgen::JsCast;

use constants::*;
use game_loop::setup_physics_timer;
use timers::{setup_architect_timer, setup_sync_timer, setup_state_tick, setup_viewport_timer};
use handlers::{make_buy_upgrade, make_select_planet, make_select_sun};
use helpers::{get_player_uuid, load_state};
use signals::{create_game_signals, GameSignals};
use components::SolarSystem;
use architect_quips::{quip_for_energy_milestone, quip_for_planet_visit, quip_for_upgrade_purchase};
use constants::*;

/// Main app component.
#[component]
pub fn App() -> impl IntoView {
    let player_uuid = get_player_uuid();
    let mut initial_state = load_state();
    initial_state.last_synced_total_energy = initial_state.total_energy_generated;

    let signals = create_game_signals(initial_state);
    let GameSignals {
        state, show_how_to_play, architect_message, upgrade_effects,
        last_purchase_time, planet_angles, moon_angles, spaceship_angle,
        target_planet_idx, is_flying, fly_progress: _, fly_from_x, fly_from_y,
        fly_to_x, fly_to_y, fly_x, fly_y, fly_vx: _, fly_vy: _,
        planet_offset, trail_positions, flight_path, flight_prediction,
        is_arriving, arrival_time: _,
    } = signals;

    let (show_how_to_play_read, show_how_to_play_write) = (show_how_to_play.0, show_how_to_play.1);
    let (architect_message_read, architect_message_write) = (architect_message.0, architect_message.1);

    // Viewport signals
    let (view_offset_x, set_view_offset_x) = create_signal(0.0f64);
    let (view_offset_y, set_view_offset_y) = create_signal(0.0f64);
    let (zoom_level, set_zoom_level) = create_signal(1.0f64);

    // Theme
    create_effect(|_| {
        if let Some(window) = web_sys::window() {
            if let Some(doc) = window.document() {
                if let Some(el) = doc.document_element() {
                    let _ = el.set_attribute("class", "theme-neutral");
                }
            }
        }
    });

    // Viewport pan/zoom, state tick, game loops
    setup_viewport_timer(&signals, set_view_offset_x, set_view_offset_y, set_view_offset_x, set_view_offset_y, zoom_level, set_zoom_level);
    setup_state_tick(state.1);
    setup_physics_timer(&signals);
    setup_sync_timer(&signals, player_uuid.clone());
    setup_architect_timer(&signals);

    let select_planet = make_select_planet(&signals);
    let select_sun = make_select_sun(&signals);
    let buy_upgrade = make_buy_upgrade(&signals);

    // Track previous state for change detection
    let (prev_energy, set_prev_energy) = create_signal(state.0.get().energy);
    let (prev_target_planet, set_prev_target_planet) = create_signal(target_planet_idx.0.get());
    let (prev_solar_sails, set_prev_solar_sails) = create_signal(state.0.get().solar_sails);
    let (prev_plasma_tethers, set_prev_plasma_tethers) = create_signal(state.0.get().plasma_tethers);
    let (prev_orbital_mirrors, set_prev_orbital_mirrors) = create_signal(state.0.get().orbital_mirrors);

    // React to energy milestones
    create_effect(move |_| {
        let current = state.0.get();
        let prev_e = prev_energy.get();
        if current.energy != prev_e {
            let milestones = [1000.0, 10000.0, 100000.0, 1_000_000.0, 10_000_000.0, 100_000_000.0, 1_000_000_000.0];
            for m in milestones {
                if current.energy >= m && prev_e < m {
                    architect_message_write.set(Some(quip_for_energy_milestone(current.energy)));
                    break;
                }
            }
            set_prev_energy.set(current.energy);
        }
    });

    // React to planet visits
    create_effect(move |_| {
        let current_target = target_planet_idx.0.get();
        let prev_target = prev_target_planet.get();
        if current_target != prev_target {
            if let Some(idx) = current_target {
                architect_message_write.set(Some(quip_for_planet_visit(idx)));
            }
            set_prev_target_planet.set(current_target);
        }
    });

    // React to upgrade purchases
    create_effect(move |_| {
        let current = state.0.get();
        let prev_sails = prev_solar_sails.get();
        let prev_plasma = prev_plasma_tethers.get();
        let prev_mirrors = prev_orbital_mirrors.get();

        if current.solar_sails != prev_sails {
            architect_message_write.set(Some(quip_for_upgrade_purchase("SolarSail", current.solar_sails)));
            set_prev_solar_sails.set(current.solar_sails);
        } else if current.plasma_tethers != prev_plasma {
            architect_message_write.set(Some(quip_for_upgrade_purchase("PlasmaTether", current.plasma_tethers)));
            set_prev_plasma_tethers.set(current.plasma_tethers);
        } else if current.orbital_mirrors != prev_mirrors {
            architect_message_write.set(Some(quip_for_upgrade_purchase("OrbitalMirror", current.orbital_mirrors)));
            set_prev_orbital_mirrors.set(current.orbital_mirrors);
        }
    });

    // Pending selection — header writes this, main loop reads and dispatches
    let (pending_planet_select, set_pending_planet_select) = create_signal(None::<usize>);
    create_effect(move |_| {
        let pending = pending_planet_select.get();
        if pending.is_none() { return; }
        if pending == Some(usize::MAX) {
            select_sun();
        } else if let Some(idx) = pending {
            select_planet(idx);
        }
        set_pending_planet_select.set(None);
    });

    view! {
        <div class="flex flex-col h-full bg-transparent text-app-text overflow-hidden transition-all duration-500 font-mono">
            <components::GameHeader
                state={state.0}
                target_planet_idx={target_planet_idx.0}
                pending_planet_select={set_pending_planet_select}
            />

            <div class="relative flex-1 overflow-hidden">
                <components::NeonOrb intensity={Some(state.0.get().energy_per_second() as i32)}/>
                <SolarSystem
                    planet_angles={planet_angles.0} moon_angles={moon_angles.0}
                    spaceship_angle={spaceship_angle.0} target_planet_idx={target_planet_idx.0}
                    is_flying={is_flying.0} fly_from_x={fly_from_x.0} fly_from_y={fly_from_y.0}
                    fly_x={fly_x.0} fly_y={fly_y.0} planet_offset={planet_offset.0}
                    upgrade_effects={upgrade_effects.0} trail_positions={trail_positions.0}
                    flight_path={flight_path.0} flight_prediction={flight_prediction.0}
                    is_arriving={is_arriving.0}
                    view_offset_x={view_offset_x} view_offset_y={view_offset_y}
                    zoom_level={zoom_level}
                />
            </div>

            <div class="w-full flex flex-col items-center pb-4 shrink-0 relative z-10 pointer-events-auto">
                <components::UpgradePanel state={state.0} on_buy={Callback::from(buy_upgrade)}/>
            </div>

            <Show when={move || architect_message_read.get().is_some()}>
                <components::ArchitectToast
                    message={architect_message_read.get().unwrap_or_default()}
                    on_close={Callback::from(move |_| architect_message_write.set(None))}
                />
            </Show>

            <Show when={move || show_how_to_play_read.get()}>
                <components::HowToPlayModal on_close={Callback::from(move |_| show_how_to_play_write.set(false))}/>
            </Show>
        </div>
    }
}

fn main() {
    console_error_panic_hook::set_once();
    if let Some(window) = web_sys::window() {
        if let Some(doc) = window.document() {
            if let Some(root) = doc.get_element_by_id("root") {
                leptos::mount_to(root.unchecked_into::<web_sys::HtmlElement>(), || view! { <App/> });
            }
        }
    }
}
