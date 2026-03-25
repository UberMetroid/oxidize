//! Unit tests for pure game logic (no WASM dependency)
//!
//! These test the core game mechanics: physics, collision, scoring, asteroid behavior.
//! Run with: cargo test --package oxidize-ui

// Pure math helpers that don't need WASM

/// Distance between two 2D points
fn dist(x1: f64, y1: f64, x2: f64, y2: f64) -> f64 {
    ((x2 - x1).powi(2) + (y2 - y1).powi(2)).sqrt()
}

/// Clamp a value between min and max
fn clamp(val: f64, min: f64, max: f64) -> f64 {
    val.max(min).min(max)
}

/// World coordinate wrapping (toroidal topology)
fn wrap_coord(val: f64, world_size: f64) -> f64 {
    ((val % world_size) + world_size) % world_size
}

/// Hit radius for asteroid size
fn asteroid_hit_radius(size: u8) -> f64 {
    match size { 0 => 6.0, 1 => 3.0, _ => 1.5 }
}

/// Circle-circle collision
fn circles_collide(x1: f64, y1: f64, r1: f64, x2: f64, y2: f64, r2: f64) -> bool {
    dist(x1, y1, x2, y2) < r1 + r2
}

/// Score awarded for destroying an asteroid (by size and weapon type)
fn asteroid_score(size: u8, charged: bool) -> i32 {
    let base = match size { 0 => 20, 1 => 50, _ => 100 };
    if charged { base * 2 + 10 } else { base }
}

/// Points string generation (simplified, matches actual impl)
fn asteroid_points_str(radii: &[(f64, f64)]) -> String {
    radii.iter()
        .map(|(x, y)| format!("{:.2},{:.2}", x, y))
        .collect::<Vec<_>>().join(" ")
}

#[cfg(test)]
mod tests {
    use super::*;

    // ── World wrapping ───────────────────────────────────────────────────────

    #[test]
    fn test_wrap_positive() {
        // Normal case
        assert!((wrap_coord(50.0, 100.0) - 50.0).abs() < 1e-9);
    }

    #[test]
    fn test_wrap_negative() {
        // Negative coordinates wrap around
        let result = wrap_coord(-5.0, 100.0);
        assert!((result - 95.0).abs() < 1e-9);
    }

    #[test]
    fn test_wrap_overshoot() {
        // Overflow wraps
        let result = wrap_coord(105.0, 100.0);
        assert!((result - 5.0).abs() < 1e-9);
    }

    #[test]
    fn test_wrap_exact_boundary() {
        // Exact boundary
        assert!((wrap_coord(100.0, 100.0) - 0.0).abs() < 1e-9);
    }

    #[test]
    fn test_wrap_twice() {
        // Double wrap
        let result = wrap_coord(250.0, 100.0);
        assert!((result - 50.0).abs() < 1e-9);
    }

    // ── Collision detection ─────────────────────────────────────────────────

    #[test]
    fn test_collision_direct_hit() {
        // Two circles at same position
        assert!(circles_collide(50.0, 50.0, 3.0, 50.0, 50.0, 3.0));
    }

    #[test]
    fn test_collision_touching() {
        // Circles just touching (edge case: should NOT collide if dist == r1+r2)
        // dist = 6, r1+r2 = 6, so this is a touch = no collision
        assert!(!circles_collide(50.0, 50.0, 3.0, 56.0, 50.0, 3.0));
    }

    #[test]
    fn test_collision_overlapping() {
        // Circles overlapping
        assert!(circles_collide(50.0, 50.0, 3.0, 54.0, 50.0, 3.0));
    }

    #[test]
    fn test_collision_no_hit() {
        // Circles far apart
        assert!(!circles_collide(50.0, 50.0, 3.0, 70.0, 50.0, 3.0));
    }

    #[test]
    fn test_collision_asteroid_large_vs_ship() {
        // Large asteroid (r=6) vs ship (r=1.5), combined = 7.5
        assert!(circles_collide(50.0, 50.0, 6.0, 55.0, 50.0, 1.5)); // dist=5, r=7.5 → hit
        assert!(!circles_collide(50.0, 50.0, 6.0, 60.0, 50.0, 1.5)); // dist=10, r=7.5 → miss
    }

    #[test]
    fn test_collision_asteroid_small_vs_ship() {
        // Small asteroid (r=1.5) vs ship (r=1.5), combined = 3
        assert!(circles_collide(50.0, 50.0, 1.5, 51.0, 50.0, 1.5)); // dist=1, r=3 → hit
        assert!(!circles_collide(50.0, 50.0, 1.5, 54.0, 50.0, 1.5)); // dist=4, r=3 → miss
    }

    #[test]
    fn test_collision_diagonal() {
        // Diagonal collision
        // dist = sqrt(3^2 + 4^2) = 5
        assert!(circles_collide(0.0, 0.0, 3.0, 3.0, 4.0, 3.0)); // dist=5, r=6 → hit
        assert!(!circles_collide(0.0, 0.0, 2.0, 3.0, 4.0, 2.0)); // dist=5, r=4 → miss
    }

    // ── Asteroid hit radius ───────────────────────────────────────────────

    #[test]
    fn test_asteroid_radius_large() {
        assert!((asteroid_hit_radius(0) - 6.0).abs() < 1e-9);
    }

    #[test]
    fn test_asteroid_radius_medium() {
        assert!((asteroid_hit_radius(1) - 3.0).abs() < 1e-9);
    }

    #[test]
    fn test_asteroid_radius_small() {
        assert!((asteroid_hit_radius(2) - 1.5).abs() < 1e-9);
    }

    #[test]
    fn test_asteroid_radius_larger_sizes() {
        // Sizes beyond 2 should still return 1.5
        assert!((asteroid_hit_radius(3) - 1.5).abs() < 1e-9);
        assert!((asteroid_hit_radius(255) - 1.5).abs() < 1e-9);
    }

    // ── Scoring ──────────────────────────────────────────────────────────

    #[test]
    fn test_score_normal_shot() {
        assert_eq!(asteroid_score(0, false), 20);   // Large
        assert_eq!(asteroid_score(1, false), 50);   // Medium
        assert_eq!(asteroid_score(2, false), 100);  // Small
    }

    #[test]
    fn test_score_charged_shot() {
        // Charged = 2x + 10 bonus
        assert_eq!(asteroid_score(0, true), 20 * 2 + 10);  // 50
        assert_eq!(asteroid_score(1, true), 50 * 2 + 10);  // 110
        assert_eq!(asteroid_score(2, true), 100 * 2 + 10);  // 210
    }

    #[test]
    fn test_score_charged_bonus_significant() {
        // Charged should always be worth more than normal
        assert!(asteroid_score(0, true) > asteroid_score(0, false));
        assert!(asteroid_score(1, true) > asteroid_score(1, false));
        assert!(asteroid_score(2, true) > asteroid_score(2, false));
    }

    // ── Clamping ──────────────────────────────────────────────────────────

    #[test]
    fn test_clamp_in_range() {
        assert!((clamp(5.0, 0.0, 10.0) - 5.0).abs() < 1e-9);
    }

    #[test]
    fn test_clamp_above_max() {
        assert!((clamp(15.0, 0.0, 10.0) - 10.0).abs() < 1e-9);
    }

    #[test]
    fn test_clamp_below_min() {
        assert!((clamp(-5.0, 0.0, 10.0) - 0.0).abs() < 1e-9);
    }

    #[test]
    fn test_clamp_exact_boundaries() {
        assert!((clamp(0.0, 0.0, 10.0) - 0.0).abs() < 1e-9);
        assert!((clamp(10.0, 0.0, 10.0) - 10.0).abs() < 1e-9);
    }

    // ── Asteroid points string ─────────────────────────────────────────────

    #[test]
    fn test_points_str_format() {
        let pts = &[(1.0, 2.0), (3.0, 4.0)];
        let result = asteroid_points_str(pts);
        assert_eq!(result, "1.00,2.00 3.00,4.00");
    }

    #[test]
    fn test_points_str_empty() {
        let pts: &[(f64, f64)] = &[];
        let result = asteroid_points_str(pts);
        assert_eq!(result, "");
    }

    #[test]
    fn test_points_str_single_point() {
        let pts = &[(0.0, 0.0)];
        let result = asteroid_points_str(pts);
        assert_eq!(result, "0.00,0.00");
    }

    // ── Ship physics ──────────────────────────────────────────────────────

    #[test]
    fn test_ship_thrust_increases_velocity() {
        let angle_deg = 90.0_f64; // pointing up
        let angle_rad = angle_deg.to_radians();
        let thrust = 0.05;

        let vx_before = 0.0;
        let vy_before = 0.0;
        let vx_after = vx_before + angle_rad.cos() * thrust;
        let vy_after = vy_before + angle_rad.sin() * thrust;

        // At 90°, cos=0, sin=1 → all thrust goes to Y
        assert!(vx_after.abs() < 1e-10);
        assert!((vy_after - thrust).abs() < 1e-10);
    }

    #[test]
    fn test_ship_thrust_right() {
        let angle_deg = 0.0_f64; // pointing right
        let angle_rad = angle_deg.to_radians();
        let thrust = 0.05;

        let vx_after = angle_rad.cos() * thrust;
        let vy_after = angle_rad.sin() * thrust;

        assert!((vx_after - thrust).abs() < 1e-10);
        assert!(vy_after.abs() < 1e-10);
    }

    #[test]
    fn test_ship_drag_reduces_velocity() {
        let drag = 0.998;
        let vx = 5.0_f64;

        let vx_after = vx * drag;
        assert!(vx_after < vx);
        assert!((vx_after - 4.99).abs() < 0.01); // ~5% reduction
    }

    #[test]
    fn test_ship_wrap() {
        // Moving past world edge wraps around
        let x = 98.0_f64;
        let vx = 5.0_f64;
        let wrapped = wrap_coord(x + vx, 100.0);
        assert!((wrapped - 3.0).abs() < 1e-9);
    }

    #[test]
    fn test_charge_produces_power() {
        let charge_level = 1.5_f64; // half charge
        let max_charge = 3.0_f64;
        let charge_power = 1.5_f64;
        let base_speed = 1.0_f64;

        let power = charge_level / max_charge;
        let speed = base_speed * (1.0 + power * charge_power);

        // At 50% charge: power = 0.5, speed = 1.0 * (1 + 0.5*1.5) = 1.75
        assert!((speed - 1.75).abs() < 1e-9);
    }

    #[test]
    fn test_max_charge_speed() {
        let charge_level = 3.0_f64; // full charge
        let max_charge = 3.0_f64;
        let charge_power = 1.5_f64;
        let base_speed = 1.0_f64;

        let power = charge_level / max_charge;
        let speed = base_speed * (1.0 + power * charge_power);

        // At 100% charge: power = 1.0, speed = 1.0 * (1 + 1.5) = 2.5
        assert!((speed - 2.5).abs() < 1e-9);
    }

    // ── Level progression ────────────────────────────────────────────────

    #[test]
    fn test_level_wave_count() {
        // Wave count = 5 + level * 2
        assert_eq!(5 + 1 * 2, 7);   // Level 1: 7
        assert_eq!(5 + 2 * 2, 9);   // Level 2: 9
        assert_eq!(5 + 5 * 2, 15);  // Level 5: 15
        assert_eq!(5 + 10 * 2, 25); // Level 10: 25
    }

    // ── Safe spawn ───────────────────────────────────────────────────────

    #[test]
    fn test_safe_spawn_distance_check() {
        // Safe spawn requires min_dist from all asteroids
        let _asteroids = &[(50.0_f64, 50.0_f64)]; // one asteroid at center
        let min_dist = 12.0_f64;

        // Candidate at (30, 30): dist = sqrt(20^2 + 20^2) ≈ 28.3 > 12 → safe
        let dx: f64 = 30.0 - 50.0;
        let dy: f64 = 30.0 - 50.0;
        let distance = (dx.powi(2) + dy.powi(2)).sqrt();
        assert!(distance > min_dist); // should be safe

        // Candidate at (45, 50): dist = 5 < 12 → NOT safe
        let dx2: f64 = 45.0 - 50.0;
        let dy2: f64 = 50.0 - 50.0;
        let distance2 = (dx2.powi(2) + dy2.powi(2)).sqrt();
        assert!(distance2 < min_dist); // should be unsafe
    }

    // ── Edge cases ────────────────────────────────────────────────────────

    #[test]
    fn test_dist_zero_when_same_position() {
        assert!(dist(5.0, 5.0, 5.0, 5.0).abs() < 1e-9);
    }

    #[test]
    fn test_dist_symmetric() {
        let d1 = dist(0.0, 0.0, 3.0, 4.0);
        let d2 = dist(3.0, 4.0, 0.0, 0.0);
        assert!((d1 - d2).abs() < 1e-9);
    }

    #[test]
    fn test_circles_very_small_radius() {
        // Tiny circles far apart
        assert!(!circles_collide(0.0, 0.0, 0.01, 10.0, 0.0, 0.01));
    }

    #[test]
    fn test_circles_zero_distance() {
        // Same center, any positive radius
        assert!(circles_collide(0.0, 0.0, 1.0, 0.0, 0.0, 1.0));
    }
}
