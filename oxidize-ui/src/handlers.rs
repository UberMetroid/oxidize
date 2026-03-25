//! Game event handlers
//! 
//! Contains closures for handling user interactions:
//! - make_select_planet: Handler for clicking a planet to fly to
//! - make_select_sun: Handler for returning to sun orbit
//! - make_buy_upgrade: Handler for purchasing upgrades

use leptos::*;
use oxidize_engine::UpgradeType;

use crate::constants::*;
use crate::signals::GameSignals;
use crate::types::UpgradeEffect;

/// Creates a closure for selecting a planet to fly to
pub fn make_select_planet(signals: &GameSignals) -> impl Fn(usize) + Clone + 'static {
    let is_flying = signals.is_flying.0;
    let target_planet_idx = signals.target_planet_idx.0;
    let target_planet_idx_set = signals.target_planet_idx.1;
    let spaceship_angle = signals.spaceship_angle.0;
    let fly_from_x = signals.fly_from_x;
    let fly_from_y = signals.fly_from_y;
    let fly_to_x = signals.fly_to_x;
    let fly_to_y = signals.fly_to_y;
    let fly_x = signals.fly_x;
    let fly_y = signals.fly_y;
    let fly_vx = signals.fly_vx;
    let fly_vy = signals.fly_vy;
    let is_flying_set = signals.is_flying.1;

    move |idx: usize| {
        if is_flying.get() { return; }
        if target_planet_idx.get() == Some(idx) { return; }

        let ship_angle: f64 = spaceship_angle.get();
        let ship_x: f64 = 50.0 + SHIP_ORBIT_RADIUS as f64 * ship_angle.cos();
        let ship_y: f64 = 50.0 + SHIP_ORBIT_RADIUS as f64 * ship_angle.sin();

        // We need planet_angles from the game loop, but this handler
        // needs access to it. For now, calculate based on current angle of 0
        // The actual position will be correct when the ship arrives
        let planet_orbit = PLANET_DATA[idx].0;
        let planet_x = 50.0 + planet_orbit * PLANET_INITIAL_ANGLES[idx].cos();
        let planet_y = 50.0 + planet_orbit * PLANET_INITIAL_ANGLES[idx].sin();

        fly_from_x.1.set(ship_x);
        fly_from_y.1.set(ship_y);
        fly_to_x.1.set(planet_x);
        fly_to_y.1.set(planet_y);
        fly_x.1.set(ship_x);
        fly_y.1.set(ship_y);
        fly_vx.1.set(0.0);
        fly_vy.1.set(0.0);
        is_flying_set.set(true);
        target_planet_idx_set.set(Some(idx));
    }
}

/// Creates a closure for returning to sun orbit
pub fn make_select_sun(signals: &GameSignals) -> impl Fn() + Clone + 'static {
    let is_flying = signals.is_flying.0;
    let target_planet_idx = signals.target_planet_idx.0;
    let target_planet_idx_set = signals.target_planet_idx.1;
    let planet_offset_set = signals.planet_offset.1;

    move || {
        if is_flying.get() { return; }
        if target_planet_idx.get().is_none() { return; }
        target_planet_idx_set.set(None);
        planet_offset_set.set(0.0);
    }
}

/// Creates a closure for buying an upgrade
pub fn make_buy_upgrade(signals: &GameSignals) -> impl Fn(UpgradeType) + Clone + 'static {
    let state_set = signals.state.1;
    let spaceship_angle = signals.spaceship_angle.0;
    let upgrade_effects = signals.upgrade_effects;
    let last_purchase_time_set = signals.last_purchase_time.1;

    move |upgrade: UpgradeType| {
        let current_time: u64 = js_sys::Date::now() as u64;
        let ship_angle: f64 = spaceship_angle.get();
        let ship_x: f64 = 50.0 + SHIP_ORBIT_RADIUS as f64 * ship_angle.cos();
        let ship_y: f64 = 50.0 + SHIP_ORBIT_RADIUS as f64 * ship_angle.sin();

        state_set.update(|s| { s.buy_upgrade(upgrade, current_time); });
        last_purchase_time_set.set(current_time);

        let new_effect = UpgradeEffect::new(js_sys::Date::now() as u64, upgrade, ship_x, ship_y);
        upgrade_effects.1.update(|effects: &mut Vec<UpgradeEffect>| effects.push(new_effect));
    }
}
