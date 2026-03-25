//! Saturn's 3-layer rings rendering
//! 
//! Props: planet_angles signal to compute ring rotation.
//! Saturn is at index 5 in PLANET_DATA. Rings are tilted at -20deg
//! and rotate based on Saturn's orbital position.

use leptos::*;

use crate::constants::*;

#[component]
pub fn SaturnRings(
    planet_angles: ReadSignal<Vec<f64>>,
) -> impl IntoView {
    const SATURN_IDX: usize = 5; // Saturn is index 5

    view! {
        {move || {
            let porbits = planet_angles.get();
            let angle = PLANET_INITIAL_ANGLES[SATURN_IDX] + porbits[SATURN_IDX];
            let size = PLANET_DATA[SATURN_IDX].1;
            view! {
                // Outer ring
                <div style=format!("position: absolute; left: 50%%; top: 50%%; transform: translate(-50%%, -50%%) rotate({}rad); width: {}px; height: {}px; border: 2px #d4a574; border-radius: 50%%; opacity: 0.3; box-shadow: 0 0 3px #eab3084c; pointer-events: none;", angle, size * 3.4, size * 0.85)></div>
                // Middle ring
                <div style=format!("position: absolute; left: 50%%; top: 50%%; transform: translate(-50%%, -50%%) rotate({}rad); width: {}px; height: {}px; border: 2px #c0a060; border-radius: 50%%; opacity: 0.5; box-shadow: 0 0 5px #eab30866; pointer-events: none;", angle, size * 2.8, size * 0.75)></div>
                // Inner ring
                <div style=format!("position: absolute; left: 50%%; top: 50%%; transform: translate(-50%%, -50%%) rotate({}rad); width: {}px; height: {}px; border: 2px #eab308; border-radius: 50%%; opacity: 0.7; box-shadow: 0 0 8px #eab3087f; pointer-events: none;", angle, size * 2.3, size * 0.65)></div>
            }
        }}
    }
}
