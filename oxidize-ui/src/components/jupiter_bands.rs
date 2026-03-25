//! Jupiter bands and Great Red Spot overlay.
//!
//! Renders horizontal atmospheric bands and the Great Red Spot
//! for Jupiter. Self-contained — used inside Jupiter's wrapper in solar_system.rs.

use leptos::*;

#[component]
pub fn JupiterBands(size: i32) -> impl IntoView {
    let spot_w = size / 5;
    let spot_h = (size / 7) as i32;

    view! {
        <div class="jupiter-bands"></div>
        <div style=format!(
            "position: absolute; left: 55%%; top: 60%%; width: {}px; height: {}px; \
             background: radial-gradient(ellipse, #ef4444, #b91c1c, transparent); \
             border-radius: 50%%; opacity: 0.7;",
            spot_w, spot_h
        )></div>
    }
}
