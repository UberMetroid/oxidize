//! Reusable planet component
//! 
//! Renders a single planet with dynamic sun lighting, selection glow,
//! and Jupiter bands if applicable. Uses PLANET_DATA constants for
//! orbit radius, size, and color.

use leptos::*;

use crate::constants::*;

#[component]
pub fn Planet(
    planet_idx: usize,
    planet_angles: ReadSignal<Vec<f64>>,
    target_planet_idx: ReadSignal<Option<usize>>,
) -> impl IntoView {
    // Planet-specific gradient colors
    let gradients = [
        "#d1d5db,#6b7280 60%%,#374151",     // Mercury: gray
        "#fef3c7,#fbbf24 50%%,#d97706",     // Venus: yellow/orange
        "#93c5fd,#3b82f6 50%%,#1e3a8a",     // Earth: blue
        "#fca5a5,#ef4444 50%%,#7f1d1d",     // Mars: red
        "#fcd34d,#f97316 50%%,#c2410c",     // Jupiter: orange
        "#fef9c3,#eab308 50%%,#a16207",     // Saturn: gold
        "#a5f3fc,#06b6d4 50%%,#0e7490",     // Uranus: cyan
        "#a5b4fc,#6366f1 50%%,#4338ca",     // Neptune: indigo
    ];

    let glow_colors = [
        "#9ca3af", // Mercury
        "#fbbf24", // Venus
        "#3b82f6", // Earth
        "#ef4444", // Mars
        "#f97316", // Jupiter
        "#eab308", // Saturn
        "#06b6d4", // Uranus
        "#6366f1", // Neptune
    ];

    view! {
        <div class="absolute pointer-events-none" style=move || {
            let porbits = planet_angles.get();
            let angle = PLANET_INITIAL_ANGLES[planet_idx] + porbits[planet_idx];
            let r = PLANET_DATA[planet_idx].0;
            let x = 50.0 + r * angle.cos();
            let y = 50.0 + r * angle.sin();
            format!("left: {}%%; top: {}%%; transform: translate(-50%%, -50%%); z-index: {};", x, y, 10 + planet_idx)
        }>
            <div style=move || {
                let porbits = planet_angles.get();
                let angle = PLANET_INITIAL_ANGLES[planet_idx] + porbits[planet_idx];
                let lx = 50.0 + 25.0 * angle.cos();
                let ly = 50.0 - 25.0 * angle.sin();
                let lxc = lx.max(15.0).min(65.0);
                let lyc = ly.max(15.0).min(65.0);
                let is_selected = target_planet_idx.get() == Some(planet_idx);
                let size = PLANET_DATA[planet_idx].1;
                let glow = if is_selected { 
                    format!("position: absolute; left: 50%%; top: 50%%; transform: translate(-50%%, -50%%); width: {}px; height: {}px; border-radius: 50%%; color: {}; animation: glow-pulse 1.5s ease-in-out infinite; z-index: -1;", size + 6.0, size + 6.0, glow_colors[planet_idx]) 
                } else { 
                    String::new() 
                };
                format!("{}{}", glow, format!("width: {}px; height: {}px; background: radial-gradient(circle at {}%% {}%, {}); border-radius: 50%%; box-shadow: 0 0 8px #3b82f6e5, 0 0 16px rgba(59,130,246,0.3), inset -1px -1px 3px #0000004c;", size, size, lxc, lyc, gradients[planet_idx]))
            }></div>
            // Earth cloud layer
            {move || {
                if planet_idx != 2 { return view! { <></> }.into_view(); }
                view! {
                    <div style="position: absolute; left: 0%%; top: 0%%; width: 100%%; height: 100%%; border-radius: 50%%; background: repeating-linear-gradient(45deg, transparent 0%%, transparent 20%%, rgba(255,255,255,0.15) 20%%, rgba(255,255,255,0.15) 25%%, transparent 25%%, transparent 50%%, rgba(255,255,255,0.1) 50%%, rgba(255,255,255,0.1) 55%%); animation: earth-clouds 20s linear infinite; pointer-events: none; overflow: hidden;"></div>
                }.into_view()
            }}
            // Jupiter bands and red spot
            {move || {
                if planet_idx != 4 { return view! { <></> }.into_view(); }
                let size = PLANET_DATA[planet_idx].1;
                view! {
                    <div class="jupiter-bands"></div>
                    <div style=format!("position: absolute; left: 55%%; top: 60%%; width: {}px; height: {}px; background: radial-gradient(ellipse, #ef4444, #b91c1c, transparent); border-radius: 50%%; opacity: 0.7;", size * 0.22, size * 0.14)></div>
                }.into_view()
            }}
        </div>
    }
}
