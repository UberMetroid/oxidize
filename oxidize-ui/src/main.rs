//! Main entry point for Oxidize UI
//! 
//! Mounts the app, creates all signals, sets up game loops,
//! and orchestrates the view.

mod api;
mod components;
mod constants;
mod game_loop;
mod handlers;
mod helpers;
mod signals;
mod types;

use leptos::*;
use oxidize_engine::architect::{generate_quip, QuipTrigger};
use oxidize_engine::{PlayerState, UpgradeType};
use wasm_bindgen::JsCast;

use constants::*;
use game_loop::{setup_architect_timer, setup_physics_timer, setup_sync_timer};
use handlers::{make_buy_upgrade, make_select_planet, make_select_sun};
use helpers::{get_player_uuid, load_state, save_state};
use signals::{create_game_signals, GameSignals};
use components::SolarSystem;
use types::UpgradeEffect;

/// Target zoom levels per planet (smaller = more zoomed out)
const PLANET_ZOOM: [f64; 8] = [
    1.5, // Mercury - close up
    1.3, // Venus
    1.1, // Earth
    1.4, // Mars
    0.9, // Jupiter - medium
    0.7, // Saturn
    0.65, // Uranus - zoomed out
    0.55, // Neptune - most zoomed out
];

#[component]
pub fn App() -> impl IntoView {
    let player_uuid = get_player_uuid();
    let mut initial_state = load_state();
    initial_state.last_synced_total_energy = initial_state.total_energy_generated;
    
    let signals = create_game_signals(initial_state);
    
    let GameSignals {
        state,
        show_how_to_play,
        architect_message,
        upgrade_effects,
        last_purchase_time,
        planet_angles,
        moon_angles,
        spaceship_angle,
        target_planet_idx,
        is_flying,
        fly_progress,
        fly_from_x,
        fly_from_y,
        fly_to_x,
        fly_to_y,
        fly_x,
        fly_y,
        fly_vx: _fly_vx,
        fly_vy: _fly_vy,
        planet_offset,
        trail_positions,
        flight_path,
        flight_prediction,
        is_arriving,
        arrival_time: _arrival_time,
    } = signals;
    
    let (show_how_to_play_set, set_show_how_to_play) = (show_how_to_play.1, show_how_to_play.1);
    let (architect_message_set, _set_architect_message) = (architect_message.1, architect_message.1);

    // Viewport transform signals (pan + zoom)
    let (view_offset_x, set_view_offset_x) = create_signal(0.0f64);
    let (view_offset_y, set_view_offset_y) = create_signal(0.0f64);
    let (zoom_level, set_zoom_level) = create_signal(1.0f64);

    // Set up theme
    create_effect(|_| {
        if let Some(window) = web_sys::window() {
            if let Some(doc) = window.document() {
                if let Some(el) = doc.document_element() {
                    let _ = el.set_attribute("class", "theme-neutral");
                }
            }
        }
    });

    // Viewport animation: smooth zoom lerp + pan centering
    create_effect(move |_| {
        let porbits = planet_angles.0.get();
        let target_idx = target_planet_idx.0.get();
        let current_zoom = zoom_level.get();
        
        // Compute target planet's world position for pan centering
        let (planet_x, planet_y, target_zoom) = if let Some(idx) = target_idx {
            let angle = PLANET_INITIAL_ANGLES[idx] + porbits[idx];
            let r = PLANET_DATA[idx].0;
            let px = 50.0 + r * angle.cos();
            let py = 50.0 + r * angle.sin();
            (px, py, PLANET_ZOOM[idx])
        } else {
            // Sun at center (50, 50), zoom 1.0
            (50.0, 50.0, 1.0)
        };
        
        // Smooth zoom lerp (5% per frame toward target)
        let new_zoom = current_zoom + (target_zoom - current_zoom) * 0.08;
        set_zoom_level.set(new_zoom);
        
        // Pan centering: offset to keep planet centered
        let z = new_zoom;
        let pan_x = (50.0 - planet_x) * z;
        let pan_y = (50.0 - planet_y) * z;
        set_view_offset_x.set(pan_x);
        set_view_offset_y.set(pan_y);
    });

    // Game state tick timer (100ms)
    create_effect(move |_| {
        let state_set = state.1;
        let interval = gloo_timers::callback::Interval::new(100, move || {
            let current_time = js_sys::Date::now() as u64;
            state_set.update(|s| { s.tick(0.1, current_time); s.last_sync_time = current_time; });
        });
        interval.forget();
    });

    // Set up game loops
    setup_physics_timer(&signals);
    setup_sync_timer(&signals, player_uuid.clone());
    setup_architect_timer(&signals);

    // Create handlers
    let select_planet = make_select_planet(&signals);
    let select_sun = make_select_sun(&signals);
    let buy_upgrade = make_buy_upgrade(&signals);

    view! {
        <div class="flex flex-col h-full bg-transparent text-app-text overflow-hidden transition-all duration-500 font-mono">
            <components::GameHeader
                state={state.0}
                target_planet_idx={target_planet_idx.0}
                on_select_sun={move || { select_sun(); }}
                on_select_planet={move |idx| { select_planet(idx); }}
            />

            {/* Solar System */}
            <div class="relative flex-1 overflow-hidden">
                <components::NeonOrb intensity={Some(state.0.get().energy_per_second() as i32)}/>

                <SolarSystem
                    planet_angles={planet_angles.0}
                    moon_angles={moon_angles.0}
                    spaceship_angle={spaceship_angle.0}
                    target_planet_idx={target_planet_idx.0}
                    is_flying={is_flying.0}
                    fly_from_x={fly_from_x.0}
                    fly_from_y={fly_from_y.0}
                    fly_x={fly_x.0}
                    fly_y={fly_y.0}
                    planet_offset={planet_offset.0}
                    upgrade_effects={upgrade_effects.0}
                    trail_positions={trail_positions.0}
                    flight_path={flight_path.0}
                    flight_prediction={flight_prediction.0}
                    is_arriving={is_arriving.0}
                    view_offset_x={view_offset_x}
                    view_offset_y={view_offset_y}
                    zoom_level={zoom_level}
                />
            </div>

            {/* Upgrade Panel */}
            <div class="w-full flex flex-col items-center pb-4 shrink-0 relative z-10 pointer-events-auto">
                <components::UpgradePanel state={state.0} on_buy={Callback::from(buy_upgrade)}/>
            </div>

            <Show when={move || architect_message.0.get().is_some()}>
                <components::ArchitectToast
                    message={architect_message.0.get().unwrap_or_default()}
                    on_close={Callback::from(move |_| architect_message.1.set(None))}
                />
            </Show>

            <Show when={move || show_how_to_play.0.get()}>
                <components::HowToPlayModal on_close={Callback::from(move |_| show_how_to_play_set.set(false))} />
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
