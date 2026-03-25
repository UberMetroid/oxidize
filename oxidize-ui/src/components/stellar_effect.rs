//! Permanent Stellar Engine white-hot flare effect.

use leptos::*;

const FLARE_SVG: &str = "<svg width=\"120\" height=\"120\" style=\"position:absolute;left:50%;top:50%;transform:translate(-50%,-50%)\"><circle cx=\"60\" cy=\"60\" r=\"50\" fill=\"none\" stroke=\"#ff6b00\" stroke-width=\"2\" style=\"animation:shockwave 3.5s ease-out infinite;transform-origin:60px 60px;\"/><circle cx=\"60\" cy=\"60\" r=\"20\" fill=\"#fff\" style=\"animation:stellar-flare 3.5s ease-out forwards;transform-origin:60px 60px;\"/><circle cx=\"60\" cy=\"60\" r=\"35\" fill=\"none\" stroke=\"#ff8c00\" stroke-width=\"1\" style=\"animation:shockwave 3.5s ease-out infinite .5s;transform-origin:60px 60px;\"/></svg>";

#[component]
pub fn StellarEffect(ship_x: f64, ship_y: f64) -> impl IntoView {
    view! {
        <div class="absolute pointer-events-none"
            style=format!("left:{}%;top:{}%;transform:translate(-50%,-50%);opacity:1;", ship_x, ship_y)>
            <div inner_html={FLARE_SVG}></div>
        </div>
    }
}
