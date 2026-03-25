//! Uranus's tilted cyan rings rendering
//! 
//! Props: planet_angles signal to compute ring rotation.
//! Uranus is at index 6 in PLANET_DATA. Rings are tilted with
//! rotateX(75deg) and rotate based on Uranus's orbital position.

use leptos::*;

use crate::constants::*;

#[component]
pub fn UranusRings(
    planet_angles: ReadSignal<Vec<f64>>,
) -> impl IntoView {
    const URANUS_IDX: usize = 6; // Uranus is index 6

    view! {
        {move || {
            let porbits = planet_angles.get();
            let angle = PLANET_INITIAL_ANGLES[URANUS_IDX] + porbits[URANUS_IDX];
            let size = PLANET_DATA[URANUS_IDX].1;
            view! {
                // Outer ring
                <div style=format!("position: absolute; left: 50%%; top: 50%%; transform: translate(-50%%, -50%%) rotateX(75deg) rotate({}rad); width: {}px; height: {}px; border: 1px #67e8f9; border-radius: 50%%; opacity: 0.2; box-shadow: 0 0 4px #67e8f940; pointer-events: none;", angle, size * 2.8, size * 0.7)></div>
                // Inner ring
                <div style=format!("position: absolute; left: 50%%; top: 50%%; transform: translate(-50%%, -50%%) rotateX(75deg) rotate({}rad); width: {}px; height: {}px; border: 1px #67e8f9; border-radius: 50%%; opacity: 0.35; box-shadow: 0 0 6px #67e8f960; pointer-events: none;", angle, size * 2.3, size * 0.6)></div>
            }
        }}
    }
}
