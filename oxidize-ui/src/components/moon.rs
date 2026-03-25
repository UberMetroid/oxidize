//! Moon rendering for a specific planet. Gradient IDs use planet_idx to avoid collision.

use leptos::*;
use crate::constants::*;

fn flat_idx(planet_idx: usize, local: usize) -> usize {
    MOON_DATA[..planet_idx].iter().map(|m| m.len()).sum::<usize>() + local
}

#[component]
pub fn Moon(planet_idx: usize, planet_angles: ReadSignal<Vec<f64>>, moon_angles: ReadSignal<Vec<f64>>) -> impl IntoView {
    view! {
        <svg class="absolute inset-0 w-full h-full pointer-events-none" style="z-index: 18;">
            <defs>{move || {
                let moons = MOON_DATA[planet_idx];
                if moons.is_empty() { return view! { <g></g> }.into_view(); }
                let porbits = planet_angles.get();
                let pa = PLANET_INITIAL_ANGLES[planet_idx] + porbits[planet_idx];
                let pr = PLANET_DATA[planet_idx].0;
                let px = 50.0 + pr * pa.cos(); let py = 50.0 + pr * pa.sin();
                let mut defs = String::new();
                for (i, moon) in moons.iter().enumerate() {
                    let &(_, mr, ms, mc) = moon;
                    let gidx = flat_idx(planet_idx, i);
                    let ma = moon_angles.get()[gidx];
                    let mx = px + mr * ma.cos(); let my = py + mr * ma.sin();
                    let dx = mx - 50.0; let dy = my - 50.0;
                    let dist = (dx*dx + dy*dy).sqrt().max(0.1);
                    let nx = dx/dist; let ny = dy/dist;
                    let cx = (50.0 + nx*0.35).clamp(50.0-ms*0.15, 50.0+ms*0.15);
                    let cy = (50.0 + ny*0.35).clamp(50.0-ms*0.15, 50.0+ms*0.15);
                    defs.push_str(&format!(r##"<radialGradient id="moon-lit-{}-{}" cx="{}" cy="{}" r="50%"><stop offset="0%" stop-color="#fffaf0"/><stop offset="40%" stop-color="{}"/><stop offset="100%" stop-color="{}"/></radialGradient>"##, planet_idx, i, cx, cy, mc, mc));
                }
                view! { <g inner_html={defs}></g> }.into_view()
            }}</defs>
            {move || {
                let moons = MOON_DATA[planet_idx];
                if moons.is_empty() { return view! { <g></g> }.into_view(); }
                let porbits = planet_angles.get();
                let pa = PLANET_INITIAL_ANGLES[planet_idx] + porbits[planet_idx];
                let pr = PLANET_DATA[planet_idx].0;
                let px = 50.0 + pr * pa.cos(); let py = 50.0 + pr * pa.sin();
                let mut html = String::new();
                for (i, moon) in moons.iter().enumerate() {
                    let &(_, mr, ms, _) = moon;
                    let gidx = flat_idx(planet_idx, i);
                    let ma = moon_angles.get()[gidx];
                    html.push_str(&format!(r##"<circle cx="{}" cy="{}" r="{}" fill="url(#moon-lit-{}-{})"/>"##, px + mr*ma.cos(), py + mr*ma.sin(), ms, planet_idx, i));
                }
                view! { <g inner_html={html}></g> }.into_view()
            }}
        </svg>
    }
}
