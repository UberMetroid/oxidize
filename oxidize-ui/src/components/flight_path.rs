//! Flight path trail and gravity-bent prediction rendering.
//!
//! Shows: orange dotted trail of actual path taken, and a brighter
//! dashed prediction line showing where the ship is heading.

use leptos::*;

#[component]
pub fn FlightPath(
    /// Actual path the ship has flown (world x%, y%)
    flight_path: ReadSignal<Vec<(f64, f64)>>,
    /// Pre-computed curved prediction path
    flight_prediction: ReadSignal<Vec<(f64, f64)>>,
    /// Whether the ship is currently in flight
    is_flying: ReadSignal<bool>,
    /// Unused — snap animation removed
    #[allow(unused)]
    is_arriving: ReadSignal<bool>,
    /// Unused
    #[allow(unused)]
    fly_x: ReadSignal<f64>,
    /// Unused
    #[allow(unused)]
    fly_y: ReadSignal<f64>,
) -> impl IntoView {
    // ─── Actual path trail (dotted orange line) ───────────────────────────
    view! {
        <svg class="absolute inset-0 w-full h-full pointer-events-none" style="z-index: 20;">
            {move || {
                let path = flight_path.get();
                let flying = is_flying.get();
                if path.is_empty() { return view! { <g></g> }.into_view(); }
                let n = path.len();

                // Build SVG path string from recorded positions
                let d: String = path.iter().enumerate()
                    .map(|(i, &(x, y))| {
                        if i == 0 { format!("M{} {}", x, y) }
                        else { format!(" L{} {}", x, y) }
                    })
                    .collect();

                // Dots — fade from dim (old) to bright (current position)
                let dots: String = path.iter().enumerate()
                    .map(|(i, &(x, y))| {
                        let prog = i as f64 / n as f64;
                        let op = 0.3 + prog * 0.7;
                        let sz = 1.0 + prog * 2.0;
                        format!("<circle cx=\"{}\" cy=\"{}\" r=\"{}\" fill=\"#f97316\" opacity=\"{}\"/>", x, y, sz, op)
                    })
                    .collect();

                // Filled line — thick glow trail
                let line = if n > 1 {
                    format!("<path d=\"{}\" fill=\"none\" stroke=\"#f97316\" stroke-width=\"3\" opacity=\"0.5\"/>", d)
                } else { String::new() };

                view! { <g inner_html={format!("{}{}", line, dots)}></g> }.into_view()
            }}
        </svg>

        // ─── Prediction line (brighter, dashed — shows where you're going) ─
        <svg class="absolute inset-0 w-full h-full pointer-events-none" style="z-index: 21;">
            {move || {
                let pred = flight_prediction.get();
                if pred.is_empty() { return view! { <g></g> }.into_view(); }
                let n = pred.len();
                let d: String = pred.iter().enumerate()
                    .map(|(i, &(x, y))| {
                        if i == 0 { format!("M{} {}", x, y) }
                        else { format!(" L{} {}", x, y) }
                    })
                    .collect();

                // Outer glow line
                let glow = if n > 1 {
                    format!("<path d=\"{}\" fill=\"none\" stroke=\"#f97316\" stroke-width=\"5\" opacity=\"0.25\" style=\"filter:blur(3px)\"/>", d)
                } else { String::new() };
                // Main dashed prediction line
                let main = if n > 1 {
                    format!("<path d=\"{}\" fill=\"none\" stroke=\"#f97316\" stroke-width=\"2\" stroke-dasharray=\"6,4\" opacity=\"0.75\" style=\"filter:drop-shadow(0 0 4px #f97316)\"/>", d)
                } else { String::new() };
                // Arrow head at end
                let tip: String = if n >= 2 {
                    let end: &(f64, f64) = pred.last().unwrap();
                    let prev: &(f64, f64) = &pred[n - 2];
                    let angle = (end.1 - prev.1).atan2(end.0 - prev.0);
                    format!("<polygon points=\"0,-4 8,0 0,4\" fill=\"#f97316\" opacity=\"0.9\" transform=\"translate({},{}) rotate({}deg)\" style=\"filter:drop-shadow(0 0 4px #f97316)\"/>", end.0, end.1, angle.to_degrees())
                } else { String::new() };

                view! { <g inner_html={format!("{}{}{}", glow, main, tip)}></g> }.into_view()
            }}
        </svg>
    }
}
