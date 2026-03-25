//! Solar System visualization component

use leptos::*;
use oxidize_engine::UpgradeType;

use crate::constants::*;
use crate::types::UpgradeEffect;

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
    upgrade_effects: ReadSignal<Vec<UpgradeEffect>>,
    trail_positions: ReadSignal<Vec<(f64, f64, f64)>>,
    flight_path: ReadSignal<Vec<(f64, f64)>>,
    flight_prediction: ReadSignal<Vec<(f64, f64)>>,
    is_arriving: ReadSignal<bool>,
) -> impl IntoView {
    let stars = "\
<circle cx=\"5%\" cy=\"10%\" r=\"1px\" fill=\"white\" opacity=\"0.9\" style=\"--base-opacity: 0.9; animation: star-twinkle 3.1s ease-in-out infinite;\"/>\
<circle cx=\"12%\" cy=\"25%\" r=\"1.5px\" fill=\"white\" opacity=\"0.7\" style=\"--base-opacity: 0.7; animation: star-twinkle 4.2s ease-in-out infinite 0.5s;\"/>\
<circle cx=\"8%\" cy=\"40%\" r=\"1px\" fill=\"white\" opacity=\"1\" style=\"--base-opacity: 1; animation: star-twinkle 2.8s ease-in-out infinite 1s;\"/>\
<circle cx=\"15%\" cy=\"55%\" r=\"0.5px\" fill=\"white\" opacity=\"0.8\" style=\"--base-opacity: 0.8; animation: star-twinkle 3.7s ease-in-out infinite 0.3s;\"/>\
<circle cx=\"3%\" cy=\"70%\" r=\"1px\" fill=\"white\" opacity=\"0.6\" style=\"--base-opacity: 0.6; animation: star-twinkle 5s ease-in-out infinite;\"/>\
<circle cx=\"18%\" cy=\"80%\" r=\"1.5px\" fill=\"white\" opacity=\"0.9\" style=\"--base-opacity: 0.9; animation: star-twinkle 3.3s ease-in-out infinite 0.7s;\"/>\
<circle cx=\"22%\" cy=\"8%\" r=\"1px\" fill=\"white\" opacity=\"0.8\" style=\"--base-opacity: 0.8; animation: star-twinkle 4.5s ease-in-out infinite 0.2s;\"/>\
<circle cx=\"28%\" cy=\"18%\" r=\"0.5px\" fill=\"white\" opacity=\"1\" style=\"--base-opacity: 1; animation: star-twinkle 2.5s ease-in-out infinite 1.2s;\"/>\
<circle cx=\"25%\" cy=\"35%\" r=\"1.5px\" fill=\"white\" opacity=\"0.7\" style=\"--base-opacity: 0.7; animation: star-twinkle 3.9s ease-in-out infinite;\"/>\
<circle cx=\"30%\" cy=\"50%\" r=\"1px\" fill=\"white\" opacity=\"0.9\" style=\"--base-opacity: 0.9; animation: star-twinkle 4.1s ease-in-out infinite 0.6s;\"/>\
<circle cx=\"20%\" cy=\"65%\" r=\"1px\" fill=\"white\" opacity=\"0.8\" style=\"--base-opacity: 0.8; animation: star-twinkle 3.4s ease-in-out infinite 0.9s;\"/>\
<circle cx=\"32%\" cy=\"75%\" r=\"0.5px\" fill=\"white\" opacity=\"0.6\" style=\"--base-opacity: 0.6; animation: star-twinkle 4.8s ease-in-out infinite;\"/>\
<circle cx=\"35%\" cy=\"5%\" r=\"1px\" fill=\"white\" opacity=\"1\" style=\"--base-opacity: 1; animation: star-twinkle 2.9s ease-in-out infinite 0.4s;\"/>\
<circle cx=\"40%\" cy=\"15%\" r=\"1.5px\" fill=\"white\" opacity=\"0.7\" style=\"--base-opacity: 0.7; animation: star-twinkle 3.6s ease-in-out infinite 1.1s;\"/>\
<circle cx=\"38%\" cy=\"30%\" r=\"0.5px\" fill=\"white\" opacity=\"0.9\" style=\"--base-opacity: 0.9; animation: star-twinkle 4.3s ease-in-out infinite;\"/>\
<circle cx=\"42%\" cy=\"45%\" r=\"1px\" fill=\"white\" opacity=\"0.8\" style=\"--base-opacity: 0.8; animation: star-twinkle 3.2s ease-in-out infinite 0.8s;\"/>\
<circle cx=\"36%\" cy=\"60%\" r=\"1px\" fill=\"white\" opacity=\"0.6\" style=\"--base-opacity: 0.6; animation: star-twinkle 5.1s ease-in-out infinite 0.1s;\"/>\
<circle cx=\"45%\" cy=\"85%\" r=\"1.5px\" fill=\"white\" opacity=\"1\" style=\"--base-opacity: 1; animation: star-twinkle 2.7s ease-in-out infinite 1.3s;\"/>\
<circle cx=\"50%\" cy=\"12%\" r=\"1px\" fill=\"white\" opacity=\"0.7\" style=\"--base-opacity: 0.7; animation: star-twinkle 4s ease-in-out infinite;\"/>\
<circle cx=\"55%\" cy=\"28%\" r=\"0.5px\" fill=\"white\" opacity=\"0.9\" style=\"--base-opacity: 0.9; animation: star-twinkle 3.5s ease-in-out infinite 0.55s;\"/>\
<circle cx=\"52%\" cy=\"42%\" r=\"1.5px\" fill=\"white\" opacity=\"0.8\" style=\"--base-opacity: 0.8; animation: star-twinkle 4.4s ease-in-out infinite;\"/>\
<circle cx=\"58%\" cy=\"55%\" r=\"1px\" fill=\"white\" opacity=\"0.6\" style=\"--base-opacity: 0.6; animation: star-twinkle 4.7s ease-in-out infinite 0.35s;\"/>\
<circle cx=\"48%\" cy=\"70%\" r=\"1px\" fill=\"white\" opacity=\"1\" style=\"--base-opacity: 1; animation: star-twinkle 3s ease-in-out infinite 0.75s;\"/>\
<circle cx=\"60%\" cy=\"78%\" r=\"0.5px\" fill=\"white\" opacity=\"0.7\" style=\"--base-opacity: 0.7; animation: star-twinkle 4.9s ease-in-out infinite;\"/>\
<circle cx=\"65%\" cy=\"8%\" r=\"1px\" fill=\"white\" opacity=\"0.9\" style=\"--base-opacity: 0.9; animation: star-twinkle 3.8s ease-in-out infinite 0.25s;\"/>\
<circle cx=\"70%\" cy=\"22%\" r=\"1.5px\" fill=\"white\" opacity=\"0.8\" style=\"--base-opacity: 0.8; animation: star-twinkle 2.6s ease-in-out infinite 1.05s;\"/>\
<circle cx=\"68%\" cy=\"38%\" r=\"0.5px\" fill=\"white\" opacity=\"0.6\" style=\"--base-opacity: 0.6; animation: star-twinkle 5.2s ease-in-out infinite;\"/>\
<circle cx=\"72%\" cy=\"52%\" r=\"1px\" fill=\"white\" opacity=\"1\" style=\"--base-opacity: 1; animation: star-twinkle 3.3s ease-in-out infinite 0.45s;\"/>\
<circle cx=\"66%\" cy=\"68%\" r=\"1px\" fill=\"white\" opacity=\"0.7\" style=\"--base-opacity: 0.7; animation: star-twinkle 4.2s ease-in-out infinite;\"/>\
<circle cx=\"75%\" cy=\"82%\" r=\"1.5px\" fill=\"white\" opacity=\"0.9\" style=\"--base-opacity: 0.9; animation: star-twinkle 2.8s ease-in-out infinite 0.85s;\"/>\
<circle cx=\"78%\" cy=\"15%\" r=\"1px\" fill=\"white\" opacity=\"0.8\" style=\"--base-opacity: 0.8; animation: star-twinkle 4.6s ease-in-out infinite;\"/>\
<circle cx=\"82%\" cy=\"30%\" r=\"0.5px\" fill=\"white\" opacity=\"0.6\" style=\"--base-opacity: 0.6; animation: star-twinkle 3.9s ease-in-out infinite 0.15s;\"/>\
<circle cx=\"80%\" cy=\"48%\" r=\"1.5px\" fill=\"white\" opacity=\"1\" style=\"--base-opacity: 1; animation: star-twinkle 2.4s ease-in-out infinite 0.65s;\"/>\
<circle cx=\"85%\" cy=\"60%\" r=\"1px\" fill=\"white\" opacity=\"0.7\" style=\"--base-opacity: 0.7; animation: star-twinkle 4.1s ease-in-out infinite;\"/>\
<circle cx=\"76%\" cy=\"75%\" r=\"1px\" fill=\"white\" opacity=\"0.9\" style=\"--base-opacity: 0.9; animation: star-twinkle 3.7s ease-in-out infinite 0.95s;\"/>\
<circle cx=\"88%\" cy=\"85%\" r=\"0.5px\" fill=\"white\" opacity=\"0.8\" style=\"--base-opacity: 0.8; animation: star-twinkle 5s ease-in-out infinite;\"/>\
<circle cx=\"90%\" cy=\"5%\" r=\"1px\" fill=\"white\" opacity=\"0.6\" style=\"--base-opacity: 0.6; animation: star-twinkle 3.4s ease-in-out infinite 0.5s;\"/>\
<circle cx=\"95%\" cy=\"20%\" r=\"1.5px\" fill=\"white\" opacity=\"1\" style=\"--base-opacity: 1; animation: star-twinkle 2.9s ease-in-out infinite 0.2s;\"/>\
<circle cx=\"92%\" cy=\"35%\" r=\"0.5px\" fill=\"white\" opacity=\"0.7\" style=\"--base-opacity: 0.7; animation: star-twinkle 4.5s ease-in-out infinite;\"/>\
<circle cx=\"97%\" cy=\"50%\" r=\"1px\" fill=\"white\" opacity=\"0.9\" style=\"--base-opacity: 0.9; animation: star-twinkle 3.1s ease-in-out infinite 0.7s;\"/>\
<circle cx=\"94%\" cy=\"65%\" r=\"1px\" fill=\"white\" opacity=\"0.8\" style=\"--base-opacity: 0.8; animation: star-twinkle 4.8s ease-in-out infinite;\"/>\
<circle cx=\"98%\" cy=\"80%\" r=\"1.5px\" fill=\"white\" opacity=\"0.6\" style=\"--base-opacity: 0.6; animation: star-twinkle 3.6s ease-in-out infinite 0.35s;\"/>\
<circle cx=\"10%\" cy=\"90%\" r=\"1px\" fill=\"white\" opacity=\"1\" style=\"--base-opacity: 1; animation: star-twinkle 2.7s ease-in-out infinite 0.55s;\"/>\
<circle cx=\"25%\" cy=\"95%\" r=\"0.5px\" fill=\"white\" opacity=\"0.7\" style=\"--base-opacity: 0.7; animation: star-twinkle 4.3s ease-in-out infinite;\"/>\
<circle cx=\"40%\" cy=\"92%\" r=\"1.5px\" fill=\"white\" opacity=\"0.9\" style=\"--base-opacity: 0.9; animation: star-twinkle 3.2s ease-in-out infinite 0.15s;\"/>\
<circle cx=\"55%\" cy=\"98%\" r=\"1px\" fill=\"white\" opacity=\"0.8\" style=\"--base-opacity: 0.8; animation: star-twinkle 5.3s ease-in-out infinite;\"/>\
<circle cx=\"70%\" cy=\"93%\" r=\"1px\" fill=\"white\" opacity=\"0.6\" style=\"--base-opacity: 0.6; animation: star-twinkle 4s ease-in-out infinite 0.85s;\"/>\
<circle cx=\"85%\" cy=\"95%\" r=\"0.5px\" fill=\"white\" opacity=\"1\" style=\"--base-opacity: 1; animation: star-twinkle 2.5s ease-in-out infinite 0.4s;\"/>\
<circle cx=\"7%\" cy=\"17%\" r=\"0.5px\" fill=\"white\" opacity=\"0.5\" style=\"--base-opacity: 0.5; animation: star-twinkle 6s ease-in-out infinite 1s;\"/>\
<circle cx=\"17%\" cy=\"42%\" r=\"0.5px\" fill=\"white\" opacity=\"0.4\" style=\"--base-opacity: 0.4; animation: star-twinkle 5.5s ease-in-out infinite 0.3s;\"/>\
<circle cx=\"33%\" cy=\"3%\" r=\"0.5px\" fill=\"white\" opacity=\"0.5\" style=\"--base-opacity: 0.5; animation: star-twinkle 6.5s ease-in-out infinite 0.8s;\"/>\
<circle cx=\"43%\" cy=\"58%\" r=\"0.5px\" fill=\"white\" opacity=\"0.4\" style=\"--base-opacity: 0.4; animation: star-twinkle 7s ease-in-out infinite;\"/>\
<circle cx=\"57%\" cy=\"63%\" r=\"0.5px\" fill=\"white\" opacity=\"0.5\" style=\"--base-opacity: 0.5; animation: star-twinkle 5.8s ease-in-out infinite 0.6s;\"/>\
<circle cx=\"73%\" cy=\"7%\" r=\"0.5px\" fill=\"white\" opacity=\"0.4\" style=\"--base-opacity: 0.4; animation: star-twinkle 6.2s ease-in-out infinite 1.2s;\"/>\
<circle cx=\"83%\" cy=\"73%\" r=\"0.5px\" fill=\"white\" opacity=\"0.5\" style=\"--base-opacity: 0.5; animation: star-twinkle 5.4s ease-in-out infinite;\"/>\
<circle cx=\"93%\" cy=\"43%\" r=\"0.5px\" fill=\"white\" opacity=\"0.4\" style=\"--base-opacity: 0.4; animation: star-twinkle 7.2s ease-in-out infinite 0.45s;\"/>";

    let asteroid_belt = "\
<circle cx=\"50%\" cy=\"50%\" r=\"30%\" fill=\"none\" stroke=\"#555\" stroke-width=\"1%\" opacity=\"0.05\"/>\
<circle cx=\"50%\" cy=\"50%\" r=\"31%\" fill=\"none\" stroke=\"#666\" stroke-width=\"1.5%\" opacity=\"0.08\"/>\
<circle cx=\"50%\" cy=\"50%\" r=\"32%\" fill=\"none\" stroke=\"#777\" stroke-width=\"2%\" opacity=\"0.1\"/>\
<circle cx=\"50%\" cy=\"50%\" r=\"33%\" fill=\"none\" stroke=\"#666\" stroke-width=\"2.5%\" opacity=\"0.12\"/>\
<circle cx=\"50%\" cy=\"50%\" r=\"34%\" fill=\"none\" stroke=\"#555\" stroke-width=\"3%\" opacity=\"0.08\"/>\
<circle cx=\"52%\" cy=\"46%\" r=\"1px\" fill=\"#888\" opacity=\"0.6\"/>\
<circle cx=\"48%\" cy=\"53%\" r=\"1.5px\" fill=\"#777\" opacity=\"0.5\"/>\
<circle cx=\"55%\" cy=\"48%\" r=\"0.5px\" fill=\"#999\" opacity=\"0.7\"/>\
<circle cx=\"51%\" cy=\"55%\" r=\"2px\" fill=\"#666\" opacity=\"0.4\"/>\
<circle cx=\"46%\" cy=\"47%\" r=\"1px\" fill=\"#888\" opacity=\"0.55\"/>\
<circle cx=\"54%\" cy=\"43%\" r=\"1.5px\" fill=\"#777\" opacity=\"0.6\"/>\
<circle cx=\"49%\" cy=\"57%\" r=\"0.5px\" fill=\"#999\" opacity=\"0.5\"/>\
<circle cx=\"56%\" cy=\"52%\" r=\"1px\" fill=\"#666\" opacity=\"0.65\"/>\
<circle cx=\"47%\" cy=\"51%\" r=\"2px\" fill=\"#888\" opacity=\"0.45\"/>\
<circle cx=\"53%\" cy=\"45%\" r=\"0.5px\" fill=\"#777\" opacity=\"0.55\"/>\
<circle cx=\"50%\" cy=\"49%\" r=\"1.5px\" fill=\"#999\" opacity=\"0.5\"/>\
<circle cx=\"44%\" cy=\"50%\" r=\"1px\" fill=\"#666\" opacity=\"0.4\"/>\
<circle cx=\"57%\" cy=\"49%\" r=\"0.5px\" fill=\"#888\" opacity=\"0.6\"/>\
<circle cx=\"48%\" cy=\"44%\" r=\"2px\" fill=\"#777\" opacity=\"0.5\"/>\
<circle cx=\"52%\" cy=\"56%\" r=\"1px\" fill=\"#999\" opacity=\"0.45\"/>\
<circle cx=\"45%\" cy=\"54%\" r=\"1.5px\" fill=\"#666\" opacity=\"0.55\"/>\
<circle cx=\"55%\" cy=\"54%\" r=\"0.5px\" fill=\"#888\" opacity=\"0.4\"/>\
<circle cx=\"51%\" cy=\"43%\" r=\"1px\" fill=\"#777\" opacity=\"0.6\"/>\
<circle cx=\"49%\" cy=\"58%\" r=\"1.5px\" fill=\"#999\" opacity=\"0.35\"/>\
<circle cx=\"53%\" cy=\"57%\" r=\"0.5px\" fill=\"#666\" opacity=\"0.5\"/>\
<circle cx=\"47%\" cy=\"44%\" r=\"1px\" fill=\"#888\" opacity=\"0.45\"/>\
<circle cx=\"54%\" cy=\"47%\" r=\"2px\" fill=\"#777\" opacity=\"0.4\"/>\
<circle cx=\"48%\" cy=\"57%\" r=\"0.5px\" fill=\"#999\" opacity=\"0.55\"/>\
<circle cx=\"52%\" cy=\"52%\" r=\"1px\" fill=\"#666\" opacity=\"0.5\"/>\
<circle cx=\"46%\" cy=\"52%\" r=\"1.5px\" fill=\"#888\" opacity=\"0.35\"/>\
<circle cx=\"55%\" cy=\"51%\" r=\"0.5px\" fill=\"#777\" opacity=\"0.45\"/>\
<circle cx=\"50%\" cy=\"46%\" r=\"1px\" fill=\"#999\" opacity=\"0.5\"/>\
<circle cx=\"50%\" cy=\"55%\" r=\"0.5px\" fill=\"#666\" opacity=\"0.4\"/>";

    view! {
        {/* Star field background */}
        <svg class="absolute inset-0 w-full h-full pointer-events-none" style="z-index: 0;" inner_html={stars}></svg>

        {/* Orbital path rings */}
        <svg class="absolute inset-0 w-full h-full pointer-events-none" style="z-index: 1;">
            <circle cx="50%%" cy="50%%" r="8%%" fill="none" stroke="#9ca3af" stroke-width="0.3" stroke-dasharray="1,3" opacity="0.3"/>
            <circle cx="50%%" cy="50%%" r="14%%" fill="none" stroke="#fbbf24" stroke-width="0.3" stroke-dasharray="1,3" opacity="0.25"/>
            <circle cx="50%%" cy="50%%" r="20%%" fill="none" stroke="#3b82f6" stroke-width="0.3" stroke-dasharray="1,3" opacity="0.25"/>
            <circle cx="50%%" cy="50%%" r="27%%" fill="none" stroke="#ef4444" stroke-width="0.3" stroke-dasharray="1,3" opacity="0.25"/>
            <circle cx="50%%" cy="50%%" r="38%%" fill="none" stroke="#f97316" stroke-width="0.3" stroke-dasharray="1,3" opacity="0.25"/>
            <circle cx="50%%" cy="50%%" r="50%%" fill="none" stroke="#eab308" stroke-width="0.3" stroke-dasharray="1,3" opacity="0.25"/>
            <circle cx="50%%" cy="50%%" r="62%%" fill="none" stroke="#06b6d4" stroke-width="0.3" stroke-dasharray="1,3" opacity="0.25"/>
            <circle cx="50%%" cy="50%%" r="75%%" fill="none" stroke="#6366f1" stroke-width="0.3" stroke-dasharray="1,3" opacity="0.25"/>
        </svg>

        {/* Sun */}
        <div class="absolute pointer-events-none" style="left: 50%%; top: 50%%; transform: translate(-50%%, -50%%); z-index: 5;">
            <div style="position: absolute; left: 50%%; top: 50%%; transform: translate(-50%%, -50%%); width: 100px; height: 100px; border-radius: 50%%; background: radial-gradient(circle, transparent 35%%, rgba(255,200,50,0.04) 50%%, transparent 65%%); animation: corona-pulse 4s ease-in-out infinite;"></div>
            <div style="position: absolute; left: 50%%; top: 50%%; transform: translate(-50%%, -50%%); width: 50px; height: 50px; border-radius: 50%%; background: radial-gradient(circle, #ffc83280 0%%, #ff960040 40%%, transparent 70%%); animation: corona-pulse 3s ease-in-out infinite;"></div>
            <div style="position: absolute; left: 50%%; top: 50%%; transform: translate(-50%%, -50%%); width: 24px; height: 24px; border-radius: 50%%; background: radial-gradient(circle, #fff 0%%, #ffd700 35%%, #ff8c00 70%%, #ff6600 100%%); animation: sun-pulse 2.5s ease-in-out infinite; box-shadow: 0 0 15px #ff8c00, 0 0 30px #ffa500, 0 0 50px #ffa50080;"></div>
        </div>

        {/* Mercury */}
        <div class="absolute pointer-events-none" style=move || {
            let porbits = planet_angles.get();
            let angle = PLANET_INITIAL_ANGLES[0] + porbits[0];
            let r = PLANET_DATA[0].0;
            let x = 50.0 + r * angle.cos();
            let y = 50.0 + r * angle.sin();
            format!("left: {}%%; top: {}%%; transform: translate(-50%%, -50%%); z-index: 10;", x, y)
        }>
            <div style=move || {
                let porbits = planet_angles.get();
                let angle = PLANET_INITIAL_ANGLES[0] + porbits[0];
                let lx = 50.0 + 25.0 * angle.cos();
                let ly = 50.0 - 25.0 * angle.sin();
                let lxc = lx.max(15.0).min(65.0);
                let lyc = ly.max(15.0).min(65.0);
                let is_selected = target_planet_idx.get() == Some(0);
                let glow = if is_selected { format!("position: absolute; left: 50%%; top: 50%%; transform: translate(-50%%, -50%%); width: {}px; height: {}px; border-radius: 50%%; color: #9ca3af; animation: glow-pulse 1.5s ease-in-out infinite; z-index: -1;", PLANET_DATA[0].1 + 6.0, PLANET_DATA[0].1 + 6.0) } else { String::new() };
                format!("{}{}", glow, format!("width: {}px; height: {}px; background: radial-gradient(circle at {}%% {}%%, #d1d5db, #6b7280 60%%, #374151); border-radius: 50%%; box-shadow: 0 0 3px #9ca3af, inset -1px -1px 2px #00000066;", PLANET_DATA[0].1, PLANET_DATA[0].1, lxc, lyc))
            }></div>
        </div>

        {/* Venus */}
        <div class="absolute pointer-events-none" style=move || {
            let porbits = planet_angles.get();
            let angle = PLANET_INITIAL_ANGLES[1] + porbits[1];
            let r = PLANET_DATA[1].0;
            let x = 50.0 + r * angle.cos();
            let y = 50.0 + r * angle.sin();
            format!("left: {}%%; top: {}%%; transform: translate(-50%%, -50%%); z-index: 11;", x, y)
        }>
            <div style=move || {
                let porbits = planet_angles.get();
                let angle = PLANET_INITIAL_ANGLES[1] + porbits[1];
                let lx = 50.0 + 25.0 * angle.cos();
                let ly = 50.0 - 25.0 * angle.sin();
                let lxc = lx.max(15.0).min(65.0);
                let lyc = ly.max(15.0).min(65.0);
                let is_selected = target_planet_idx.get() == Some(1);
                let glow = if is_selected { format!("position: absolute; left: 50%%; top: 50%%; transform: translate(-50%%, -50%%); width: {}px; height: {}px; border-radius: 50%%; color: #fbbf24; animation: glow-pulse 1.5s ease-in-out infinite; z-index: -1;", PLANET_DATA[1].1 + 6.0, PLANET_DATA[1].1 + 6.0) } else { String::new() };
                format!("{}{}", glow, format!("width: {}px; height: {}px; background: radial-gradient(circle at {}%% {}%%, #fef3c7, #fbbf24 50%%, #d97706); border-radius: 50%%; box-shadow: 0 0 6px #fbbf24cc, inset -1px -1px 3px #0000004c;", PLANET_DATA[1].1, PLANET_DATA[1].1, lxc, lyc))
            }></div>
        </div>

        {/* Earth - Feature 6: cloud patches */}
        <div class="absolute pointer-events-none" style=move || {
            let porbits = planet_angles.get();
            let angle = PLANET_INITIAL_ANGLES[2] + porbits[2];
            let r = PLANET_DATA[2].0;
            let x = 50.0 + r * angle.cos();
            let y = 50.0 + r * angle.sin();
            format!("left: {}%%; top: {}%%; transform: translate(-50%%, -50%%); z-index: 12;", x, y)
        }>
            <div style=move || {
                let porbits = planet_angles.get();
                let angle = PLANET_INITIAL_ANGLES[2] + porbits[2];
                let lx = 50.0 + 25.0 * angle.cos();
                let ly = 50.0 - 25.0 * angle.sin();
                let lxc = lx.max(15.0).min(65.0);
                let lyc = ly.max(15.0).min(65.0);
                let is_selected = target_planet_idx.get() == Some(2);
                let glow = if is_selected { format!("position: absolute; left: 50%%; top: 50%%; transform: translate(-50%%, -50%%); width: {}px; height: {}px; border-radius: 50%%; color: #3b82f6; animation: glow-pulse 1.5s ease-in-out infinite; z-index: -1;", PLANET_DATA[2].1 + 6.0, PLANET_DATA[2].1 + 6.0) } else { String::new() };
                format!("{}{}", glow, format!("width: {}px; height: {}px; background: radial-gradient(circle at {}%% {}%%, #93c5fd, #3b82f6 50%%, #1e3a8a); border-radius: 50%%; box-shadow: 0 0 8px #3b82f6e5, 0 0 16px rgba(59,130,246,0.3), inset -1px -1px 3px #0000004c; position: relative;", PLANET_DATA[2].1, PLANET_DATA[2].1, lxc, lyc))
            }></div>
            {/* Feature 6: Earth cloud layer */}
            <div style="position: absolute; left: 0%%; top: 0%%; width: 100%%; height: 100%%; border-radius: 50%%; background: repeating-linear-gradient(45deg, transparent 0%%, transparent 20%%, rgba(255,255,255,0.15) 20%%, rgba(255,255,255,0.15) 25%%, transparent 25%%, transparent 50%%, rgba(255,255,255,0.1) 50%%, rgba(255,255,255,0.1) 55%%); animation: earth-clouds 20s linear infinite; pointer-events: none; overflow: hidden;"></div>
        </div>

        {/* Mars */}
        <div class="absolute pointer-events-none" style=move || {
            let porbits = planet_angles.get();
            let angle = PLANET_INITIAL_ANGLES[3] + porbits[3];
            let r = PLANET_DATA[3].0;
            let x = 50.0 + r * angle.cos();
            let y = 50.0 + r * angle.sin();
            format!("left: {}%%; top: {}%%; transform: translate(-50%%, -50%%); z-index: 13;", x, y)
        }>
            <div style=move || {
                let porbits = planet_angles.get();
                let angle = PLANET_INITIAL_ANGLES[3] + porbits[3];
                let lx = 50.0 + 25.0 * angle.cos();
                let ly = 50.0 - 25.0 * angle.sin();
                let lxc = lx.max(15.0).min(65.0);
                let lyc = ly.max(15.0).min(65.0);
                let is_selected = target_planet_idx.get() == Some(3);
                let glow = if is_selected { format!("position: absolute; left: 50%%; top: 50%%; transform: translate(-50%%, -50%%); width: {}px; height: {}px; border-radius: 50%%; color: #ef4444; animation: glow-pulse 1.5s ease-in-out infinite; z-index: -1;", PLANET_DATA[3].1 + 6.0, PLANET_DATA[3].1 + 6.0) } else { String::new() };
                format!("{}{}", glow, format!("width: {}px; height: {}px; background: radial-gradient(circle at {}%% {}%%, #fca5a5, #ef4444 50%%, #7f1d1d); border-radius: 50%%; box-shadow: 0 0 5px #ef4444cc, inset -1px -1px 2px #0000004c;", PLANET_DATA[3].1, PLANET_DATA[3].1, lxc, lyc))
            }></div>
        </div>

        {/* Feature 5: Asteroid Belt - depth */}
        <svg class="absolute inset-0 w-full h-full pointer-events-none" style="z-index: 5;" inner_html={asteroid_belt}></svg>

        {/* Jupiter */}
        <div class="absolute pointer-events-none" style=move || {
            let porbits = planet_angles.get();
            let angle = PLANET_INITIAL_ANGLES[4] + porbits[4];
            let r = PLANET_DATA[4].0;
            let x = 50.0 + r * angle.cos();
            let y = 50.0 + r * angle.sin();
            format!("left: {}%%; top: {}%%; transform: translate(-50%%, -50%%); z-index: 14;", x, y)
        }>
            <div style=move || {
                let porbits = planet_angles.get();
                let angle = PLANET_INITIAL_ANGLES[4] + porbits[4];
                let lx = 50.0 + 25.0 * angle.cos();
                let ly = 50.0 - 25.0 * angle.sin();
                let lxc = lx.max(15.0).min(65.0);
                let lyc = ly.max(15.0).min(65.0);
                let is_selected = target_planet_idx.get() == Some(4);
                let glow = if is_selected { format!("position: absolute; left: 50%%; top: 50%%; transform: translate(-50%%, -50%%); width: {}px; height: {}px; border-radius: 50%%; color: #f97316; animation: glow-pulse 1.5s ease-in-out infinite; z-index: -1;", PLANET_DATA[4].1 + 6.0, PLANET_DATA[4].1 + 6.0) } else { String::new() };
                format!("{}{}", glow, format!("width: {}px; height: {}px; background: radial-gradient(circle at {}%% {}%%, #fcd34d, #f97316 50%%, #c2410c); border-radius: 50%%; box-shadow: 0 0 10px #f97316cc, inset -2px -2px 4px #0000004c; position: relative;", PLANET_DATA[4].1, PLANET_DATA[4].1, lxc, lyc))
            }></div>
            <div class="jupiter-bands"></div>
            <div style=format!("position: absolute; left: 55%%; top: 60%%; width: {}px; height: {}px; background: radial-gradient(ellipse, #ef4444, #b91c1c, transparent); border-radius: 50%%; opacity: 0.7;", PLANET_DATA[4].1 * 0.22, PLANET_DATA[4].1 * 0.14)></div>
        </div>

        {/* Saturn */}
        <div class="absolute pointer-events-none" style=move || {
            let porbits = planet_angles.get();
            let angle = PLANET_INITIAL_ANGLES[5] + porbits[5];
            let r = PLANET_DATA[5].0;
            let x = 50.0 + r * angle.cos();
            let y = 50.0 + r * angle.sin();
            format!("left: {}%%; top: {}%%; transform: translate(-50%%, -50%%); z-index: 15;", x, y)
        }>
            <div style=move || {
                let porbits = planet_angles.get();
                let angle = PLANET_INITIAL_ANGLES[5] + porbits[5];
                let lx = 50.0 + 25.0 * angle.cos();
                let ly = 50.0 - 25.0 * angle.sin();
                let lxc = lx.max(15.0).min(65.0);
                let lyc = ly.max(15.0).min(65.0);
                let is_selected = target_planet_idx.get() == Some(5);
                let glow = if is_selected { format!("position: absolute; left: 50%%; top: 50%%; transform: translate(-50%%, -50%%); width: {}px; height: {}px; border-radius: 50%%; color: #eab308; animation: glow-pulse 1.5s ease-in-out infinite; z-index: -1;", PLANET_DATA[5].1 + 6.0, PLANET_DATA[5].1 + 6.0) } else { String::new() };
                let size = PLANET_DATA[5].1;
                format!("{}{}", glow, format!("width: {}px; height: {}px; background: radial-gradient(circle at {}%% {}%%, #fef9c3, #eab308 50%%, #a16207); border-radius: 50%%; box-shadow: 0 0 8px #eab308b2, inset -1px -1px 3px #0000004c; position: relative;", size, size, lxc, lyc))
            }></div>
            {move || {
                let porbits = planet_angles.get();
                let angle = PLANET_INITIAL_ANGLES[5] + porbits[5];
                let size = PLANET_DATA[5].1;
                view! {
                    <div style=format!("position: absolute; left: 50%%; top: 50%%; transform: translate(-50%%, -50%%) rotate({}rad); width: {}px; height: {}px; border: 2px #d4a574; border-radius: 50%%; opacity: 0.3; box-shadow: 0 0 3px #eab3084c; pointer-events: none;", angle, size * 3.4, size * 0.85)></div>
                    <div style=format!("position: absolute; left: 50%%; top: 50%%; transform: translate(-50%%, -50%%) rotate({}rad); width: {}px; height: {}px; border: 2px #c0a060; border-radius: 50%%; opacity: 0.5; box-shadow: 0 0 5px #eab30866; pointer-events: none;", angle, size * 2.8, size * 0.75)></div>
                    <div style=format!("position: absolute; left: 50%%; top: 50%%; transform: translate(-50%%, -50%%) rotate({}rad); width: {}px; height: {}px; border: 2px #eab308; border-radius: 50%%; opacity: 0.7; box-shadow: 0 0 8px #eab3087f; pointer-events: none;", angle, size * 2.3, size * 0.65)></div>
                }
            }}
        </div>

        {/* Feature 1: Uranus - tilted cyan rings */}
        <div class="absolute pointer-events-none" style=move || {
            let porbits = planet_angles.get();
            let angle = PLANET_INITIAL_ANGLES[6] + porbits[6];
            let r = PLANET_DATA[6].0;
            let x = 50.0 + r * angle.cos();
            let y = 50.0 + r * angle.sin();
            format!("left: {}%%; top: {}%%; transform: translate(-50%%, -50%%); z-index: 16;", x, y)
        }>
            <div style=move || {
                let porbits = planet_angles.get();
                let angle = PLANET_INITIAL_ANGLES[6] + porbits[6];
                let lx = 50.0 + 25.0 * angle.cos();
                let ly = 50.0 - 25.0 * angle.sin();
                let lxc = lx.max(15.0).min(65.0);
                let lyc = ly.max(15.0).min(65.0);
                let is_selected = target_planet_idx.get() == Some(6);
                let glow = if is_selected { format!("position: absolute; left: 50%%; top: 50%%; transform: translate(-50%%, -50%%); width: {}px; height: {}px; border-radius: 50%%; color: #06b6d4; animation: glow-pulse 1.5s ease-in-out infinite; z-index: -1;", PLANET_DATA[6].1 + 6.0, PLANET_DATA[6].1 + 6.0) } else { String::new() };
                let size = PLANET_DATA[6].1;
                format!("{}{}", glow, format!("width: {}px; height: {}px; background: radial-gradient(circle at {}%% {}%%, #a5f3fc, #06b6d4 50%%, #0e7490); border-radius: 50%%; box-shadow: 0 0 6px #06b6d4cc, inset -1px -1px 3px #0000004c; position: relative;", size, size, lxc, lyc))
            }></div>
            {move || {
                let porbits = planet_angles.get();
                let angle = PLANET_INITIAL_ANGLES[6] + porbits[6];
                let size = PLANET_DATA[6].1;
                view! {
                    <div style=format!("position: absolute; left: 50%%; top: 50%%; transform: translate(-50%%, -50%%) rotateX(75deg) rotate({}rad); width: {}px; height: {}px; border: 1px #67e8f9; border-radius: 50%%; opacity: 0.2; box-shadow: 0 0 4px #67e8f940; pointer-events: none;", angle, size * 2.8, size * 0.7)></div>
                    <div style=format!("position: absolute; left: 50%%; top: 50%%; transform: translate(-50%%, -50%%) rotateX(75deg) rotate({}rad); width: {}px; height: {}px; border: 1px #67e8f9; border-radius: 50%%; opacity: 0.35; box-shadow: 0 0 6px #67e8f960; pointer-events: none;", angle, size * 2.3, size * 0.6)></div>
                }
            }}
        </div>

        {/* Neptune */}
        <div class="absolute pointer-events-none" style=move || {
            let porbits = planet_angles.get();
            let angle = PLANET_INITIAL_ANGLES[7] + porbits[7];
            let r = PLANET_DATA[7].0;
            let x = 50.0 + r * angle.cos();
            let y = 50.0 + r * angle.sin();
            format!("left: {}%%; top: {}%%; transform: translate(-50%%, -50%%); z-index: 17;", x, y)
        }>
            <div style=move || {
                let porbits = planet_angles.get();
                let angle = PLANET_INITIAL_ANGLES[7] + porbits[7];
                let lx = 50.0 + 25.0 * angle.cos();
                let ly = 50.0 - 25.0 * angle.sin();
                let lxc = lx.max(15.0).min(65.0);
                let lyc = ly.max(15.0).min(65.0);
                let is_selected = target_planet_idx.get() == Some(7);
                let glow = if is_selected { format!("position: absolute; left: 50%%; top: 50%%; transform: translate(-50%%, -50%%); width: {}px; height: {}px; border-radius: 50%%; color: #6366f1; animation: glow-pulse 1.5s ease-in-out infinite; z-index: -1;", PLANET_DATA[7].1 + 6.0, PLANET_DATA[7].1 + 6.0) } else { String::new() };
                format!("{}{}", glow, format!("width: {}px; height: {}px; background: radial-gradient(circle at {}%% {}%%, #a5b4fc, #6366f1 50%%, #4338ca); border-radius: 50%%; box-shadow: 0 0 8px #6366f1cc, inset -1px -1px 3px #0000004c;", PLANET_DATA[7].1, PLANET_DATA[7].1, lxc, lyc))
            }></div>
        </div>

        {/* Feature 3: Ship engine trail */}
        <svg class="absolute inset-0 w-full h-full pointer-events-none" style="z-index: 19;">
            {move || {
                let trail = trail_positions.get();
                let mut circles = String::new();
                for &(x, y, age) in trail.iter() {
                    let opacity = (1.0 - age).max(0.0);
                    let size = 3.0 * (1.0 - age * 0.5);
                    if opacity > 0.0 && size > 0.0 {
                        circles.push_str(&format!("<circle cx=\"{}\" cy=\"{}\" r=\"{}\" fill=\"#f97316\" opacity=\"{}\"/>", x, y, size, opacity));
                    }
                }
                view! { <g inner_html={circles}></g> }
            }}
        </svg>

        {/* Flight path (actual trajectory) - glowing energy trail */}
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
                
                // Dashed line showing actual path
                let path_line = if path.len() > 1 {
                    format!("<path d=\"{}\" fill=\"none\" stroke=\"#f97316\" stroke-width=\"1.5\" stroke-dasharray=\"3,2\" opacity=\"0.7\"/>", path_d)
                } else { String::new() };
                
                view! { <g inner_html={format!("{}{}", path_line, dots)}></g> }
            }}
        </svg>

        {/* Flight prediction (planned curved path) */}
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

        {/* Gravity wells visualization during flight */}
        <svg class="absolute inset-0 w-full h-full pointer-events-none" style="z-index: 6;">
            {move || {
                let flying = is_flying.get();
                if !flying { return view! { <g></g> }; }
                
                let porbits = planet_angles.get();
                let target_idx = target_planet_idx.get().unwrap_or(0);
                
                let mut wells = String::new();
                for (pidx, pdata) in PLANET_DATA.iter().enumerate() {
                    if pidx == target_idx { continue; } // Skip target planet
                    
                    let pangle = PLANET_INITIAL_ANGLES[pidx] + porbits[pidx];
                    let px = 50.0 + pdata.0 * pangle.cos();
                    let py = 50.0 + pdata.0 * pangle.sin();
                    let mass = pdata.1;
                    
                    // Draw concentric circles for gravity influence
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

        {/* Snap into orbit animation */}
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

        {/* Feature 2: Moons with sun lighting */}
        <svg class="absolute inset-0 w-full h-full pointer-events-none" style="z-index: 18;">
            <defs>
                {move || {
                    let porbits = planet_angles.get();
                    let mangles = moon_angles.get();
                    let mut defs = String::new();
                    let mut moon_idx = 0;
                    for (pidx, moons) in MOON_DATA.iter().enumerate() {
                        let planet_angle = PLANET_INITIAL_ANGLES[pidx] + porbits[pidx];
                        let planet_orbit_r = PLANET_DATA[pidx].0;
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
                            let moon_color = MOON_DATA[pidx][moon_idx].3;
                            defs.push_str(&format!("<radialGradient id=\"moon-lit-{}\" cx=\"{}\" cy=\"{}\" r=\"50%\"><stop offset=\"0%\" stop-color=\"#fffaf0\"/><stop offset=\"40%\" stop-color=\"{}\"/><stop offset=\"100%\" stop-color=\"{}\"/></radialGradient>",
                                moon_idx, hl_cx_c, hl_cy_c, moon_color, moon_color));
                            moon_idx += 1;
                        }
                    }
                    view! { <g inner_html={defs}></g> }
                }}
            </defs>
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
                        let (_, moon_orbit_r, moon_size, _) = *moon;
                        let angle = mangles[moon_idx];
                        let moon_x = planet_x + moon_orbit_r * angle.cos();
                        let moon_y = planet_y + moon_orbit_r * angle.sin();
                        paths.push_str(&format!("<circle cx=\"{}\" cy=\"{}\" r=\"{}\" fill=\"url(#moon-lit-{})\"/>", moon_x, moon_y, moon_size, moon_idx));
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
            format!("left: {}%%; top: {}%%; transform: translate(-50%%, -50%%) rotate({}rad); z-index: 30;", world_x, world_y, facing)
        }>
            <div style="position: absolute; left: 50%%; top: 50%%; width: 0; height: 0; border-left: 2px solid transparent; border-right: 2px solid transparent; border-top: 8px solid #f97316; filter: blur(1px); opacity: 0.6; transform: translate(-50%%, 2px); animation: engine-glow 0.3s ease-in-out infinite;"></div>
            <div style="width: 0; height: 0; border-left: 3.5px solid transparent; border-right: 3.5px solid transparent; border-bottom: 7px solid #ef4444; filter: drop-shadow(0 0 4px #ef4444);"></div>
            <div style="position: absolute; left: 50%%; top: 2px; transform: translateX(-50%%); width: 3px; height: 3px; background: #fef08a; border-radius: 50%%; box-shadow: 0 0 3px #fef08a;"></div>
        </div>

        {/* Upgrade effects */}
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
