//! Ship rendering with engine glow and cockpit
//! 
//! Props: spaceship_angle, target_planet_idx, is_flying, planet_angles,
//! planet_offset, fly_x, fly_y, fly_from_x, fly_from_y
//! 
//! Renders ship at world position with engine glow + cockpit.

use leptos::*;

use crate::constants::*;

#[component]
pub fn Ship(
    spaceship_angle: ReadSignal<f64>,
    target_planet_idx: ReadSignal<Option<usize>>,
    is_flying: ReadSignal<bool>,
    planet_angles: ReadSignal<Vec<f64>>,
    planet_offset: ReadSignal<f64>,
    fly_x: ReadSignal<f64>,
    fly_y: ReadSignal<f64>,
    fly_from_x: ReadSignal<f64>,
    fly_from_y: ReadSignal<f64>,
) -> impl IntoView {
    view! {
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
    }
}
