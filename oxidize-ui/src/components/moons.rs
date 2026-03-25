//! Container component that renders moons for all planets
//! 
//! Props: planet_angles, moon_angles
//! Iterates over all 8 planets and renders a <Moon> component
//! for each one that has moons.

use leptos::*;

use super::moon::Moon;

#[component]
pub fn Moons(
    planet_angles: ReadSignal<Vec<f64>>,
    moon_angles: ReadSignal<Vec<f64>>,
) -> impl IntoView {
    // Moons only exist for planets: Earth(2), Mars(3), Jupiter(4), Saturn(5), Uranus(6), Neptune(7)
    // Mercury(0) and Venus(1) have no moons
    view! {
        <Moon planet_idx={2} planet_angles={planet_angles} moon_angles={moon_angles} />
        <Moon planet_idx={3} planet_angles={planet_angles} moon_angles={moon_angles} />
        <Moon planet_idx={4} planet_angles={planet_angles} moon_angles={moon_angles} />
        <Moon planet_idx={5} planet_angles={planet_angles} moon_angles={moon_angles} />
        <Moon planet_idx={6} planet_angles={planet_angles} moon_angles={moon_angles} />
        <Moon planet_idx={7} planet_angles={planet_angles} moon_angles={moon_angles} />
    }
}
