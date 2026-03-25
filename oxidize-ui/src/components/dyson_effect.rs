//! Permanent Dyson Collector golden ring effect.

use leptos::*;

const RING_SVG: &str = "<svg width=\"80\" height=\"80\" style=\"position:absolute;left:50%;top:50%;transform:translate(-50%,-50%)\"><circle cx=\"40\" cy=\"40\" r=\"35\" fill=\"none\" stroke=\"#fbbf24\" stroke-width=\"3\" stroke-dasharray=\"4,2\" style=\"animation:dyson-assemble 3s ease-out forwards;filter:drop-shadow(0 0 8px #fbbf24);\"/><circle cx=\"40\" cy=\"40\" r=\"25\" fill=\"none\" stroke=\"#f59e0b\" stroke-width=\"2\" style=\"animation:dyson-assemble 3s ease-out forwards;filter:drop-shadow(0 0 4px #f59e0b);\"/></svg>";

#[component]
pub fn DysonEffect(ship_x: f64, ship_y: f64) -> impl IntoView {
    view! {
        <div class="absolute pointer-events-none"
            style=format!("left:{}%;top:{}%;transform:translate(-50%,-50%);opacity:1;", ship_x, ship_y)>
            <div inner_html={RING_SVG}></div>
        </div>
    }
}
