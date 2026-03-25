//! Flight path and prediction rendering
//! 
//! Props: flight_path, flight_prediction, is_flying, is_arriving, planet_angles
//! Renders:
//! - Actual flight path (orange dots + dashed line)
//! - Prediction line (semi-transparent dashed)
//! - Gravity wells during flight (concentric circles per planet)
//! - Snap ring animation when arriving

use leptos::*;

use crate::constants::*;

#[component]
pub fn FlightPath(
    flight_path: ReadSignal<Vec<(f64, f64)>>,
    flight_prediction: ReadSignal<Vec<(f64, f64)>>,
    is_flying: ReadSignal<bool>,
    is_arriving: ReadSignal<bool>,
    planet_angles: ReadSignal<Vec<f64>>,
    target_planet_idx: ReadSignal<Option<usize>>,
    fly_x: ReadSignal<f64>,
    fly_y: ReadSignal<f64>,
) -> impl IntoView {
    view! {
        // Actual flight path (orange dots + dashed line)
        <svg class="absolute inset-0 w-full h-full pointer-events-none" style="z-index: 20;">
            {move || {
                let path = flight_path.get();
                let flying = is_flying.get();
                if path.is_empty() && !flying { return view! { <g></g> }; }
                
                let mut path_d = String::new();
                let mut dots = String::new();
                
                for (i, &(x, y)) in path.iter().enumerate() {
                    let progress = i as f64 / path.len() as f64;
                    let opacity = if flying { 0.6 + progress * 0.4 } else { progress * 0.8 };
                    let size = 1.5 + progress * 1.5;
                    
                    if i == 0 {
                        path_d = format!("M{} {}", x, y);
                    } else {
                        path_d.push_str(&format!(" L{} {}", x, y));
                    }
                    
                    dots.push_str(&format!(
                        "<circle cx=\"{}\" cy=\"{}\" r=\"{}\" fill=\"#f97316\" opacity=\"{}\"/>",
                        x, y, size, opacity
                    ));
                }
                
                let path_line = if path.len() > 1 {
                    format!("<path d=\"{}\" fill=\"none\" stroke=\"#f97316\" stroke-width=\"1.5\" stroke-dasharray=\"3,2\" opacity=\"0.7\"/>", path_d)
                } else { String::new() };
                
                view! { <g inner_html={format!("{}{}", path_line, dots)}></g> }
            }}
        </svg>

        // Flight prediction (planned curved path)
        <svg class="absolute inset-0 w-full h-full pointer-events-none" style="z-index: 21;">
            {move || {
                let pred = flight_prediction.get();
                if pred.is_empty() { return view! { <g></g> }; }
                
                let mut path_d = String::new();
                for (i, &(x, y)) in pred.iter().enumerate() {
                    if i == 0 {
                        path_d = format!("M{} {}", x, y);
                    } else {
                        path_d.push_str(&format!(" L{} {}", x, y));
                    }
                }
                
                view! {
                    <g>
                        <path d={path_d} fill="none" stroke="#f97316" stroke-width="1" stroke-dasharray="4,4" opacity="0.4" style="filter: drop-shadow(0 0 3px #f9731666);"/>
                    </g>
                }
            }}
        </svg>

        // Gravity wells visualization during flight
        <svg class="absolute inset-0 w-full h-full pointer-events-none" style="z-index: 6;">
            {move || {
                let flying = is_flying.get();
                let target_idx = target_planet_idx.get().unwrap_or(0);
                if !flying { return view! { <g></g> }; }
                
                let porbits = planet_angles.get();
                let mut wells = String::new();
                for (pidx, pdata) in PLANET_DATA.iter().enumerate() {
                    if pidx == target_idx { continue; }
                    
                    let pangle = PLANET_INITIAL_ANGLES[pidx] + porbits[pidx];
                    let px = 50.0 + pdata.0 * pangle.cos();
                    let py = 50.0 + pdata.0 * pangle.sin();
                    let mass = pdata.1;
                    
                    for ring in 1..=3 {
                        let radius = mass * 0.3 * ring as f64;
                        let opacity = 0.15 - ring as f64 * 0.04;
                        wells.push_str(&format!(
                            "<circle cx=\"{}\" cy=\"{}\" r=\"{}\" fill=\"none\" stroke=\"#f97316\" stroke-width=\"0.5\" opacity=\"{}\" stroke-dasharray=\"2,2\"/>",
                            px, py, radius, opacity
                        ));
                    }
                }
                
                view! { <g inner_html={wells}></g> }
            }}
        </svg>

        // Snap into orbit animation
        <svg class="absolute inset-0 w-full h-full pointer-events-none" style="z-index: 25;">
            {move || {
                let arriving = is_arriving.get();
                if !arriving { return view! { <g></g> }; }
                
                let x = fly_x.get();
                let y = fly_y.get();
                
                view! {
                    <g>
                        <circle cx={x} cy={y} r="8" fill="none" stroke="#f97316" stroke-width="2" class="snap-ring" style="filter: drop-shadow(0 0 8px #f97316);"/>
                        <circle cx={x} cy={y} r="4" fill="#fef08a" opacity="0.8" style="filter: drop-shadow(0 0 6px #fef08a);"/>
                    </g>
                }
            }}
        </svg>
    }
}
