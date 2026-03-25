//! 16ms physics + animation loop: orbits, ship flight, trail, upgrade effects.

use gloo_timers::callback::Interval;
use leptos::*;
use crate::constants::*;
use crate::physics::{advance_upgrade_effects, compute_flight_prediction, ship_world_pos};
use crate::signals::GameSignals;
use crate::types::UpgradeEffect;

/// Drives planet/moon orbits, ship physics, trail recording, and arrival logic.
pub fn setup_physics_timer(s: &GameSignals) {
    // Unpack signals for closure capture
    let sa_r = s.spaceship_angle.0;   let sa_w = s.spaceship_angle.1;
    let tpi_r = s.target_planet_idx.0;
    let if_r = s.is_flying.0;         let if_w = s.is_flying.1;
    let fx_r = s.fly_x.0;             let fx_w = s.fly_x.1;
    let fy_r = s.fly_y.0;             let fy_w = s.fly_y.1;
    let fvx_r = s.fly_vx.0;           let fvx_w = s.fly_vx.1;
    let fvy_r = s.fly_vy.0;           let fvy_w = s.fly_vy.1;
    let ftx_r = s.fly_to_x.0;
    let fty_r = s.fly_to_y.0;
    let ff_r = s.fly_from_x.0;
    let ffy_r = s.fly_from_y.0;
    let pa_r = s.planet_angles.0;    let pa_w = s.planet_angles.1;
    let ma_r = s.moon_angles.0;      let ma_w = s.moon_angles.1;
    let po_r = s.planet_offset.0;    let po_w = s.planet_offset.1;
    let tp_w = s.trail_positions.1;
    let fp_w = s.flight_path.1;
    let fpr_r = s.flight_prediction.0;
    let fpd_w = s.flight_prediction.1;
    let ia_r = s.is_arriving.0;       let ia_w = s.is_arriving.1;
    let at_r = s.arrival_time.0;     let at_w = s.arrival_time.1;
    let ue_w = s.upgrade_effects.1;

    create_effect(move |_| {
        let mut frame = 0i32;
        Interval::new(16, move || {
            frame = (frame + 1) % 3;

            // Orbit angles
            pa_w.update(|angles: &mut Vec<f64>| {
                for (i, a) in angles.iter_mut().enumerate() {
                    *a += (2.0 * std::f64::consts::PI) / (PLANET_PERIODS[i] * 60.0);
                    if *a > 2.0 * std::f64::consts::PI { *a -= 2.0 * std::f64::consts::PI; }
                }
            });
            ma_w.update(|angles: &mut Vec<f64>| {
                for (i, a) in angles.iter_mut().enumerate() {
                    *a += (2.0 * std::f64::consts::PI) / (MOON_PERIODS_FLAT[i] * 60.0);
                    if *a > 2.0 * std::f64::consts::PI { *a -= 2.0 * std::f64::consts::PI; }
                }
            });

            // Upgrade timers
            ue_w.update(|effs: &mut Vec<UpgradeEffect>| advance_upgrade_effects(effs, js_sys::Date::now()));

            // Ship world pos
            let (wx, wy) = ship_world_pos(
                sa_r.get(), tpi_r.get(), if_r.get(), &pa_r.get(),
                po_r.get(), fx_r.get(), fy_r.get(), ff_r.get(), ffy_r.get(),
            );

            // Trail (every 3 frames)
            if frame == 0 {
                tp_w.update(|t: &mut Vec<(f64,f64,f64)>| {
                    t.push((wx, wy, 0.0)); if t.len() > 20 { t.remove(0); }
                });
            }
            tp_w.update(|t: &mut Vec<(f64,f64,f64)>| {
                for p in t.iter_mut() { p.2 = (p.2 + 0.05).min(1.0); }
                t.retain(|p| p.2 < 1.0);
            });

            // Flight physics
            if if_r.get() {
                let tx = ftx_r.get(); let ty = fty_r.get();
                let cx = fx_r.get(); let cy = fy_r.get();
                let vx = fvx_r.get(); let vy = fvy_r.get();
                let dx = tx - cx; let dy = ty - cy;
                let dist = (dx*dx + dy*dy).sqrt();
                let pull = 0.15;
                let nvx = (vx + dx/dist.max(0.1) * pull) * 0.92;
                let nvy = (vy + dy/dist.max(0.1) * pull) * 0.92;
                let nx = cx + nvx; let ny = cy + nvy;
                fx_w.set(nx); fy_w.set(ny); fvx_w.set(nvx); fvy_w.set(nvy);
                if frame == 0 { fp_w.update(|p: &mut Vec<(f64,f64)>| { p.push((nx, ny)); if p.len() > 100 { p.remove(0); } }); }
                let porbits = pa_r.get();
                let tidx = tpi_r.get().unwrap_or(0);
                fpd_w.set(compute_flight_prediction(nx, ny, nvx, nvy, tx, ty, &porbits, tidx));
                let speed = (nvx*nvx + nvy*nvy).sqrt();
                if dist < 0.5 && speed < 0.3 {
                    fx_w.set(tx); fy_w.set(ty); fvx_w.set(0.0); fvy_w.set(0.0);
                    if_w.set(false); ia_w.set(true); at_w.set(js_sys::Date::now());
                    if let Some(i) = tpi_r.get() {
                        let a = PLANET_INITIAL_ANGLES[i] + porbits[i];
                        let px = 50.0 + PLANET_DATA[i].0 * a.cos();
                        let py = 50.0 + PLANET_DATA[i].0 * a.sin();
                        po_w.set((ny - py).atan2(nx - px));
                    }
                }
            } else if tpi_r.get().is_none() {
                if !fpr_r.get().is_empty() { fpd_w.set(Vec::new()); }
                if ia_r.get() && js_sys::Date::now() - at_r.get() > 300.0 {
                    ia_w.set(false);
                    fp_w.update(|p: &mut Vec<(f64,f64)>| p.clear());
                }
                sa_w.update(|a| *a += 0.01);
            } else {
                if !fpr_r.get().is_empty() { fpd_w.set(Vec::new()); }
            }
        }).forget();
    });
}
