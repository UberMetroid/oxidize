//! Physics helper functions for the game loop.
//!
//! Contains pure-ish physics utilities used by the main game loop.

use crate::constants::*;
use crate::types::UpgradeEffect;

/// Advance upgrade effect progress by elapsed time.
pub fn advance_upgrade_effects(effects: &mut Vec<UpgradeEffect>, current_time: f64) {
    effects.retain_mut(|effect| {
        let elapsed = (current_time - effect.start_time) / 1000.0;
        let duration = effect.duration_secs();
        if elapsed < duration || effect.permanent {
            effect.progress = (elapsed / duration).min(1.0);
            true
        } else {
            false
        }
    });
}

/// Compute gravity-bent flight prediction from current state.
pub fn compute_flight_prediction(
    start_x: f64, start_y: f64,
    start_vx: f64, start_vy: f64,
    target_x: f64, target_y: f64,
    planet_angles: &[f64],
    target_planet_idx: usize,
) -> Vec<(f64, f64)> {
    let mut points = Vec::new();
    let mut px = start_x;
    let mut py = start_y;
    let mut pvx = start_vx;
    let mut pvy = start_vy;

    for _ in 0..20 {
        points.push((px, py));

        let dx = target_x - px;
        let dy = target_y - py;
        let dist = (dx * dx + dy * dy).sqrt();

        let attraction = 0.15;
        let mut pax = dx / dist.max(0.1) * attraction;
        let mut pay = dy / dist.max(0.1) * attraction;

        for (pidx, pdata) in PLANET_DATA.iter().enumerate() {
            if pidx == target_planet_idx { continue; }

            let pangle = PLANET_INITIAL_ANGLES[pidx] + planet_angles[pidx];
            let ppx = 50.0 + pdata.0 * pangle.cos();
            let ppy = 50.0 + pdata.0 * pangle.sin();

            let gdx = ppx - px;
            let gdy = ppy - py;
            let gdist = (gdx * gdx + gdy * gdy).sqrt().max(1.0);
            let mass = pdata.1;
            let gravity_strength = (mass / (gdist * gdist) * 0.5).min(0.1);

            pax += gdx / gdist * gravity_strength;
            pay += gdy / gdist * gravity_strength;
        }

        pvx = (pvx + pax) * 0.92;
        pvy = (pvy + pay) * 0.92;
        px += pvx;
        py += pvy;
    }

    points
}

/// Compute ship world position from game state signals.
pub fn ship_world_pos(
    spaceship_angle: f64,
    target_planet_idx: Option<usize>,
    is_flying: bool,
    planet_angles: &[f64],
    planet_offset: f64,
    fly_x: f64, fly_y: f64,
    fly_from_x: f64, fly_from_y: f64,
) -> (f64, f64) {
    if is_flying {
        (fly_x, fly_y)
    } else if let Some(idx) = target_planet_idx {
        let planet_angle = PLANET_INITIAL_ANGLES[idx] + planet_angles[idx];
        let planet_orbit_r = PLANET_DATA[idx].0;
        let planet_x = 50.0 + planet_orbit_r * planet_angle.cos();
        let planet_y = 50.0 + planet_orbit_r * planet_angle.sin();
        let ship_angle = planet_angle + planet_offset;
        let sx = planet_x + SHIP_PLANET_ORBIT_RADIUS * ship_angle.cos();
        let sy = planet_y + SHIP_PLANET_ORBIT_RADIUS * ship_angle.sin();
        (sx, sy)
    } else {
        (
            50.0 + SHIP_ORBIT_RADIUS as f64 * spaceship_angle.cos(),
            50.0 + SHIP_ORBIT_RADIUS as f64 * spaceship_angle.sin(),
        )
    }
}
