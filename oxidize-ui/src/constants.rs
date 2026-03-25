//! Static constants for the solar system visualization

/// Planet data: (orbit_radius, size, color)
/// Sizes balanced for visual clarity while maintaining relative proportions
pub static PLANET_DATA: &[(f64, f64, &str)] = &[
    (8.0, 5.0, "#9ca3af"),   // Mercury
    (14.0, 8.0, "#fbbf24"),  // Venus
    (20.0, 9.0, "#3b82f6"),   // Earth
    (27.0, 6.0, "#ef4444"),   // Mars
    (38.0, 18.0, "#f97316"),  // Jupiter - biggest
    (50.0, 15.0, "#eab308"),  // Saturn
    (62.0, 10.0, "#06b6d4"),  // Uranus
    (75.0, 9.0, "#6366f1"),  // Neptune
];

pub static PLANET_PERIODS: [f64; 8] = [4.0, 7.0, 10.0, 15.0, 30.0, 45.0, 60.0, 80.0];
pub static PLANET_INITIAL_ANGLES: [f64; 8] = [0.0, 1.2, 2.4, 3.8, 5.1, 0.7, 2.1, 4.5];
pub static SHIP_ORBIT_RADIUS: f64 = 18.0;
pub static SHIP_PLANET_ORBIT_RADIUS: f64 = 4.0;

/// Moon data: (moon_orbit_radius, moon_size, moon_color)
#[allow(dead_code)]
pub static MOON_DATA: &[&[(&str, f64, f64, &str)]] = &[
    &[], // Mercury - no moons
    &[], // Venus - no moons
    &[("Luna", 1.5, 8.0, "#c0c0c0")], // Earth - Luna
    &[("Phobos", 1.0, 5.0, "#8b7355"), ("Deimos", 0.8, 8.0, "#a0522d")], // Mars
    &[("Io", 1.2, 6.0, "#ffff00"), ("Europa", 1.0, 10.0, "#f5deb3"), ("Ganymede", 1.5, 14.0, "#808080"), ("Callisto", 1.2, 18.0, "#4a4a4a")], // Jupiter - Galilean moons
    &[("Titan", 2.0, 10.0, "#d4a574"), ("Enceladus", 0.8, 14.0, "#f0f0f0"), ("Mimas", 0.7, 16.0, "#c0c0c0"), ("Rhea", 1.0, 20.0, "#d3d3d3")], // Saturn
    &[("Titania", 1.2, 15.0, "#808080"), ("Oberon", 1.0, 20.0, "#696969")], // Uranus
    &[("Triton", 1.5, 12.0, "#87ceeb")], // Neptune - Triton
];
#[allow(dead_code)]
pub static MOON_PERIODS: &[&[f64]] = &[
    &[],
    &[],
    &[8.0], // Luna
    &[2.0, 3.5], // Phobos, Deimos
    &[3.0, 5.0, 12.0, 25.0], // Io, Europa, Ganymede, Callisto
    &[4.0, 5.5, 8.0, 12.0], // Titan, Enceladus, Mimas, Rhea
    &[12.0, 18.0], // Titania, Oberon
    &[10.0], // Triton
];

/// Flattened moon periods for all 14 moons in order
pub static MOON_PERIODS_FLAT: [f64; 14] = [
    8.0,   // Luna (Earth)
    2.0,   // Phobos (Mars)
    3.5,   // Deimos (Mars)
    3.0,   // Io (Jupiter)
    5.0,   // Europa (Jupiter)
    12.0,  // Ganymede (Jupiter)
    25.0,  // Callisto (Jupiter)
    4.0,   // Titan (Saturn)
    5.5,   // Enceladus (Saturn)
    8.0,   // Mimas (Saturn)
    12.0,  // Rhea (Saturn)
    12.0,  // Titania (Uranus)
    18.0,  // Oberon (Uranus)
    10.0,  // Triton (Neptune)
];
