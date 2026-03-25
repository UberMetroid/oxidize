mod api;
mod components;
mod constants;
mod helpers;
mod solar_system;
mod types;

use leptos::*;
use oxidize_engine::architect::{generate_quip, QuipTrigger};
use oxidize_engine::{PlayerState, UpgradeType};
use std::f64::consts::PI;
use wasm_bindgen::JsCast;

use constants::*;
use helpers::{get_player_uuid, load_state, save_state};
use solar_system::SolarSystem;
use types::UpgradeEffect;

#[component]
pub fn App() -> impl IntoView {
    let player_uuid = get_player_uuid();
    let mut initial_state = load_state();
    initial_state.last_synced_total_energy = initial_state.total_energy_generated;
    let (state, set_state) = create_signal(initial_state);
    let (show_how_to_play, set_show_how_to_play) = create_signal(false);
    let (architect_message, set_architect_message) = create_signal(None as Option<String>);
    let (upgrade_effects, set_upgrade_effects) = create_signal(Vec::new());
    let (last_purchase_time, set_last_purchase_time) = create_signal(0u64);

    let (planet_angles, set_planet_angles) = create_signal(vec![0.0f64; 8]);
    // Moon angles: 14 total moons (Luna, Phobos, Deimos, Io, Europa, Ganymede, Callisto, Titan, Enceladus, Mimas, Rhea, Titania, Oberon, Triton)
    let (moon_angles, set_moon_angles) = create_signal(vec![0.0f64; 14]);
    let (spaceship_angle, set_spaceship_angle) = create_signal(0.0);
    let (target_planet_idx, set_target_planet_idx) = create_signal(None as Option<usize>);
    let (is_flying, set_is_flying) = create_signal(false);
    let (fly_progress, _set_fly_progress) = create_signal(1.0);
    let (fly_from_x, set_fly_from_x) = create_signal(0.0f64);
    let (fly_from_y, set_fly_from_y) = create_signal(0.0f64);
    let (fly_to_x, set_fly_to_x) = create_signal(0.0f64);
    let (fly_to_y, set_fly_to_y) = create_signal(0.0f64);
    let (fly_x, set_fly_x) = create_signal(0.0f64);
    let (fly_y, set_fly_y) = create_signal(0.0f64);
    let (fly_vx, set_fly_vx) = create_signal(0.0f64); // velocity x
    let (fly_vy, set_fly_vy) = create_signal(0.0f64); // velocity y
    let (planet_offset, set_planet_offset) = create_signal(0.0f64);
    let (trail_positions, set_trail_positions) = create_signal(Vec::<(f64, f64, f64)>::new()); // (x, y, age)
    let (flight_path, set_flight_path) = create_signal(Vec::<(f64, f64)>::new()); // actual trajectory during flight
    let (flight_prediction, set_flight_prediction) = create_signal(Vec::<(f64, f64)>::new()); // predicted curved path
    let (is_arriving, set_is_arriving) = create_signal(false); // brief moment when snapping into orbit
    let (arrival_time, set_arrival_time) = create_signal(0.0); // timestamp when arrival started

    create_effect(|_| {
        if let Some(window) = web_sys::window() {
            if let Some(doc) = window.document() {
                if let Some(el) = doc.document_element() {
                    let _ = el.set_attribute("class", "theme-neutral");
                }
            }
        }
    });

    create_effect(move |_| {
        let interval = gloo_timers::callback::Interval::new(100, move || {
            let current_time = js_sys::Date::now() as u64;
            set_state.update(|s| { s.tick(0.1, current_time); s.last_sync_time = current_time; });
        });
        interval.forget();
    });

    create_effect(move |_| {
        let uuid = player_uuid.clone();
        let interval = gloo_timers::callback::Interval::new(2000, move || {
            let mut current_state = state.get();
            current_state.last_synced_total_energy = current_state.total_energy_generated;
            save_state(&current_state);
            let uuid_clone = uuid.clone();
            let state_clone = current_state.clone();
            wasm_bindgen_futures::spawn_local(async move { let _ = api::sync_state(&uuid_clone, &state_clone).await; });
        });
        interval.forget();
    });

    create_effect(move |_| {
        let interval = gloo_timers::callback::Interval::new(10000, move || {
            let last_purchase = last_purchase_time.get();
            if last_purchase == 0 { return; }
            let seconds_idle = (js_sys::Date::now() as f64 - last_purchase as f64) / 1000.0;
            if seconds_idle >= 60.0 {
                set_architect_message.set(Some(generate_quip(QuipTrigger::Idle)));
            }
        });
        interval.forget();
    });

    create_effect(move |_| {
        let mut frame_count = 0i32;
        let interval = gloo_timers::callback::Interval::new(16, move || {
            frame_count = (frame_count + 1) % 3;
            set_planet_angles.update(|angles: &mut Vec<f64>| {
                for (i, angle) in angles.iter_mut().enumerate() {
                    *angle += (2.0 * PI) / (PLANET_PERIODS[i] * 60.0);
                    if *angle > 2.0 * PI { *angle -= 2.0 * PI; }
                }
            });
            set_moon_angles.update(|angles: &mut Vec<f64>| {
                for (i, angle) in angles.iter_mut().enumerate() {
                    *angle += (2.0 * PI) / (MOON_PERIODS_FLAT[i] * 60.0);
                    if *angle > 2.0 * PI { *angle -= 2.0 * PI; }
                }
            });
            set_upgrade_effects.update(|effects: &mut Vec<UpgradeEffect>| {
                let current_time: f64 = js_sys::Date::now();
                effects.retain_mut(|effect: &mut UpgradeEffect| {
                    let elapsed: f64 = (current_time - effect.start_time) / 1000.0;
                    let duration = effect.duration_secs();
                    if elapsed < duration || effect.permanent {
                        effect.progress = (elapsed / duration).min(1.0);
                        true
                    } else {
                        false
                    }
                });
            });
            // Track ship world position for engine trail
            let ship_angle: f64 = spaceship_angle.get();
            let porbits = planet_angles.get();
            let orbiting_idx = target_planet_idx.get();
            let flying = is_flying.get();
            let (world_x, world_y) = if flying {
                (fly_x.get(), fly_y.get())
            } else if let Some(idx) = orbiting_idx {
                let planet_angle = PLANET_INITIAL_ANGLES[idx] + porbits[idx];
                let planet_orbit_r = PLANET_DATA[idx].0;
                let planet_x = 50.0 + planet_orbit_r * planet_angle.cos();
                let planet_y = 50.0 + planet_orbit_r * planet_angle.sin();
                let ship_orbit_r = SHIP_PLANET_ORBIT_RADIUS;
                let offset = planet_offset.get();
                let ship_angle = planet_angle + offset;
                let sx = planet_x + ship_orbit_r * ship_angle.cos();
                let sy = planet_y + ship_orbit_r * ship_angle.sin();
                (sx, sy)
            } else {
                let x = 50.0 + SHIP_ORBIT_RADIUS as f64 * ship_angle.cos();
                let y = 50.0 + SHIP_ORBIT_RADIUS as f64 * ship_angle.sin();
                (x, y)
            };
            if frame_count == 0 {
                set_trail_positions.update(|trail: &mut Vec<(f64, f64, f64)>| {
                    trail.push((world_x, world_y, 0.0));
                    if trail.len() > 20 { trail.remove(0); }
                });
            }
            set_trail_positions.update(|trail: &mut Vec<(f64, f64, f64)>| {
                for pos in trail.iter_mut() { pos.2 = (pos.2 + 0.05).min(1.0); }
                trail.retain(|p| p.2 < 1.0);
            });
            if is_flying.get() {
                // Physics-based movement with attraction and drag
                let _dt = 0.016; // ~60fps timestep
                let target_x = fly_to_x.get();
                let target_y = fly_to_y.get();
                let current_x = fly_x.get();
                let current_y = fly_y.get();
                let vx = fly_vx.get();
                let vy = fly_vy.get();

                // Direction to target
                let dx = target_x - current_x;
                let dy = target_y - current_y;
                let dist = (dx * dx + dy * dy).sqrt();

                // Attraction force (stronger when further, weaker when close)
                let attraction_strength = 0.15;
                let ax = dx / dist.max(0.1) * attraction_strength;
                let ay = dy / dist.max(0.1) * attraction_strength;

                // Drag (proportional to velocity)
                let drag = 0.92;

                // Update velocity: apply attraction, then drag
                let new_vx = (vx + ax) * drag;
                let new_vy = (vy + ay) * drag;

                // Update position
                let new_x = current_x + new_vx;
                let new_y = current_y + new_vy;

                set_fly_vx.set(new_vx);
                set_fly_vy.set(new_vy);
                set_fly_x.set(new_x);
                set_fly_y.set(new_y);

                // Track flight path (actual trajectory)
                if frame_count == 0 {
                    set_flight_path.update(|path: &mut Vec<(f64, f64)>| {
                        path.push((new_x, new_y));
                        if path.len() > 100 { path.remove(0); }
                    });
                }

                // Compute gravity-bent flight prediction
                let porbits = planet_angles.get();
                let target_idx = target_planet_idx.get().unwrap_or(0);
                let mut prediction_points = Vec::new();
                let mut pred_x = new_x;
                let mut pred_y = new_y;
                let mut pred_vx = new_vx;
                let mut pred_vy = new_vy;
                
                for _ in 0..20 {
                    prediction_points.push((pred_x, pred_y));
                    
                    // Direction to target planet
                    let tdx = target_x - pred_x;
                    let tdy = target_y - pred_y;
                    let tdist = (tdx * tdx + tdy * tdy).sqrt();
                    
                    // Strong attraction to target
                    let pred_attraction = 0.15;
                    let mut pax = tdx / tdist.max(0.1) * pred_attraction;
                    let mut pay = tdy / tdist.max(0.1) * pred_attraction;
                    
                    // Weak gravity wells from OTHER planets (not target)
                    for (pidx, pdata) in PLANET_DATA.iter().enumerate() {
                        if pidx == target_idx { continue; }
                        
                        let pangle = PLANET_INITIAL_ANGLES[pidx] + porbits[pidx];
                        let ppx = 50.0 + pdata.0 * pangle.cos();
                        let ppy = 50.0 + pdata.0 * pangle.sin();
                        
                        let gdx = ppx - pred_x;
                        let gdy = ppy - pred_y;
                        let gdist = (gdx * gdx + gdy * gdy).sqrt().max(1.0);
                        
                        // Gravity proportional to planet size (mass proxy), inverse square law
                        let mass = pdata.1;
                        let gravity_strength = mass / (gdist * gdist) * 0.5; // weak influence
                        let gravity_force = gravity_strength.min(0.1);
                        
                        // Accumulate gravity influence
                        pax += gdx / gdist * gravity_force;
                        pay += gdy / gdist * gravity_force;
                    }
                    
                    // Update prediction velocity and position
                    let pdrag = 0.92;
                    pred_vx = (pred_vx + pax) * pdrag;
                    pred_vy = (pred_vy + pay) * pdrag;
                    pred_x += pred_vx;
                    pred_y += pred_vy;
                }
                set_flight_prediction.set(prediction_points);

                // Check if arrived (slow enough and close enough)
                let speed = (new_vx * new_vx + new_vy * new_vy).sqrt();
                if dist < 0.5 && speed < 0.3 {
                    set_fly_x.set(target_x);
                    set_fly_y.set(target_y);
                    set_fly_vx.set(0.0);
                    set_fly_vy.set(0.0);
                    set_is_flying.set(false);
                    set_is_arriving.set(true);
                    // Clear flight path after 3 seconds (handled in component)
                    if let Some(idx) = target_planet_idx.get() {
                        let porbits = planet_angles.get();
                        let planet_angle = PLANET_INITIAL_ANGLES[idx] + porbits[idx];
                        let planet_x = 50.0 + PLANET_DATA[idx].0 * planet_angle.cos();
                        let planet_y = 50.0 + PLANET_DATA[idx].0 * planet_angle.sin();
                        set_planet_offset.set((new_y - planet_y).atan2(new_x - planet_x));
                    }
                }
            } else if target_planet_idx.get().is_none() {
                // Not flying and no target planet - update spaceship angle
                // Clear prediction when not flying
                if !flight_prediction.get().is_empty() {
                    set_flight_prediction.set(Vec::new());
                }
                // Reset arriving flag after brief moment and fade flight path
                if is_arriving.get() {
                    let current_time = js_sys::Date::now() as f64;
                    use std::sync::OnceLock;
                    static ARRIVAL_START: OnceLock<f64> = OnceLock::new();
                    if ARRIVAL_START.get().is_none() {
                        ARRIVAL_START.set(current_time).ok();
                    }
                    if let Some(&start) = ARRIVAL_START.get() {
                        if current_time - start > 300.0 {
                            set_is_arriving.set(false);
                            // Fade out flight path after arrival
                            set_flight_path.update(|path: &mut Vec<(f64, f64)>| {
                                path.clear();
                            });
                        }
                    }
                }
                set_spaceship_angle.update(|a| *a += 0.01);
            } else {
                // Not flying but has target planet (orbiting) - clear prediction
                if !flight_prediction.get().is_empty() {
                    set_flight_prediction.set(Vec::new());
                }
            }
        });
        interval.forget();
    });

    // Selection handlers
    let select_planet = move |idx: usize| {
        if is_flying.get() { return; }
        if target_planet_idx.get() == Some(idx) { return; }

        let ship_angle: f64 = spaceship_angle.get();
        let ship_x: f64 = 50.0 + SHIP_ORBIT_RADIUS as f64 * ship_angle.cos();
        let ship_y: f64 = 50.0 + SHIP_ORBIT_RADIUS as f64 * ship_angle.sin();

        let porbits = planet_angles.get();
        let planet_angle = PLANET_INITIAL_ANGLES[idx] + porbits[idx];
        let planet_orbit = PLANET_DATA[idx].0;
        let planet_x = 50.0 + planet_orbit * planet_angle.cos();
        let planet_y = 50.0 + planet_orbit * planet_angle.sin();

        set_fly_from_x.set(ship_x);
        set_fly_from_y.set(ship_y);
        set_fly_to_x.set(planet_x);
        set_fly_to_y.set(planet_y);
        set_fly_x.set(ship_x);
        set_fly_y.set(ship_y);
        set_fly_vx.set(0.0);
        set_fly_vy.set(0.0);
        set_is_flying.set(true);
        set_target_planet_idx.set(Some(idx));
    };

    let select_sun = move || {
        if is_flying.get() { return; }
        if target_planet_idx.get().is_none() { return; }
        set_target_planet_idx.set(None);
        set_planet_offset.set(0.0);
    };

    let buy_upgrade = move |upgrade: UpgradeType| {
        let current_time: u64 = js_sys::Date::now() as u64;
        let ship_angle: f64 = spaceship_angle.get();
        let ship_x: f64 = 50.0 + SHIP_ORBIT_RADIUS as f64 * ship_angle.cos();
        let ship_y: f64 = 50.0 + SHIP_ORBIT_RADIUS as f64 * ship_angle.sin();

        set_state.update(|s| { s.buy_upgrade(upgrade, current_time); });
        set_last_purchase_time.set(current_time);

        let new_effect = UpgradeEffect::new(js_sys::Date::now() as u64, upgrade, ship_x, ship_y);
        set_upgrade_effects.update(|effects: &mut Vec<UpgradeEffect>| effects.push(new_effect));
    };

    view! {
        <div class="flex flex-col h-full bg-transparent text-app-text overflow-hidden transition-all duration-500 font-mono">
            <components::GameHeader
                state={state}
                target_planet_idx={target_planet_idx}
                on_select_sun={move || { select_sun(); }}
                on_select_planet={move |idx| { select_planet(idx); }}
            />

            {/* Solar System */}
            <div class="relative flex-1 overflow-hidden">
                <components::NeonOrb intensity={Some(state.get().energy_per_second() as i32)}/>

                <SolarSystem
                    planet_angles={planet_angles}
                    moon_angles={moon_angles}
                    spaceship_angle={spaceship_angle}
                    target_planet_idx={target_planet_idx}
                    is_flying={is_flying}
                    fly_progress={fly_progress}
                    fly_from_x={fly_from_x}
                    fly_from_y={fly_from_y}
                    fly_to_x={fly_to_x}
                    fly_to_y={fly_to_y}
                    fly_x={fly_x}
                    fly_y={fly_y}
                    planet_offset={planet_offset}
                    upgrade_effects={upgrade_effects}
                    trail_positions={trail_positions}
                    flight_path={flight_path}
                    flight_prediction={flight_prediction}
                    is_arriving={is_arriving}
                />
            </div>

            {/* Upgrade Panel */}
            <div class="w-full flex flex-col items-center pb-4 shrink-0 relative z-10 pointer-events-auto">
                <components::UpgradePanel state={state} on_buy={Callback::from(buy_upgrade)}/>
            </div>

            <Show when={move || architect_message.get().is_some()}>
                <components::ArchitectToast
                    message={architect_message.get().unwrap_or_default()}
                    on_close={Callback::from(move |_| set_architect_message.set(None))}
                />
            </Show>

            <Show when={move || show_how_to_play.get()}>
                <components::HowToPlayModal on_close={Callback::from(move |_| set_show_how_to_play.set(false))} />
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
