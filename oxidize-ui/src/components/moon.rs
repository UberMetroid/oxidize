//! Moon rendering for a specific planet
//! 
//! Props: planet_idx, planet_angles, moon_angles
//! Uses MOON_DATA and MOON_PERIODS_FLAT to render all moons
//! for the given planet. Uses SVG approach with radial gradients
//! for sun-lit appearance.

use leptos::*;

use crate::constants::*;

#[component]
pub fn Moon(
    planet_idx: usize,
    planet_angles: ReadSignal<Vec<f64>>,
    moon_angles: ReadSignal<Vec<f64>>,
) -> impl IntoView {
    view! {
        <svg class="absolute inset-0 w-full h-full pointer-events-none" style="z-index: 18;">
            <defs>
                {move || {
                    let porbits = planet_angles.get();
                    let mangles = moon_angles.get();
                    let mut defs = String::new();
                    let mut moon_idx = 0;
                    
                    // Count moons before this planet
                    for (pidx, moons) in MOON_DATA.iter().enumerate() {
                        if pidx < planet_idx {
                            moon_idx += moons.len();
                        }
                    }
                    
                    let moons = MOON_DATA[planet_idx];
                    if moons.is_empty() { return view! { <g></g> }.into_view(); }
                    
                    let planet_angle = PLANET_INITIAL_ANGLES[planet_idx] + porbits[planet_idx];
                    let planet_orbit_r = PLANET_DATA[planet_idx].0;
                    let planet_x = 50.0 + planet_orbit_r * planet_angle.cos();
                    let planet_y = 50.0 + planet_orbit_r * planet_angle.sin();
                    
                    for moon in moons.iter() {
                        let (_, moon_orbit_r, moon_size, _moon_color) = *moon;
                        let angle = mangles[moon_idx];
                        let moon_x = planet_x + moon_orbit_r * angle.cos();
                        let moon_y = planet_y + moon_orbit_r * angle.sin();
                        let dx = moon_x - 50.0;
                        let dy = moon_y - 50.0;
                        let dist = (dx * dx + dy * dy).sqrt().max(0.1);
                        let nx = dx / dist;
                        let ny = dy / dist;
                        let hl_cx = 50.0 + nx * 0.35;
                        let hl_cy = 50.0 + ny * 0.35;
                        let hl_cx_c = hl_cx.max(50.0 - moon_size * 0.15).min(50.0 + moon_size * 0.15);
                        let hl_cy_c = hl_cy.max(50.0 - moon_size * 0.15).min(50.0 + moon_size * 0.15);
                        let moon_color = MOON_DATA[planet_idx][moon_idx].3;
                        defs.push_str(&format!("<radialGradient id=\"moon-lit-{}\" cx=\"{}\" cy=\"{}\" r=\"50%\"><stop offset=\"0%\" stop-color=\"#fffaf0\"/><stop offset=\"40%\" stop-color=\"{}\"/><stop offset=\"100%\" stop-color=\"{}\"/></radialGradient>",
                            moon_idx, hl_cx_c, hl_cy_c, moon_color, moon_color));
                        moon_idx += 1;
                    }
                    view! { <g inner_html={defs}></g> }.into_view()
                }}
            </defs>
            {move || {
                let porbits = planet_angles.get();
                let mangles = moon_angles.get();
                let mut moon_idx = 0;
                
                // Count moons before this planet
                for (pidx, moons) in MOON_DATA.iter().enumerate() {
                    if pidx < planet_idx {
                        moon_idx += moons.len();
                    }
                }
                
                let moons = MOON_DATA[planet_idx];
                if moons.is_empty() { return view! { <g></g> }.into_view(); }
                
                let planet_angle = PLANET_INITIAL_ANGLES[planet_idx] + porbits[planet_idx];
                let planet_orbit_r = PLANET_DATA[planet_idx].0;
                let planet_x = 50.0 + planet_orbit_r * planet_angle.cos();
                let planet_y = 50.0 + planet_orbit_r * planet_angle.sin();
                
                let mut paths = String::new();
                for moon in moons.iter() {
                    let (_, moon_orbit_r, moon_size, _) = *moon;
                    let angle = mangles[moon_idx];
                    let moon_x = planet_x + moon_orbit_r * angle.cos();
                    let moon_y = planet_y + moon_orbit_r * angle.sin();
                    paths.push_str(&format!("<circle cx=\"{}\" cy=\"{}\" r=\"{}\" fill=\"url(#moon-lit-{})\"/>", moon_x, moon_y, moon_size, moon_idx));
                    moon_idx += 1;
                }
                view! { <g inner_html={paths}></g> }.into_view()
            }}
        </svg>
    }
}
