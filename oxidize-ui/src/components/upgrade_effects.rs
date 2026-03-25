//! Transient upgrade deployment animations.

use leptos::*;
use crate::types::UpgradeEffect;
use oxidize_engine::UpgradeType;

fn fade_opacity(p: f64, permanent: bool) -> f64 {
    if p < 0.1 { p * 10.0 }
    else if p > 0.7 && !permanent { (1.0 - p) / 0.3 }
    else { 1.0 }
}

// SVG strings as module constants to avoid raw-string lexer issues
const SAIL_SVG: &str = "<svg width=\"80\" height=\"80\" style=\"position:absolute;left:50%;top:50%;transform:translate(-50%,-50%)\"><defs><linearGradient id=\"sg\" x1=\"0%\" y1=\"0%\" x2=\"100%\" y2=\"0%\"><stop offset=\"0%\" stop-color=\"#a5f3fc\" stop-opacity=\"0.8\"/><stop offset=\"100%\" stop-color=\"#67e8f9\" stop-opacity=\"0.3\"/></linearGradient></defs><path d=\"M40,15Q55,30,40,45Q25,30,40,15\" fill=\"url(#sg)\" style=\"animation:sail-unfurl 2s ease-out forwards\"/><path d=\"M25,20Q40,35,25,50Q10,35,25,20\" fill=\"url(#sg)\" style=\"animation:sail-unfurl 2s ease-out forwards .1s\"/><path d=\"M55,20Q70,35,55,50Q40,35,55,20\" fill=\"url(#sg)\" style=\"animation:sail-unfurl 2s ease-out forwards .2s\"/></svg>";
const PLASMA_SVG: &str = "<svg width=\"120\" height=\"60\" style=\"position:absolute;left:50%;top:50%;transform:translate(-50%,-50%)\"><polyline points=\"0,30 20,10 40,30 60,10 80,30 100,10 120,30\" fill=\"none\" stroke=\"#00d4ff\" stroke-width=\"3\" style=\"filter:drop-shadow(0 0 4px #00d4ff)drop-shadow(0 0 8px #00d4ff);animation:plasma-zap 1.5s ease-in-out infinite\"/><circle cx=\"15\" cy=\"20\" r=\"2\" fill=\"#00ffff\" style=\"animation:sparkle .5s ease-in-out infinite\"/><circle cx=\"55\" cy=\"20\" r=\"2\" fill=\"#00ffff\" style=\"animation:sparkle .5s ease-in-out infinite .2s\"/><circle cx=\"95\" cy=\"20\" r=\"2\" fill=\"#00ffff\" style=\"animation:sparkle .5s ease-in-out infinite .4s\"/></svg>";
const MIRROR_SVG: &str = "<svg width=\"60\" height=\"60\" style=\"position:absolute;left:50%;top:50%;transform:translate(-50%,-50%)\"><polygon points=\"30,5 52,17.5 52,42.5 30,55 8,42.5 8,17.5\" fill=\"none\" stroke=\"#fef08a\" stroke-width=\"2\" style=\"filter:drop-shadow(0 0 6px #fef08a);animation:mirror-spin 2s ease-out forwards\"/><circle cx=\"30\" cy=\"30\" r=\"4\" fill=\"#fef08a\" style=\"animation:sparkle .3s ease-in-out infinite\"/></svg>";
const QUANTUM_SVG: &str = "<svg width=\"70\" height=\"70\" style=\"position:absolute;left:50%;top:50%;transform:translate(-50%,-50%)\"><polygon points=\"35,5 45,20 65,25 50,40 55,60 35,50 15,60 20,40 5,25 25,20\" fill=\"none\" stroke=\"#c084fc\" stroke-width=\"2\" style=\"filter:drop-shadow(0 0 8px #c084fc);animation:quantum-glitch 2s ease-in-out forwards\"/><polygon points=\"35,15 42,25 52,27 44,35 46,45 35,40 24,45 26,35 18,27 28,25\" fill=\"#c084fc\" style=\"animation:quantum-glitch 2s ease-in-out infinite\"/></svg>";

#[component]
pub fn UpgradeEffects(
    upgrade_effects: ReadSignal<Vec<UpgradeEffect>>,
) -> impl IntoView {
    view! {
        {move || upgrade_effects.get().iter().filter_map(|e| {
            if matches!(e.upgrade_type, UpgradeType::DysonCollector | UpgradeType::StellarEngine) {
                return None;
            }
            let op = fade_opacity(e.progress, e.permanent);
            let svg = match e.upgrade_type {
                UpgradeType::SolarSail => SAIL_SVG,
                UpgradeType::PlasmaTether => PLASMA_SVG,
                UpgradeType::OrbitalMirror => MIRROR_SVG,
                UpgradeType::QuantumArray => QUANTUM_SVG,
                _ => "",
            };
            if svg.is_empty() { return None; }
            Some(view! {
                <div class="absolute pointer-events-none"
                    style=format!("left:{}%;top:{}%;transform:translate(-50%,-50%);opacity:{};",
                        e.ship_x, e.ship_y, op)>
                    <div inner_html={svg}></div>
                </div>
            }.into_view())
        }).collect::<Vec<_>>()}
    }
}
