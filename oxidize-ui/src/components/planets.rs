//! Container component that renders all 8 planets
//! 
//! Iterates over indices 0..8 and renders a <Planet> component
//! for each, passing the planet index and signals.

use leptos::*;

use super::planet::Planet;

#[component]
pub fn Planets(
    planet_angles: ReadSignal<Vec<f64>>,
    target_planet_idx: ReadSignal<Option<usize>>,
) -> impl IntoView {
    view! {
        // Mercury (0)
        <Planet planet_idx={0} planet_angles={planet_angles} target_planet_idx={target_planet_idx} />
        // Venus (1)
        <Planet planet_idx={1} planet_angles={planet_angles} target_planet_idx={target_planet_idx} />
        // Earth (2)
        <Planet planet_idx={2} planet_angles={planet_angles} target_planet_idx={target_planet_idx} />
        // Mars (3)
        <Planet planet_idx={3} planet_angles={planet_angles} target_planet_idx={target_planet_idx} />
        // Jupiter (4)
        <Planet planet_idx={4} planet_angles={planet_angles} target_planet_idx={target_planet_idx} />
        // Saturn (5)
        <Planet planet_idx={5} planet_angles={planet_angles} target_planet_idx={target_planet_idx} />
        // Uranus (6)
        <Planet planet_idx={6} planet_angles={planet_angles} target_planet_idx={target_planet_idx} />
        // Neptune (7)
        <Planet planet_idx={7} planet_angles={planet_angles} target_planet_idx={target_planet_idx} />
    }
}
