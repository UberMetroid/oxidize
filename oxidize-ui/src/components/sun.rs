//! Sun rendering with corona and pulsing glow
//! 
//! Self-contained component. Uses hardcoded values for sun position
//! and appearance. Renders at center (50%, 50%) with radial gradient
//! and animated corona layers.

use leptos::*;

#[component]
pub fn Sun() -> impl IntoView {
    view! {
        <div class="absolute pointer-events-none" style="left: 50%%; top: 50%%; transform: translate(-50%%, -50%%); z-index: 5;">
            // Outer corona glow layer
            <div style="position: absolute; left: 50%%; top: 50%%; transform: translate(-50%%, -50%%); width: 100px; height: 100px; border-radius: 50%%; background: radial-gradient(circle, transparent 35%%, rgba(255,200,50,0.04) 50%%, transparent 65%%); animation: corona-pulse 4s ease-in-out infinite;"></div>
            // Middle corona glow
            <div style="position: absolute; left: 50%%; top: 50%%; transform: translate(-50%%, -50%%); width: 50px; height: 50px; border-radius: 50%%; background: radial-gradient(circle, #ffc83280 0%%, #ff960040 40%%, transparent 70%%); animation: corona-pulse 3s ease-in-out infinite;"></div>
            // Core sun with gradient
            <div style="position: absolute; left: 50%%; top: 50%%; transform: translate(-50%%, -50%%); width: 24px; height: 24px; border-radius: 50%%; background: radial-gradient(circle, #fff 0%%, #ffd700 35%%, #ff8c00 70%%, #ff6600 100%%); animation: sun-pulse 2.5s ease-in-out infinite; box-shadow: 0 0 15px #ff8c00, 0 0 30px #ffa500, 0 0 50px #ffa50080;"></div>
        </div>
    }
}
