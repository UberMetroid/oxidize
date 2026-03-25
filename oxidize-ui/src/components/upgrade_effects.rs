//! Upgrade deployment animations rendering
//! 
//! Props: upgrade_effects, planet_angles
//! Renders all upgrade deployment animations:
//! - SolarSail: Unfurling sail with cyan gradient
//! - PlasmaTether: Lightning bolt with sparkles
//! - OrbitalMirror: Spinning hexagon
//! - DysonCollector: Concentric rotating rings
//! - QuantumArray: Glitching star polygon
//! - StellarEngine: Expanding shockwave

use leptos::*;

use crate::types::UpgradeEffect;
use oxidize_engine::UpgradeType;

#[component]
pub fn UpgradeEffects(
    upgrade_effects: ReadSignal<Vec<UpgradeEffect>>,
) -> impl IntoView {
    view! {
        {move || upgrade_effects.get().iter().map(|effect| {
            let x = effect.ship_x;
            let y = effect.ship_y;
            let p = effect.progress;
            let fade_opacity = if p < 0.1 {
                p * 10.0
            } else if p > 0.7 && !effect.permanent {
                (1.0 - p) / 0.3
            } else {
                1.0
            };

            match effect.upgrade_type {
                UpgradeType::SolarSail => {
                    let op = fade_opacity;
                    view! {
                        <div class="absolute pointer-events-none" style=format!("left: {}%%; top: {}%%; transform: translate(-50%%, -50%%); opacity: {};", x, y, op)>
                            <svg width="80" height="80" style="position:absolute;left:50%%;top:50%%;transform:translate(-50%%,-50%%);">
                                <defs>
                                    <linearGradient id="sail-grad" x1="0%%" y1="0%%" x2="100%%" y2="0%%">
                                        <stop offset="0%%" stop-color="#a5f3fc" stop-opacity="0.8"/>
                                        <stop offset="100%%" stop-color="#67e8f9" stop-opacity="0.3"/>
                                    </linearGradient>
                                </defs>
                                <path d="M40,15 Q55,30 40,45 Q25,30 40,15" fill="url(#sail-grad)" style="animation:sail-unfurl 2s ease-out forwards;"/>
                                <path d="M25,20 Q40,35 25,50 Q10,35 25,20" fill="url(#sail-grad)" style="animation:sail-unfurl 2s ease-out forwards 0.1s;"/>
                                <path d="M55,20 Q70,35 55,50 Q40,35 55,20" fill="url(#sail-grad)" style="animation:sail-unfurl 2s ease-out forwards 0.2s;"/>
                            </svg>
                        </div>
                    }.into_view()
                }
                UpgradeType::PlasmaTether => {
                    let op = fade_opacity;
                    view! {
                        <div class="absolute pointer-events-none" style=format!("left: {}%%; top: {}%%; transform: translate(-50%%, -50%%); opacity: {};", x, y, op)>
                            <svg width="120" height="60" style="position:absolute;left:50%%;top:50%%;transform:translate(-50%%,-50%%);">
                                <polyline points="0,30 20,10 40,30 60,10 80,30 100,10 120,30" fill="none" stroke="#00d4ff" stroke-width="3" style="filter:drop-shadow(0 0 4px #00d4ff) drop-shadow(0 0 8px #00d4ff);animation:plasma-zap 1.5s ease-in-out infinite;"/>
                                <circle cx="15" cy="20" r="2" fill="#00ffff" style="animation:sparkle 0.5s ease-in-out infinite;"/>
                                <circle cx="55" cy="20" r="2" fill="#00ffff" style="animation:sparkle 0.5s ease-in-out infinite 0.2s;"/>
                                <circle cx="95" cy="20" r="2" fill="#00ffff" style="animation:sparkle 0.5s ease-in-out infinite 0.4s;"/>
                            </svg>
                        </div>
                    }.into_view()
                }
                UpgradeType::OrbitalMirror => {
                    let op = fade_opacity;
                    view! {
                        <div class="absolute pointer-events-none" style=format!("left: {}%%; top: {}%%; transform: translate(-50%%, -50%%); opacity: {};", x, y, op)>
                            <svg width="60" height="60" style="position:absolute;left:50%%;top:50%%;transform:translate(-50%%,-50%%);">
                                <polygon points="30,5 52,17.5 52,42.5 30,55 8,42.5 8,17.5" fill="none" stroke="#fef08a" stroke-width="2" style="filter:drop-shadow(0 0 6px #fef08a);animation:mirror-spin 2s ease-out forwards;"/>
                                <circle cx="30" cy="30" r="4" fill="#fef08a" style="animation:sparkle 0.3s ease-in-out infinite;"/>
                            </svg>
                        </div>
                    }.into_view()
                }
                UpgradeType::DysonCollector => {
                    let op = if effect.permanent { 1.0 } else { fade_opacity };
                    view! {
                        <div class="absolute pointer-events-none" style=format!("left: {}%%; top: {}%%; transform: translate(-50%%, -50%%); opacity: {};", x, y, op)>
                            <svg width="80" height="80" style="position:absolute;left:50%%;top:50%%;transform:translate(-50%%,-50%%);">
                                <circle cx="40" cy="40" r="35" fill="none" stroke="#fbbf24" stroke-width="3" stroke-dasharray="4,2" style="animation:dyson-assemble 3s ease-out forwards;filter:drop-shadow(0 0 8px #fbbf24);"/>
                                <circle cx="40" cy="40" r="25" fill="none" stroke="#f59e0b" stroke-width="2" style="animation:dyson-assemble 3s ease-out forwards;filter:drop-shadow(0 0 4px #f59e0b);"/>
                            </svg>
                        </div>
                    }.into_view()
                }
                UpgradeType::QuantumArray => {
                    let op = fade_opacity;
                    view! {
                        <div class="absolute pointer-events-none" style=format!("left: {}%%; top: {}%%; transform: translate(-50%%, -50%%); opacity: {};", x, y, op)>
                            <svg width="70" height="70" style="position:absolute;left:50%%;top:50%%;transform:translate(-50%%,-50%%);">
                                <polygon points="35,5 45,20 65,25 50,40 55,60 35,50 15,60 20,40 5,25 25,20" fill="none" stroke="#c084fc" stroke-width="2" style="filter:drop-shadow(0 0 8px #c084fc);animation:quantum-glitch 2s ease-in-out forwards;"/>
                                <polygon points="35,15 42,25 52,27 44,35 46,45 35,40 24,45 26,35 18,27 28,25" fill="#c084fc" style="animation:quantum-glitch 2s ease-in-out infinite;"/>
                            </svg>
                        </div>
                    }.into_view()
                }
                UpgradeType::StellarEngine => {
                    let op = if effect.permanent { 1.0 } else { fade_opacity };
                    view! {
                        <div class="absolute pointer-events-none" style=format!("left: {}%%; top: {}%%; transform: translate(-50%%, -50%%); opacity: {};", x, y, op)>
                            <svg width="120" height="120" style="position:absolute;left:50%%;top:50%%;transform:translate(-50%%,-50%%);">
                                <circle cx="60" cy="60" r="50" fill="none" stroke="#ff6b00" stroke-width="2" style="animation:shockwave 3.5s ease-out infinite;transform-origin:60px 60px;"/>
                                <circle cx="60" cy="60" r="20" fill="#fff" style="animation:stellar-flare 3.5s ease-out forwards;transform-origin:60px 60px;"/>
                                <circle cx="60" cy="60" r="35" fill="none" stroke="#ff8c00" stroke-width="1" style="animation:shockwave 3.5s ease-out infinite 0.5s;transform-origin:60px 60px;"/>
                            </svg>
                        </div>
                    }.into_view()
                }
            }
        }).collect::<Vec<_>>()}
    }
}
