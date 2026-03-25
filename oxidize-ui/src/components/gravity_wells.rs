//! Gravity wells — faint influence rings around each planet during flight.
//!
//! Shows 3 concentric dashed rings per planet (except target) to visualize
//! gravitational influence. Rings are sized relative to planet mass.

use leptos::*;
use crate::constants::*;

/// Renders gravity well influence rings around non-target planets during flight.
#[component]
pub fn GravityWells(
    planet_angles: ReadSignal<Vec<f64>>,
    target_planet_idx: ReadSignal<Option<usize>>,
    is_flying: ReadSignal<bool>,
) -> impl IntoView {
    view! {
        <svg class="absolute inset-0 w-full h-full pointer-events-none" style="z-index: 6;">
            {move || {
                let flying = is_flying.get();
                let target_idx = target_planet_idx.get().unwrap_or(0);
                if !flying { return view! { <g></g> }.into_view(); }

                let porbits = planet_angles.get();
                let mut wells = String::new();
                for (pidx, pdata) in PLANET_DATA.iter().enumerate() {
                    if pidx == target_idx { continue; }

                    let pangle = PLANET_INITIAL_ANGLES[pidx] + porbits[pidx];
                    let px = 50.0 + pdata.0 * pangle.cos();
                    let py = 50.0 + pdata.0 * pangle.sin();
                    // Ring sizes proportional to planet visual size — more visible
                    let sz = pdata.1 * 2.0;

                    for ring in 1..=3 {
                        let r = sz * ring as f64;
                        let op = 0.18 - ring as f64 * 0.04;
                        wells.push_str(&format!(
                            r##"<circle cx="{}" cy="{}" r="{}" fill="none" stroke="#f97316" stroke-width="1" opacity="{}" stroke-dasharray="3,3"/>"##,
                            px, py, r, op
                        ));
                    }
                }

                view! { <g inner_html={wells}></g> }.into_view()
            }}
        </svg>
    }
}
