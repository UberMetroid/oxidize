//! Ship physics and entity update logic.

use super::entity::Entity;
use crate::asteroids::keys::Keys;

pub const THRUST: f64 = 0.12;
pub const ROTATE_SPEED: f64 = 0.06;
pub const FIRE_RANGE: f64 = 8.0;
pub const FIRE_COOLDOWN: f64 = 0.3;
pub const DRAG: f64 = 0.985;

/// Update ship state from keyboard input.
pub fn update_ship(s: &mut ShipState, k: &Keys, _dt: f64) {
    if k.left  { s.angle -= ROTATE_SPEED; }
    if k.right { s.angle += ROTATE_SPEED; }
    if k.up {
        s.vx += s.angle.cos() * THRUST;
        s.vy += s.angle.sin() * THRUST;
    }
    if k.down {
        s.vx -= s.angle.cos() * THRUST * 0.4;
        s.vy -= s.angle.sin() * THRUST * 0.4;
    }
    s.vx *= DRAG;
    s.vy *= DRAG;
    s.x += s.vx;
    s.y += s.vy;
    s.x = if s.x < 0.0 { s.x + 100.0 } else if s.x > 100.0 { s.x - 100.0 } else { s.x };
    s.y = if s.y < 0.0 { s.y + 100.0 } else if s.y > 100.0 { s.y - 100.0 } else { s.y };
    if k.fire && s.fire_timer <= 0.0 { s.firing = true; s.fire_timer = FIRE_COOLDOWN; }
    else { s.firing = false; }
    s.fire_timer = (s.fire_timer - _dt).max(0.0);
}

/// Update all entities by one tick.
pub fn update_entities(ents: &mut [Entity], dt: f64) {
    for e in ents.iter_mut() {
        match e.style {
            super::entity::EntityStyle::Moon => e.update_orbit(dt),
            super::entity::EntityStyle::Debris => e.update_drift(),
        }
    }
}

/// Check collisions and return total energy collected.
pub fn check_collisions(s: &ShipState, ents: &[Entity]) -> f64 {
    let mut total = 0.0;
    for e in ents.iter() {
        if e.in_range(s.x, s.y, FIRE_RANGE) && s.firing {
            total += e.energy;
        }
    }
    total
}

#[derive(Default, Clone, Copy)]
pub struct ShipState {
    pub x: f64, pub y: f64,
    pub vx: f64, pub vy: f64,
    pub angle: f64,
    pub firing: bool,
    pub fire_timer: f64,
}
