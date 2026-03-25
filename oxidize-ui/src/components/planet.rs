//! Reusable planet sphere with dynamic sun lighting and selection glow.

use leptos::*;
use crate::constants::*;

/// (gradient, glow_color) pairs per planet index.
const PLANET_STYLES: [(&str, &str); 8] = [
    ("#d1d5db,#6b7280 60%,#374151", "#9ca3af"),
    ("#fef3c7,#fbbf24 50%,#d97706", "#fbbf24"),
    ("#93c5fd,#3b82f6 50%,#1e3a8a", "#3b82f6"),
    ("#fca5a5,#ef4444 50%,#7f1d1d", "#ef4444"),
    ("#fcd34d,#f97316 50%,#c2410c", "#f97316"),
    ("#fef9c3,#eab308 50%,#a16207", "#eab308"),
    ("#a5f3fc,#06b6d4 50%,#0e7490", "#06b6d4"),
    ("#a5b4fc,#6366f1 50%,#4338ca", "#6366f1"),
];

#[component]
pub fn Planet(
    planet_idx: usize,
    planet_angles: ReadSignal<Vec<f64>>,
    target_planet_idx: ReadSignal<Option<usize>>,
) -> impl IntoView {
    view! {
        <div class="absolute pointer-events-none" style=move || {
            let porbits = planet_angles.get();
            let angle = PLANET_INITIAL_ANGLES[planet_idx] + porbits[planet_idx];
            let r = PLANET_DATA[planet_idx].0;
            format!("left: {}%; top: {}%; transform: translate(-50%, -50%); z-index: {};",
                50.0 + r * angle.cos(), 50.0 + r * angle.sin(), 10 + planet_idx)
        }>
            <div style=move || {
                let porbits = planet_angles.get();
                let angle = PLANET_INITIAL_ANGLES[planet_idx] + porbits[planet_idx];
                let lxc = (50.0 + 25.0 * angle.cos()).clamp(15.0, 65.0);
                let lyc = (50.0 - 25.0 * angle.sin()).clamp(15.0, 65.0);
                let size = PLANET_DATA[planet_idx].1;
                let (grad, glow) = PLANET_STYLES[planet_idx];
                let is_sel = target_planet_idx.get() == Some(planet_idx);
                let glow_html = if is_sel {
                    format!("position:absolute;left:50%;top:50%;transform:translate(-50%,-50%);\
                        width:{}px;height:{}px;border-radius:50%;color:{};\
                        animation:glow-pulse 1.5s ease-in-out infinite;z-index:-1;",
                        size + 6.0, size + 6.0, glow)
                } else { String::new() };
                format!("{}{}", glow_html,
                    format!("width:{}px;height:{}px;background:radial-gradient(circle at {}% {}%,{});\
                        border-radius:50%;box-shadow:0 0 8px {},0 0 16px {},\
                        inset -1px -1px 3px #0000004c;", size, size, lxc, lyc, grad, glow, glow))
            }></div>
        </div>
    }
}
