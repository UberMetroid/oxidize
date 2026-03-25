//! Asteroids and collectible entities for the Saturn arena.

use serde::{Deserialize, Serialize};

/// A floating entity in the Saturn arena — asteroid or moon.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Entity {
    /// World X (0-100)
    pub x: f64,
    /// World Y (0-100)
    pub y: f64,
    /// Velocity X
    pub vx: f64,
    /// Velocity Y
    pub vy: f64,
    /// Orbital angle around Saturn (if orbiting)
    pub orbit_angle: f64,
    /// Orbital radius around Saturn
    pub orbit_r: f64,
    /// Entity radius for collision
    pub radius: f64,
    /// Energy awarded on collection
    pub energy: f64,
    /// Whether already collected
    pub collected: bool,
    /// Sprite style
    pub style: EntityStyle,
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum EntityStyle {
    Moon,
    Debris,
}

impl Entity {
    /// Create a moon orbiting Saturn at the given angle.
    pub fn moon(x: f64, y: f64, orbit_angle: f64, orbit_r: f64, radius: f64, energy: f64) -> Self {
        Self { x, y, vx: 0.0, vy: 0.0, orbit_angle, orbit_r, radius, energy, collected: false, style: EntityStyle::Moon }
    }

    /// Create a debris piece drifting through space.
    pub fn debris(x: f64, y: f64, vx: f64, vy: f64, radius: f64, energy: f64) -> Self {
        Self { x, y, vx, vy, orbit_angle: 0.0, orbit_r: 0.0, radius, energy, collected: false, style: EntityStyle::Debris }
    }

    /// Advance orbiting entities (moons) by one tick.
    pub fn update_orbit(&mut self, dt: f64) {
        if self.style != EntityStyle::Moon { return; }
        self.orbit_angle += dt * 0.3 / (self.orbit_r / 10.0);
        self.x = 50.0 + self.orbit_r * self.orbit_angle.cos();
        self.y = 50.0 + self.orbit_r * self.orbit_angle.sin();
    }

    /// Advance drifting entities (debris) by one tick.
    pub fn update_drift(&mut self) {
        if self.style != EntityStyle::Debris { return; }
        self.x += self.vx;
        self.y += self.vy;
        // Wrap around
        if self.x < 0.0 { self.x += 100.0; }
        if self.x > 100.0 { self.x -= 100.0; }
        if self.y < 0.0 { self.y += 100.0; }
        if self.y > 100.0 { self.y -= 100.0; }
    }

    /// Check if a ship at (sx, sy) with collector radius collects this.
    pub fn in_range(&self, sx: f64, sy: f64, range: f64) -> bool {
        if self.collected { return false; }
        let dx = self.x - sx; let dy = self.y - sy;
        let dist = (dx*dx + dy*dy).sqrt();
        dist < (self.radius + range)
    }
}
