//! Solar System visualization component

use leptos::*;

use crate::constants::*;
use crate::types::LaunchEffect;

#[allow(unused_variables)]
#[component]
pub fn SolarSystem(
    planet_angles: ReadSignal<Vec<f64>>,
    moon_angles: ReadSignal<Vec<f64>>,
    spaceship_angle: ReadSignal<f64>,
    target_planet_idx: ReadSignal<Option<usize>>,
    is_flying: ReadSignal<bool>,
    fly_progress: ReadSignal<f64>,
    fly_from_x: ReadSignal<f64>,
    fly_from_y: ReadSignal<f64>,
    fly_to_x: ReadSignal<f64>,
    fly_to_y: ReadSignal<f64>,
    fly_x: ReadSignal<f64>,
    fly_y: ReadSignal<f64>,
    planet_offset: ReadSignal<f64>,
    launch_effects: ReadSignal<Vec<LaunchEffect>>,
) -> impl IntoView {
    view! {
        {/* Inject keyframes and global styles */}
        <style>
            {r#"
                @keyframes sun-pulse {
                    0%, 100% { opacity: 0.6; transform: translate(-50%, -50%) scale(1); }
                    50% { opacity: 1; transform: translate(-50%, -50%) scale(1.15); }
                }
                @keyframes corona-pulse {
                    0%, 100% { opacity: 0.15; transform: translate(-50%, -50%) scale(1); }
                    50% { opacity: 0.35; transform: translate(-50%, -50%) scale(1.1); }
                }
                @keyframes star-twinkle {
                    0%, 100% { opacity: var(--base-opacity); }
                    50% { opacity: calc(var(--base-opacity) * 0.3); }
                }
                @keyframes engine-glow {
                    0%, 100% { opacity: 0.7; }
                    50% { opacity: 1; }
                }
                @keyframes engine-trail {
                    0% { opacity: 0.8; transform: translate(-50%, 2px) scaleY(1); }
                    100% { opacity: 0; transform: translate(-50%, 6px) scaleY(1.8); }
                }
                .jupiter-bands {
                    position: absolute;
                    left: 0; top: 0;
                    width: 100%; height: 100%;
                    border-radius: 50%;
                    background: repeating-linear-gradient(
                        0deg,
                        transparent 0%,
                        transparent 18%,
                        #8b45133f 18%,
                        #8b45133f 22%,
                        transparent 22%,
                        transparent 38%,
                        #b4641433 38%,
                        #b4641433 42%,
                        transparent 42%,
                        transparent 58%,
                        #8b45133f 58%,
                        #8b45133f 62%,
                        transparent 62%,
                        transparent 78%,
                        #b4641433 78%,
                        #b4641433 82%
                    );
                    overflow: hidden;
                    border-radius: 50%;
                    pointer-events: none;
                }
            "#}
        </style>

        {/* Star field background */}
        <svg class="absolute inset-0 w-full h-full pointer-events-none" style="z-index: 0;">
            {/* Layer 1: bright small stars */}
            <circle cx="5%" cy="10%" r="1px" fill="#fff" opacity="0.9" style="--base-opacity: 0.9; animation: star-twinkle 3.1s ease-in-out infinite;"/>
            <circle cx="12%" cy="25%" r="1.5px" fill="#fff" opacity="0.7" style="--base-opacity: 0.7; animation: star-twinkle 4.2s ease-in-out infinite 0.5s;"/>
            <circle cx="8%" cy="40%" r="1px" fill="#fff" opacity="1" style="--base-opacity: 1; animation: star-twinkle 2.8s ease-in-out infinite 1s;"/>
            <circle cx="15%" cy="55%" r="0.5px" fill="#fff" opacity="0.8" style="--base-opacity: 0.8; animation: star-twinkle 3.7s ease-in-out infinite 0.3s;"/>
            <circle cx="3%" cy="70%" r="1px" fill="#fff" opacity="0.6" style="--base-opacity: 0.6; animation: star-twinkle 5s ease-in-out infinite;"/>
            <circle cx="18%" cy="80%" r="1.5px" fill="#fff" opacity="0.9" style="--base-opacity: 0.9; animation: star-twinkle 3.3s ease-in-out infinite 0.7s;"/>
            <circle cx="22%" cy="8%" r="1px" fill="#fff" opacity="0.8" style="--base-opacity: 0.8; animation: star-twinkle 4.5s ease-in-out infinite 0.2s;"/>
            <circle cx="28%" cy="18%" r="0.5px" fill="#fff" opacity="1" style="--base-opacity: 1; animation: star-twinkle 2.5s ease-in-out infinite 1.2s;"/>
            <circle cx="25%" cy="35%" r="1.5px" fill="#fff" opacity="0.7" style="--base-opacity: 0.7; animation: star-twinkle 3.9s ease-in-out infinite;"/>
            <circle cx="30%" cy="50%" r="1px" fill="#fff" opacity="0.9" style="--base-opacity: 0.9; animation: star-twinkle 4.1s ease-in-out infinite 0.6s;"/>
            <circle cx="20%" cy="65%" r="1px" fill="#fff" opacity="0.8" style="--base-opacity: 0.8; animation: star-twinkle 3.4s ease-in-out infinite 0.9s;"/>
            <circle cx="32%" cy="75%" r="0.5px" fill="#fff" opacity="0.6" style="--base-opacity: 0.6; animation: star-twinkle 4.8s ease-in-out infinite;"/>
            <circle cx="35%" cy="5%" r="1px" fill="#fff" opacity="1" style="--base-opacity: 1; animation: star-twinkle 2.9s ease-in-out infinite 0.4s;"/>
            <circle cx="40%" cy="15%" r="1.5px" fill="#fff" opacity="0.7" style="--base-opacity: 0.7; animation: star-twinkle 3.6s ease-in-out infinite 1.1s;"/>
            <circle cx="38%" cy="30%" r="0.5px" fill="#fff" opacity="0.9" style="--base-opacity: 0.9; animation: star-twinkle 4.3s ease-in-out infinite;"/>
            <circle cx="42%" cy="45%" r="1px" fill="#fff" opacity="0.8" style="--base-opacity: 0.8; animation: star-twinkle 3.2s ease-in-out infinite 0.8s;"/>
            <circle cx="36%" cy="60%" r="1px" fill="#fff" opacity="0.6" style="--base-opacity: 0.6; animation: star-twinkle 5.1s ease-in-out infinite 0.1s;"/>
            <circle cx="45%" cy="85%" r="1.5px" fill="#fff" opacity="1" style="--base-opacity: 1; animation: star-twinkle 2.7s ease-in-out infinite 1.3s;"/>
            <circle cx="50%" cy="12%" r="1px" fill="#fff" opacity="0.7" style="--base-opacity: 0.7; animation: star-twinkle 4s ease-in-out infinite;"/>
            <circle cx="55%" cy="28%" r="0.5px" fill="#fff" opacity="0.9" style="--base-opacity: 0.9; animation: star-twinkle 3.5s ease-in-out infinite 0.55s;"/>
            <circle cx="52%" cy="42%" r="1.5px" fill="#fff" opacity="0.8" style="--base-opacity: 0.8; animation: star-twinkle 4.4s ease-in-out infinite;"/>
            <circle cx="58%" cy="55%" r="1px" fill="#fff" opacity="0.6" style="--base-opacity: 0.6; animation: star-twinkle 4.7s ease-in-out infinite 0.35s;"/>
            <circle cx="48%" cy="70%" r="1px" fill="#fff" opacity="1" style="--base-opacity: 1; animation: star-twinkle 3s ease-in-out infinite 0.75s;"/>
            <circle cx="60%" cy="78%" r="0.5px" fill="#fff" opacity="0.7" style="--base-opacity: 0.7; animation: star-twinkle 4.9s ease-in-out infinite;"/>
            <circle cx="65%" cy="8%" r="1px" fill="#fff" opacity="0.9" style="--base-opacity: 0.9; animation: star-twinkle 3.8s ease-in-out infinite 0.25s;"/>
            <circle cx="70%" cy="22%" r="1.5px" fill="#fff" opacity="0.8" style="--base-opacity: 0.8; animation: star-twinkle 2.6s ease-in-out infinite 1.05s;"/>
            <circle cx="68%" cy="38%" r="0.5px" fill="#fff" opacity="0.6" style="--base-opacity: 0.6; animation: star-twinkle 5.2s ease-in-out infinite;"/>
            <circle cx="72%" cy="52%" r="1px" fill="#fff" opacity="1" style="--base-opacity: 1; animation: star-twinkle 3.3s ease-in-out infinite 0.45s;"/>
            <circle cx="66%" cy="68%" r="1px" fill="#fff" opacity="0.7" style="--base-opacity: 0.7; animation: star-twinkle 4.2s ease-in-out infinite;"/>
            <circle cx="75%" cy="82%" r="1.5px" fill="#fff" opacity="0.9" style="--base-opacity: 0.9; animation: star-twinkle 2.8s ease-in-out infinite 0.85s;"/>
            <circle cx="78%" cy="15%" r="1px" fill="#fff" opacity="0.8" style="--base-opacity: 0.8; animation: star-twinkle 4.6s ease-in-out infinite;"/>
            <circle cx="82%" cy="30%" r="0.5px" fill="#fff" opacity="0.6" style="--base-opacity: 0.6; animation: star-twinkle 3.9s ease-in-out infinite 0.15s;"/>
            <circle cx="80%" cy="48%" r="1.5px" fill="#fff" opacity="1" style="--base-opacity: 1; animation: star-twinkle 2.4s ease-in-out infinite 0.65s;"/>
            <circle cx="85%" cy="60%" r="1px" fill="#fff" opacity="0.7" style="--base-opacity: 0.7; animation: star-twinkle 4.1s ease-in-out infinite;"/>
            <circle cx="76%" cy="75%" r="1px" fill="#fff" opacity="0.9" style="--base-opacity: 0.9; animation: star-twinkle 3.7s ease-in-out infinite 0.95s;"/>
            <circle cx="88%" cy="85%" r="0.5px" fill="#fff" opacity="0.8" style="--base-opacity: 0.8; animation: star-twinkle 5s ease-in-out infinite;"/>
            <circle cx="90%" cy="5%" r="1px" fill="#fff" opacity="0.6" style="--base-opacity: 0.6; animation: star-twinkle 3.4s ease-in-out infinite 0.5s;"/>
            <circle cx="95%" cy="20%" r="1.5px" fill="#fff" opacity="1" style="--base-opacity: 1; animation: star-twinkle 2.9s ease-in-out infinite 0.2s;"/>
            <circle cx="92%" cy="35%" r="0.5px" fill="#fff" opacity="0.7" style="--base-opacity: 0.7; animation: star-twinkle 4.5s ease-in-out infinite;"/>
            <circle cx="97%" cy="50%" r="1px" fill="#fff" opacity="0.9" style="--base-opacity: 0.9; animation: star-twinkle 3.1s ease-in-out infinite 0.7s;"/>
            <circle cx="94%" cy="65%" r="1px" fill="#fff" opacity="0.8" style="--base-opacity: 0.8; animation: star-twinkle 4.8s ease-in-out infinite;"/>
            <circle cx="98%" cy="80%" r="1.5px" fill="#fff" opacity="0.6" style="--base-opacity: 0.6; animation: star-twinkle 3.6s ease-in-out infinite 0.35s;"/>
            <circle cx="10%" cy="90%" r="1px" fill="#fff" opacity="1" style="--base-opacity: 1; animation: star-twinkle 2.7s ease-in-out infinite 0.55s;"/>
            <circle cx="25%" cy="95%" r="0.5px" fill="#fff" opacity="0.7" style="--base-opacity: 0.7; animation: star-twinkle 4.3s ease-in-out infinite;"/>
            <circle cx="40%" cy="92%" r="1.5px" fill="#fff" opacity="0.9" style="--base-opacity: 0.9; animation: star-twinkle 3.2s ease-in-out infinite 0.15s;"/>
            <circle cx="55%" cy="98%" r="1px" fill="#fff" opacity="0.8" style="--base-opacity: 0.8; animation: star-twinkle 5.3s ease-in-out infinite;"/>
            <circle cx="70%" cy="93%" r="1px" fill="#fff" opacity="0.6" style="--base-opacity: 0.6; animation: star-twinkle 4s ease-in-out infinite 0.85s;"/>
            <circle cx="85%" cy="95%" r="0.5px" fill="#fff" opacity="1" style="--base-opacity: 1; animation: star-twinkle 2.5s ease-in-out infinite 0.4s;"/>
            {/* Layer 2: extra scattered stars for depth */}
            <circle cx="7%" cy="17%" r="0.5px" fill="#fff" opacity="0.5" style="--base-opacity: 0.5; animation: star-twinkle 6s ease-in-out infinite 1s;"/>
            <circle cx="17%" cy="42%" r="0.5px" fill="#fff" opacity="0.4" style="--base-opacity: 0.4; animation: star-twinkle 5.5s ease-in-out infinite 0.3s;"/>
            <circle cx="33%" cy="3%" r="0.5px" fill="#fff" opacity="0.5" style="--base-opacity: 0.5; animation: star-twinkle 6.5s ease-in-out infinite 0.8s;"/>
            <circle cx="43%" cy="58%" r="0.5px" fill="#fff" opacity="0.4" style="--base-opacity: 0.4; animation: star-twinkle 7s ease-in-out infinite;"/>
            <circle cx="57%" cy="63%" r="0.5px" fill="#fff" opacity="0.5" style="--base-opacity: 0.5; animation: star-twinkle 5.8s ease-in-out infinite 0.6s;"/>
            <circle cx="73%" cy="7%" r="0.5px" fill="#fff" opacity="0.4" style="--base-opacity: 0.4; animation: star-twinkle 6.2s ease-in-out infinite 1.2s;"/>
            <circle cx="83%" cy="73%" r="0.5px" fill="#fff" opacity="0.5" style="--base-opacity: 0.5; animation: star-twinkle 5.4s ease-in-out infinite;"/>
            <circle cx="93%" cy="43%" r="0.5px" fill="#fff" opacity="0.4" style="--base-opacity: 0.4; animation: star-twinkle 7.2s ease-in-out infinite 0.45s;"/>
        </svg>

        {/* Orbital path rings */}
        <svg class="absolute inset-0 w-full h-full pointer-events-none" style="z-index: 1;">
            <circle cx="50%" cy="50%" r="8%" fill="none" stroke="#9ca3af" stroke-width="0.3" stroke-dasharray="1,3" opacity="0.3"/>
            <circle cx="50%" cy="50%" r="14%" fill="none" stroke="#fbbf24" stroke-width="0.3" stroke-dasharray="1,3" opacity="0.25"/>
            <circle cx="50%" cy="50%" r="20%" fill="none" stroke="#3b82f6" stroke-width="0.3" stroke-dasharray="1,3" opacity="0.25"/>
            <circle cx="50%" cy="50%" r="27%" fill="none" stroke="#ef4444" stroke-width="0.3" stroke-dasharray="1,3" opacity="0.25"/>
            <circle cx="50%" cy="50%" r="38%" fill="none" stroke="#f97316" stroke-width="0.3" stroke-dasharray="1,3" opacity="0.25"/>
            <circle cx="50%" cy="50%" r="50%" fill="none" stroke="#eab308" stroke-width="0.3" stroke-dasharray="1,3" opacity="0.25"/>
            <circle cx="50%" cy="50%" r="62%" fill="none" stroke="#06b6d4" stroke-width="0.3" stroke-dasharray="1,3" opacity="0.25"/>
            <circle cx="50%" cy="50%" r="75%" fill="none" stroke="#6366f1" stroke-width="0.3" stroke-dasharray="1,3" opacity="0.25"/>
        </svg>

        {/* Sun - glowing center */}
        <div class="absolute pointer-events-none" style="left: 50%; top: 50%; transform: translate(-50%, -50%); z-index: 5;">
            {/* Outer corona glow */}
            <div style="position: absolute; left: 50%; top: 50%; transform: translate(-50%, -50%); width: 40px; height: 40px; border-radius: 50%; background: radial-gradient(circle, #ffc83266 0%, #ff960019 60%, transparent 100%); animation: corona-pulse 3s ease-in-out infinite;"></div>
            {/* Mid glow */}
            <div style="position: absolute; left: 50%; top: 50%; transform: translate(-50%, -50%); width: 22px; height: 22px; border-radius: 50%; background: radial-gradient(circle, #fff 0%, #ffd700 40%, #ff8c00 80%, #ff6600 100%); animation: sun-pulse 2s ease-in-out infinite; box-shadow: 0 0 12px #ff8c00, 0 0 24px #ffa500, 0 0 40px #ffa5007f;"></div>
        </div>

        {/* Mercury */}
        <div class="absolute pointer-events-none" style=move || { let porbits = planet_angles.get(); let i = 0; let angle = PLANET_INITIAL_ANGLES[i] + porbits[i]; let r = PLANET_DATA[i].0; let x = 50.0 + r * angle.cos(); let y = 50.0 + r * angle.sin(); format!("left: {}%; top: {}%; transform: translate(-50%, -50%); z-index: 10;", x, y) }>
            <div style=format!("width: {}px; height: {}px; background: radial-gradient(circle at 35%% 35%%, #d1d5db, #6b7280 60%%, #374151); border-radius: 50%%; box-shadow: 0 0 3px #9ca3af, inset -1px -1px 2px #00000066;", PLANET_DATA[0].1, PLANET_DATA[0].1)></div>
        </div>
        {/* Venus */}
        <div class="absolute pointer-events-none" style=move || { let porbits = planet_angles.get(); let i = 1; let angle = PLANET_INITIAL_ANGLES[i] + porbits[i]; let r = PLANET_DATA[i].0; let x = 50.0 + r * angle.cos(); let y = 50.0 + r * angle.sin(); format!("left: {}%; top: {}%; transform: translate(-50%, -50%); z-index: 11;", x, y) }>
            <div style=format!("width: {}px; height: {}px; background: radial-gradient(circle at 35%% 35%%, #fef3c7, #fbbf24 50%%, #d97706); border-radius: 50%%; box-shadow: 0 0 6px #fbbf24cc, inset -1px -1px 3px #0000004c;", PLANET_DATA[1].1, PLANET_DATA[1].1)></div>
        </div>
        {/* Earth */}
        <div class="absolute pointer-events-none" style=move || { let porbits = planet_angles.get(); let i = 2; let angle = PLANET_INITIAL_ANGLES[i] + porbits[i]; let r = PLANET_DATA[i].0; let x = 50.0 + r * angle.cos(); let y = 50.0 + r * angle.sin(); format!("left: {}%; top: {}%; transform: translate(-50%, -50%); z-index: 12;", x, y) }>
            <div style=format!("width: {}px; height: {}px; background: radial-gradient(circle at 35%% 35%%, #93c5fd, #3b82f6 50%%, #1e3a8a); border-radius: 50%%; box-shadow: 0 0 6px #3b82f6e5, inset -1px -1px 3px #0000004c;", PLANET_DATA[2].1, PLANET_DATA[2].1)></div>
        </div>
        {/* Mars */}
        <div class="absolute pointer-events-none" style=move || { let porbits = planet_angles.get(); let i = 3; let angle = PLANET_INITIAL_ANGLES[i] + porbits[i]; let r = PLANET_DATA[i].0; let x = 50.0 + r * angle.cos(); let y = 50.0 + r * angle.sin(); format!("left: {}%; top: {}%; transform: translate(-50%, -50%); z-index: 13;", x, y) }>
            <div style=format!("width: {}px; height: {}px; background: radial-gradient(circle at 35%% 35%%, #fca5a5, #ef4444 50%%, #7f1d1d); border-radius: 50%%; box-shadow: 0 0 4px #ef4444cc, inset -1px -1px 2px #0000004c;", PLANET_DATA[3].1, PLANET_DATA[3].1)></div>
        </div>

        {/* Asteroid Belt - scattered rocks */}
        <svg class="absolute inset-0 w-full h-full pointer-events-none" style="z-index: 5;">
            <circle cx="50%" cy="50%" r="32%" fill="none" stroke="#555" stroke-width="3%" opacity="0.08"/>
            <circle cx="50%" cy="50%" r="35%" fill="none" stroke="#666" stroke-width="2%" opacity="0.06"/>
            <circle cx="50%" cy="50%" r="30.5%" fill="none" stroke="#777" stroke-width="1.5%" opacity="0.1"/>
            <circle cx="52%" cy="46%" r="1px" fill="#888" opacity="0.6"/>
            <circle cx="48%" cy="53%" r="1.5px" fill="#777" opacity="0.5"/>
            <circle cx="55%" cy="48%" r="1px" fill="#999" opacity="0.7"/>
            <circle cx="51%" cy="55%" r="1px" fill="#666" opacity="0.4"/>
            <circle cx="46%" cy="47%" r="1.5px" fill="#888" opacity="0.55"/>
            <circle cx="54%" cy="43%" r="1px" fill="#777" opacity="0.6"/>
            <circle cx="49%" cy="57%" r="1px" fill="#999" opacity="0.5"/>
            <circle cx="56%" cy="52%" r="1px" fill="#666" opacity="0.65"/>
            <circle cx="47%" cy="51%" r="1.5px" fill="#888" opacity="0.45"/>
            <circle cx="53%" cy="45%" r="1px" fill="#777" opacity="0.55"/>
            <circle cx="50%" cy="49%" r="1px" fill="#999" opacity="0.5"/>
            <circle cx="44%" cy="50%" r="1.5px" fill="#666" opacity="0.4"/>
            <circle cx="57%" cy="49%" r="1px" fill="#888" opacity="0.6"/>
            <circle cx="48%" cy="44%" r="1px" fill="#777" opacity="0.5"/>
            <circle cx="52%" cy="56%" r="1.5px" fill="#999" opacity="0.45"/>
        </svg>

        {/* Jupiter - large with bands */}
        <div class="absolute pointer-events-none" style=move || { let porbits = planet_angles.get(); let i = 4; let angle = PLANET_INITIAL_ANGLES[i] + porbits[i]; let r = PLANET_DATA[i].0; let x = 50.0 + r * angle.cos(); let y = 50.0 + r * angle.sin(); format!("left: {}%; top: {}%; transform: translate(-50%, -50%); z-index: 14;", x, y) }>
            <div style=format!("width: {}px; height: {}px; background: radial-gradient(circle at 35%% 35%%, #fcd34d, #f97316 50%%, #c2410c); border-radius: 50%%; box-shadow: 0 0 8px #f97316cc, inset -2px -2px 4px #0000004c;", PLANET_DATA[4].1, PLANET_DATA[4].1)></div>
            {/* Jupiter band stripes */}
            <div class="jupiter-bands"></div>
            {/* Great Red Spot hint */}
            <div style=format!("position: absolute; left: 55%%; top: 60%%; width: {}px; height: {}px; background: radial-gradient(ellipse, #ef4444, #b91c1c, transparent); border-radius: 50%%; opacity: 0.7;", PLANET_DATA[4].1 * 0.22, PLANET_DATA[4].1 * 0.14)></div>
        </div>
        {/* Saturn - with 3-layer rings */}
        <div class="absolute pointer-events-none" style=move || { let porbits = planet_angles.get(); let i = 5; let angle = PLANET_INITIAL_ANGLES[i] + porbits[i]; let r = PLANET_DATA[i].0; let x = 50.0 + r * angle.cos(); let y = 50.0 + r * angle.sin(); format!("left: {}%; top: {}%; transform: translate(-50%, -50%); z-index: 15;", x, y) }>
            <div style=format!("width: {}px; height: {}px; background: radial-gradient(circle at 35%% 35%%, #fef9c3, #eab308 50%%, #a16207); border-radius: 50%%; box-shadow: 0 0 6px #eab308b2, inset -1px -1px 3px #0000004c;", PLANET_DATA[5].1, PLANET_DATA[5].1)></div>
            {/* Ring - outer faint */}
            <div style=format!("position: absolute; left: 50%; top: 50%; transform: translate(-50%, -50%) rotate(-20deg); width: {}px; height: {}px; border: 2px {}; border-radius: 50%; opacity: 0.3; box-shadow: 0 0 3px #eab3084c;", PLANET_DATA[5].1 * 3.4, PLANET_DATA[5].1 * 0.85, "#d4a574")></div>
            {/* Ring - middle */}
            <div style=format!("position: absolute; left: 50%; top: 50%; transform: translate(-50%, -50%) rotate(-20deg); width: {}px; height: {}px; border: 2px {}; border-radius: 50%; opacity: 0.5; box-shadow: 0 0 5px #eab30866;", PLANET_DATA[5].1 * 2.8, PLANET_DATA[5].1 * 0.75, "#c0a060")></div>
            {/* Ring - inner bright */}
            <div style=format!("position: absolute; left: 50%; top: 50%; transform: translate(-50%, -50%) rotate(-20deg); width: {}px; height: {}px; border: 2px {}; border-radius: 50%; opacity: 0.7; box-shadow: 0 0 8px #eab3087f;", PLANET_DATA[5].1 * 2.3, PLANET_DATA[5].1 * 0.65, "#eab308")></div>
        </div>
        {/* Uranus */}
        <div class="absolute pointer-events-none" style=move || { let porbits = planet_angles.get(); let i = 6; let angle = PLANET_INITIAL_ANGLES[i] + porbits[i]; let r = PLANET_DATA[i].0; let x = 50.0 + r * angle.cos(); let y = 50.0 + r * angle.sin(); format!("left: {}%; top: {}%; transform: translate(-50%, -50%); z-index: 16;", x, y) }>
            <div style=format!("width: {}px; height: {}px; background: radial-gradient(circle at 35%% 35%%, #a5f3fc, #06b6d4 50%%, #0e7490); border-radius: 50%%; box-shadow: 0 0 5px #06b6d4cc, inset -1px -1px 3px #0000004c;", PLANET_DATA[6].1, PLANET_DATA[6].1)></div>
        </div>
        {/* Neptune */}
        <div class="absolute pointer-events-none" style=move || { let porbits = planet_angles.get(); let i = 7; let angle = PLANET_INITIAL_ANGLES[i] + porbits[i]; let r = PLANET_DATA[i].0; let x = 50.0 + r * angle.cos(); let y = 50.0 + r * angle.sin(); format!("left: {}%; top: {}%; transform: translate(-50%, -50%); z-index: 17;", x, y) }>
            <div style=format!("width: {}px; height: {}px; background: radial-gradient(circle at 35%% 35%%, #a5b4fc, #6366f1 50%%, #4338ca); border-radius: 50%%; box-shadow: 0 0 6px #6366f1cc, inset -1px -1px 3px #0000004c;", PLANET_DATA[7].1, PLANET_DATA[7].1)></div>
        </div>

        {/* Moons */}
        <svg class="absolute inset-0 w-full h-full pointer-events-none" style="z-index: 18;">
            {move || {
                let porbits = planet_angles.get();
                let mangles = moon_angles.get();
                let mut moon_idx = 0;
                let mut paths = String::new();
                for (pidx, moons) in MOON_DATA.iter().enumerate() {
                    let planet_angle = PLANET_INITIAL_ANGLES[pidx] + porbits[pidx];
                    let planet_orbit_r = PLANET_DATA[pidx].0;
                    let planet_x = 50.0 + planet_orbit_r * planet_angle.cos();
                    let planet_y = 50.0 + planet_orbit_r * planet_angle.sin();
                    for moon in moons.iter() {
                        let (_, moon_orbit_r, moon_size, moon_color) = *moon;
                        let angle = mangles[moon_idx];
                        let moon_x = planet_x + moon_orbit_r * angle.cos();
                        let moon_y = planet_y + moon_orbit_r * angle.sin();
                        paths.push_str(&format!(
                            r#"<circle cx="{}%" cy="{}%" r="{}px" fill="{}" opacity="0.95"/>"#,
                            moon_x, moon_y, moon_size, moon_color
                        ));
                        moon_idx += 1;
                    }
                }
                view! { <g inner_html={paths}></g> }
            }}
        </svg>

        {/* Spaceship */}
        <div class="absolute pointer-events-none" style=move || {
            let ship_orbit_angle = spaceship_angle.get();
            let orbiting_idx = target_planet_idx.get();
            let flying = is_flying.get();
            let porbits = planet_angles.get();
            let (world_x, world_y, facing) = if flying {
                let x = fly_x.get();
                let y = fly_y.get();
                let dx = x - fly_from_x.get();
                let dy = y - fly_from_y.get();
                let facing = dy.atan2(dx);
                (x, y, facing)
            } else if let Some(idx) = orbiting_idx {
                let planet_angle = PLANET_INITIAL_ANGLES[idx] + porbits[idx];
                let planet_orbit_r = PLANET_DATA[idx].0;
                let planet_x = 50.0 + planet_orbit_r * planet_angle.cos();
                let planet_y = 50.0 + planet_orbit_r * planet_angle.sin();
                let ship_orbit_r = SHIP_PLANET_ORBIT_RADIUS;
                let offset = planet_offset.get();
                let ship_angle = planet_angle + offset;
                let ship_x = planet_x + ship_orbit_r * ship_angle.cos();
                let ship_y = planet_y + ship_orbit_r * ship_angle.sin();
                let facing = ship_angle + std::f64::consts::FRAC_PI_2;
                (ship_x, ship_y, facing)
            } else {
                let x = 50.0 + SHIP_ORBIT_RADIUS as f64 * ship_orbit_angle.cos();
                let y = 50.0 + SHIP_ORBIT_RADIUS as f64 * ship_orbit_angle.sin();
                let facing = ship_orbit_angle + std::f64::consts::FRAC_PI_2;
                (x, y, facing)
            };
            format!("left: {}%; top: {}%; transform: translate(-50%, -50%) rotate({}rad); z-index: 30;", world_x, world_y, facing)
        }>
            {/* Engine trail glow */}
            <div style="position: absolute; left: 50%; top: 50%; width: 0; height: 0; border-left: 2px solid transparent; border-right: 2px solid transparent; border-top: 8px solid #f97316; filter: blur(1px); opacity: 0.6; transform: translate(-50%, 2px); animation: engine-glow 0.3s ease-in-out infinite;"></div>
            {/* Ship body */}
            <div style="width: 0; height: 0; border-left: 3.5px solid transparent; border-right: 3.5px solid transparent; border-bottom: 7px solid #ef4444; filter: drop-shadow(0 0 4px #ef4444);"></div>
            {/* Cockpit */}
            <div style="position: absolute; left: 50%; top: 2px; transform: translateX(-50%); width: 3px; height: 3px; background: #fef08a; border-radius: 50%; box-shadow: 0 0 3px #fef08a;"></div>
        </div>

        {/* Launch effects */}
        {move || launch_effects.get().iter().map(|effect| {
            let target_distance = 120.0;
            let current_distance = 80.0 + (target_distance - 80.0) * effect.progress;
            let x = 50.0 + current_distance * effect.angle.cos();
            let y = 50.0 + current_distance * effect.angle.sin();
            let opacity = if effect.progress < 0.2 { effect.progress * 5.0 } else if effect.progress > 0.8 { (1.0 - effect.progress) * 5.0 } else { 1.0 };
            let scale = 0.3 + effect.progress * 0.7;
            view! {
                <div class="absolute pointer-events-none z-20" style=format!("left: {}%; top: {}%; transform: translate(-50%, -50%) scale({}); opacity: {};", x, y, scale, opacity)>
                    <div style="width: 12px; height: 12px; border-radius: 50%%; background: radial-gradient(circle, #fef08a, #fbbf24); box-shadow: 0 0 8px #fbbf24, 0 0 16px #fbbf247f;"></div>
                </div>
            }
        }).collect::<Vec<_>>()}
    }
}
