//! Ship trail particles rendering
//! 
//! Props: trail_positions signal containing Vec<(f64, f64, f64)>
//! representing (x, y, age). Renders fading orange trail particles
//! with size and opacity based on age.

use leptos::*;

#[component]
pub fn ShipTrail(
    trail_positions: ReadSignal<Vec<(f64, f64, f64)>>,
) -> impl IntoView {
    view! {
        <svg class="absolute inset-0 w-full h-full pointer-events-none" style="z-index: 19;">
            {move || {
                let trail = trail_positions.get();
                let mut circles = String::new();
                for &(x, y, age) in trail.iter() {
                    let opacity = (1.0 - age).max(0.0);
                    let size = 3.0 * (1.0 - age * 0.5);
                    if opacity > 0.0 && size > 0.0 {
                        circles.push_str(&format!("<circle cx=\"{}\" cy=\"{}\" r=\"{}\" fill=\"#f97316\" opacity=\"{}\"/>", x, y, size, opacity));
                    }
                }
                view! { <g inner_html={circles}></g> }
            }}
        </svg>
    }
}
