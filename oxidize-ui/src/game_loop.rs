//! Game loop - timers and physics update functions
//! 
//! Contains:
//! - setup_physics_timer: Main physics and animation loop
//! - setup_sync_timer: Interval for server sync
//! - setup_architect_timer: Interval for idle check

use leptos::*;
use std::f64::consts::PI;

use crate::constants::*;
use crate::signals::GameSignals;
use crate::types::UpgradeEffect;
use crate::{api, helpers::save_state};

/// Sets up the physics timer for flight simulation and animation (16ms interval)
pub fn setup_physics_timer(signals: &GameSignals) {
    // Get read and write signals separately
    let spaceship_angle_read = signals.spaceship_angle.0;
    let spaceship_angle_write = signals.spaceship_angle.1;
    let target_planet_idx_read = signals.target_planet_idx.0;
    let target_planet_idx_write = signals.target_planet_idx.1;
    let is_flying_read = signals.is_flying.0;
    let is_flying_write = signals.is_flying.1;
    let fly_x_read = signals.fly_x.0;
    let fly_x_write = signals.fly_x.1;
    let fly_y_read = signals.fly_y.0;
    let fly_y_write = signals.fly_y.1;
    let fly_vx_read = signals.fly_vx.0;
    let fly_vx_write = signals.fly_vx.1;
    let fly_vy_read = signals.fly_vy.0;
    let fly_vy_write = signals.fly_vy.1;
    let fly_to_x_read = signals.fly_to_x.0;
    let fly_to_y_read = signals.fly_to_y.0;
    let fly_from_x_read = signals.fly_from_x.0;
    let fly_from_y_read = signals.fly_from_y.0;
    let planet_angles_read = signals.planet_angles.0;
    let planet_offset_read = signals.planet_offset.0;
    let planet_offset_write = signals.planet_offset.1;
    let trail_positions_read = signals.trail_positions.0;
    let trail_positions_write = signals.trail_positions.1;
    let flight_path_read = signals.flight_path.0;
    let flight_path_write = signals.flight_path.1;
    let flight_prediction_read = signals.flight_prediction.0;
    let flight_prediction_write = signals.flight_prediction.1;
    let is_arriving_read = signals.is_arriving.0;
    let is_arriving_write = signals.is_arriving.1;
    let arrival_time_read = signals.arrival_time.0;
    let arrival_time_write = signals.arrival_time.1;
    let upgrade_effects_write = signals.upgrade_effects.1;

    create_effect(move |_| {
        let mut frame_count = 0i32;
        let interval = gloo_timers::callback::Interval::new(16, move || {
            frame_count = (frame_count + 1) % 3;
            
            // Update upgrade effects progress
            upgrade_effects_write.update(|effects: &mut Vec<UpgradeEffect>| {
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
            let ship_angle: f64 = spaceship_angle_read.get();
            let porbits = planet_angles_read.get();
            let orbiting_idx = target_planet_idx_read.get();
            let flying = is_flying_read.get();
            let (world_x, world_y) = if flying {
                (fly_x_read.get(), fly_y_read.get())
            } else if let Some(idx) = orbiting_idx {
                let planet_angle = PLANET_INITIAL_ANGLES[idx] + porbits[idx];
                let planet_orbit_r = PLANET_DATA[idx].0;
                let planet_x = 50.0 + planet_orbit_r * planet_angle.cos();
                let planet_y = 50.0 + planet_orbit_r * planet_angle.sin();
                let ship_orbit_r = SHIP_PLANET_ORBIT_RADIUS;
                let offset = planet_offset_read.get();
                let ship_angle = planet_angle + offset;
                let sx = planet_x + ship_orbit_r * ship_angle.cos();
                let sy = planet_y + ship_orbit_r * ship_angle.sin();
                (sx, sy)
            } else {
                let x = 50.0 + SHIP_ORBIT_RADIUS as f64 * ship_angle.cos();
                let y = 50.0 + SHIP_ORBIT_RADIUS as f64 * ship_angle.sin();
                (x, y)
            };
            
            // Record trail positions every 3 frames
            if frame_count == 0 {
                trail_positions_write.update(|trail: &mut Vec<(f64, f64, f64)>| {
                    trail.push((world_x, world_y, 0.0));
                    if trail.len() > 20 { trail.remove(0); }
                });
            }
            trail_positions_write.update(|trail: &mut Vec<(f64, f64, f64)>| {
                for pos in trail.iter_mut() { pos.2 = (pos.2 + 0.05).min(1.0); }
                trail.retain(|p| p.2 < 1.0);
            });
            
            // Flight physics
            if is_flying_read.get() {
                let target_x = fly_to_x_read.get();
                let target_y = fly_to_y_read.get();
                let current_x = fly_x_read.get();
                let current_y = fly_y_read.get();
                let vx = fly_vx_read.get();
                let vy = fly_vy_read.get();

                let dx = target_x - current_x;
                let dy = target_y - current_y;
                let dist = (dx * dx + dy * dy).sqrt();

                let attraction_strength = 0.15;
                let ax = dx / dist.max(0.1) * attraction_strength;
                let ay = dy / dist.max(0.1) * attraction_strength;
                let drag = 0.92;

                let new_vx = (vx + ax) * drag;
                let new_vy = (vy + ay) * drag;
                let new_x = current_x + new_vx;
                let new_y = current_y + new_vy;

                fly_vx_write.set(new_vx);
                fly_vy_write.set(new_vy);
                fly_x_write.set(new_x);
                fly_y_write.set(new_y);

                // Track flight path
                if frame_count == 0 {
                    flight_path_write.update(|path: &mut Vec<(f64, f64)>| {
                        path.push((new_x, new_y));
                        if path.len() > 100 { path.remove(0); }
                    });
                }

                // Compute gravity-bent flight prediction
                let porbits = planet_angles_read.get();
                let target_idx = target_planet_idx_read.get().unwrap_or(0);
                let mut prediction_points = Vec::new();
                let mut pred_x = new_x;
                let mut pred_y = new_y;
                let mut pred_vx = new_vx;
                let mut pred_vy = new_vy;
                
                for _ in 0..20 {
                    prediction_points.push((pred_x, pred_y));
                    
                    let tdx = target_x - pred_x;
                    let tdy = target_y - pred_y;
                    let tdist = (tdx * tdx + tdy * tdy).sqrt();
                    
                    let pred_attraction = 0.15;
                    let mut pax = tdx / tdist.max(0.1) * pred_attraction;
                    let mut pay = tdy / tdist.max(0.1) * pred_attraction;
                    
                    for (pidx, pdata) in PLANET_DATA.iter().enumerate() {
                        if pidx == target_idx { continue; }
                        
                        let pangle = PLANET_INITIAL_ANGLES[pidx] + porbits[pidx];
                        let ppx = 50.0 + pdata.0 * pangle.cos();
                        let ppy = 50.0 + pdata.0 * pangle.sin();
                        
                        let gdx = ppx - pred_x;
                        let gdy = ppy - pred_y;
                        let gdist = (gdx * gdx + gdy * gdy).sqrt().max(1.0);
                        
                        let mass = pdata.1;
                        let gravity_strength = mass / (gdist * gdist) * 0.5;
                        let gravity_force = gravity_strength.min(0.1);
                        
                        pax += gdx / gdist * gravity_force;
                        pay += gdy / gdist * gravity_force;
                    }
                    
                    let pdrag = 0.92;
                    pred_vx = (pred_vx + pax) * pdrag;
                    pred_vy = (pred_vy + pay) * pdrag;
                    pred_x += pred_vx;
                    pred_y += pred_vy;
                }
                flight_prediction_write.set(prediction_points);

                // Check arrival
                let speed = (new_vx * new_vx + new_vy * new_vy).sqrt();
                if dist < 0.5 && speed < 0.3 {
                    fly_x_write.set(target_x);
                    fly_y_write.set(target_y);
                    fly_vx_write.set(0.0);
                    fly_vy_write.set(0.0);
                    is_flying_write.set(false);
                    is_arriving_write.set(true);
                    arrival_time_write.set(js_sys::Date::now());
                    if let Some(idx) = target_planet_idx_read.get() {
                        let porbits = planet_angles_read.get();
                        let planet_angle = PLANET_INITIAL_ANGLES[idx] + porbits[idx];
                        let planet_x = 50.0 + PLANET_DATA[idx].0 * planet_angle.cos();
                        let planet_y = 50.0 + PLANET_DATA[idx].0 * planet_angle.sin();
                        planet_offset_write.set((new_y - planet_y).atan2(new_x - planet_x));
                    }
                }
            } else if target_planet_idx_read.get().is_none() {
                if !flight_prediction_read.get().is_empty() {
                    flight_prediction_write.set(Vec::new());
                }
                if is_arriving_read.get() {
                    let elapsed = js_sys::Date::now() - arrival_time_read.get();
                    if elapsed > 300.0 {
                        is_arriving_write.set(false);
                        flight_path_write.update(|path: &mut Vec<(f64, f64)>| path.clear());
                    }
                }
                spaceship_angle_write.update(|a| *a += 0.01);
            } else {
                if !flight_prediction_read.get().is_empty() {
                    flight_prediction_write.set(Vec::new());
                }
            }
        });
        interval.forget();
    });
}

/// Sets up server sync timer (2000ms interval)
pub fn setup_sync_timer(signals: &GameSignals, player_uuid: String) {
    let state = signals.state.0;

    create_effect(move |_| {
        let uuid = player_uuid.clone();
        let interval = gloo_timers::callback::Interval::new(2000, move || {
            let mut current_state = state.get();
            current_state.last_synced_total_energy = current_state.total_energy_generated;
            save_state(&current_state);
            let uuid_clone = uuid.clone();
            let state_clone = current_state.clone();
            wasm_bindgen_futures::spawn_local(async move { 
                let _ = api::sync_state(&uuid_clone, &state_clone).await; 
            });
        });
        interval.forget();
    });
}

/// Sets up idle check timer (10000ms interval)
pub fn setup_architect_timer(signals: &GameSignals) {
    use oxidize_engine::architect::{generate_quip, QuipTrigger};
    
    let last_purchase_time = signals.last_purchase_time.0;
    let architect_message_set = signals.architect_message.1;

    create_effect(move |_| {
        let interval = gloo_timers::callback::Interval::new(10000, move || {
            let last_purchase = last_purchase_time.get();
            if last_purchase == 0 { return; }
            let seconds_idle = (js_sys::Date::now() as f64 - last_purchase as f64) / 1000.0;
            if seconds_idle >= 60.0 {
                architect_message_set.set(Some(generate_quip(QuipTrigger::Idle)));
            }
        });
        interval.forget();
    });
}
