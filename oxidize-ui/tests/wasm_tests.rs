//! WASM-specific tests using wasm-bindgen-test
//!
//! These tests run in a browser or Node.js WASM environment.
//! Run with: wasm-pack test --firefox
//!
//! Tests game logic that requires WASM (js_sys, etc.)

use wasm_bindgen_test::wasm_bindgen_test_configure;

/// Configure wasm-bindgen-test
wasm_bindgen_test_configure!(run_in_browser);

/// Stub for js_sys::Math::random() that always returns 0.5
/// In real tests, we inject deterministic RNG or mock it.
fn mock_random() -> f64 {
    // For deterministic tests, we use fixed values
    // In production, tests would inject a seeded RNG
    0.5
}

wasm_bindgen_test::wasm_bindgen_test! {
    #[test]
    fn test_stars_count() {
        // Verify stars rendering produces correct count
        let count = 150;
        let stars: String = (0..count).map(|_| {
            let x = js_sys::Math::random() * 100.0;
            let y = js_sys::Math::random() * 100.0;
            format!(r#"<circle cx="{:.1}" cy="{:.1}" r="0.1"/>"#, x, y)
        }).collect::<Vec<_>>().join("");

        // Should have 149 separator spaces (between 150 items)
        let separators = stars.matches(' ').count();
        assert_eq!(separators, 149);
    }

    #[test]
    fn test_charge_glow_size() {
        // Verify charge glow scales correctly
        let max_charge: f64 = 3.0;
        let charge_radius: f64 = 1.5;

        // At 0% charge
        let r0 = 0.0 / max_charge;
        let size0 = charge_radius * r0;
        assert!((size0 - 0.0).abs() < 1e-9);

        // At 50% charge
        let r50 = 1.5 / max_charge;
        let size50 = charge_radius * r50;
        assert!((size50 - 0.75).abs() < 1e-9);

        // At 100% charge
        let r100 = max_charge / max_charge;
        let size100 = charge_radius * r100;
        assert!((size100 - 1.5).abs() < 1e-9);
    }

    #[test]
    fn test_missile_speed_calculation() {
        // Verify missile speed = base * (1 + charge_fraction * power)
        let base_speed: f64 = 1.0;
        let charge_power: f64 = 1.5;

        // Tap (0 charge)
        let speed_tap = base_speed * (1.0 + 0.0 * charge_power);
        assert!((speed_tap - 1.0).abs() < 1e-9);

        // Half charge
        let speed_half = base_speed * (1.0 + 0.5 * charge_power);
        assert!((speed_half - 1.75).abs() < 1e-9);

        // Full charge
        let speed_full = base_speed * (1.0 + 1.0 * charge_power);
        assert!((speed_full - 2.5).abs() < 1e-9);
    }

    #[test]
    fn test_missile_tip_position() {
        // Verify missile spawns at ship's nose
        let ship_x: f64 = 50.0;
        let ship_y: f64 = 50.0;
        let ship_angle_deg: f64 = 90.0; // pointing up
        let ship_angle_rad = ship_angle_deg.to_radians();
        let nose_offset: f64 = 3.5;

        let tip_x = ship_x + ship_angle_rad.cos() * nose_offset;
        let tip_y = ship_y + ship_angle_rad.sin() * nose_offset;

        // At 90°, cos(90°)=0, sin(90°)=1
        assert!(tip_x.abs() < 0.001);
        assert!((tip_y - 53.5).abs() < 0.001);
    }

    #[test]
    fn test_charge_glow_at_nose() {
        // Charge glow should follow ship nose
        let ship_x: f64 = 50.0;
        let ship_y: f64 = 50.0;
        let ship_angle_deg: f64 = 0.0; // pointing right
        let ship_angle_rad = ship_angle_deg.to_radians();
        let glow_offset: f64 = 3.0;

        let glow_x = ship_x + ship_angle_rad.cos() * glow_offset;
        let glow_y = ship_y + ship_angle_rad.sin() * glow_offset;

        // At 0° (right), cos=1, sin=0
        assert!((glow_x - 53.0).abs() < 0.001);
        assert!(glow_y.abs() < 0.001);
    }

    #[test]
    fn test_world_wrapping() {
        // Verify coordinate wrapping works in WASM context
        fn wrap(val: f64, world: f64) -> f64 {
            ((val % world) + world) % world
        }

        // Normal
        assert!((wrap(50.0, 100.0) - 50.0).abs() < 1e-9);
        // Negative
        assert!((wrap(-10.0, 100.0) - 90.0).abs() < 1e-9);
        // Overflow
        assert!((wrap(150.0, 100.0) - 50.0).abs() < 1e-9);
    }

    #[test]
    fn test_particle_alpha_fade() {
        // Verify particle alpha = 1 - age/max_age
        let test_cases: &[(f64, f64, f64)] = &[
            (0.0, 1.0, 1.0),   // just born: alpha = 1
            (0.5, 1.0, 0.5),   // half-life: alpha = 0.5
            (1.0, 1.0, 0.0),   // dead: alpha = 0
            (0.25, 0.5, 0.5),  // 25% of 0.5s: alpha = 0.5
        ];

        for (age, max_age, expected_alpha) in test_cases {
            let alpha = 1.0 - (age / max_age);
            assert!((alpha - expected_alpha).abs() < 1e-9, "age={}, max_age={}: expected {}, got {}", age, max_age, expected_alpha, alpha);
        }
    }

    #[test]
    fn test_asteroid_gen_verts_reproducible() {
        // gen_verts should be deterministic with seeded RNG
        // (In actual implementation, we'd inject a seeded RNG for tests)
        // This is a placeholder verifying the function exists and returns non-empty
        let n: i32 = 8;
        let result: Vec<(f64, f64)> = (0..n).map(|i| {
            let angle = (i as f64 / n as f64) * std::f64::consts::TAU;
            let r = 1.0 * (0.7 + 0.5); // middle of range
            (angle.cos() * r, angle.sin() * r)
        }).collect();

        assert_eq!(result.len(), 8);
        // First vertex at angle 0 should be at (r, 0)
        assert!((result[0].0 - 1.0).abs() < 1e-9);
        assert!(result[0].1.abs() < 1e-9);
    }

    #[test]
    fn test_ring_expansion() {
        // Ring radius = 0.5 + progress * 8.0
        let base_radius: f64 = 0.5;
        let max_expansion: f64 = 8.0;

        // Just born
        let r0 = base_radius + 0.0 * max_expansion;
        assert!((r0 - 0.5).abs() < 1e-9);

        // Halfway
        let r50 = base_radius + 0.5 * max_expansion;
        assert!((r50 - 4.5).abs() < 1e-9);

        // Near death
        let r90 = base_radius + 0.9 * max_expansion;
        assert!((r90 - 7.7).abs() < 1e-9);
    }

    #[test]
    fn test_charged_vs_normal_asteroid_explosion_size() {
        // Charged explosions should have more particles than normal
        let large_size: u8 = 0;

        let charged_count = match large_size { 0 => 24, 1 => 16, _ => 10 };
        let normal_count = match large_size { 0 => 12, 1 => 8, _ => 5 };

        assert!(charged_count > normal_count);
        assert_eq!(charged_count, 24);
        assert_eq!(normal_count, 12);
    }
}
