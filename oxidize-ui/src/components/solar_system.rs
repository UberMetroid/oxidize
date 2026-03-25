//! Main SolarSystem component — orchestrates all child components.

use leptos::*;
use crate::constants::*;
use crate::types::UpgradeEffect;
use oxidize_engine::UpgradeType;

use super::asteroid_belt::AsteroidBelt;
use super::dyson_effect::DysonEffect;
use super::flight_path::FlightPath;
use super::gravity_wells::GravityWells;
use super::jupiter_bands::JupiterBands;
use super::moons::Moons;
use super::orbital_paths::OrbitalPaths;
use super::planets::Planets;
use super::saturn_rings::SaturnRings;
use super::ship::Ship;
use super::ship_trail::ShipTrail;
use super::starfield::Starfield;
use super::stellar_effect::StellarEffect;
use super::sun::Sun;
use super::uranus_rings::UranusRings;
use super::upgrade_effects::UpgradeEffects;

/// Inline ship world position computation.
macro_rules! ship_pos {
    ($sa:expr, $tidx:expr, $fly:expr, $pa:expr, $po:expr, $fx:expr, $fy:expr, $ffx:expr, $ffy:expr) => {{
        if $fly {
            ($fx, $fy)
        } else if let Some(idx) = $tidx {
            let pa = PLANET_INITIAL_ANGLES[idx] + $pa[idx];
            let px = 50.0 + PLANET_DATA[idx].0 * pa.cos();
            let py = 50.0 + PLANET_DATA[idx].0 * pa.sin();
            let sa = pa + $po;
            (px + SHIP_PLANET_ORBIT_RADIUS * sa.cos(), py + SHIP_PLANET_ORBIT_RADIUS * sa.sin())
        } else {
            (50.0 + SHIP_ORBIT_RADIUS as f64 * $sa.cos(), 50.0 + SHIP_ORBIT_RADIUS as f64 * $sa.sin())
        }
    }};
}

#[component]
pub fn SolarSystem(
    planet_angles: ReadSignal<Vec<f64>>,
    moon_angles: ReadSignal<Vec<f64>>,
    spaceship_angle: ReadSignal<f64>,
    target_planet_idx: ReadSignal<Option<usize>>,
    is_flying: ReadSignal<bool>,
    fly_from_x: ReadSignal<f64>,
    fly_from_y: ReadSignal<f64>,
    fly_x: ReadSignal<f64>,
    fly_y: ReadSignal<f64>,
    planet_offset: ReadSignal<f64>,
    upgrade_effects: ReadSignal<Vec<UpgradeEffect>>,
    trail_positions: ReadSignal<Vec<(f64, f64, f64)>>,
    flight_path: ReadSignal<Vec<(f64, f64)>>,
    flight_prediction: ReadSignal<Vec<(f64, f64)>>,
    is_arriving: ReadSignal<bool>,
    view_offset_x: ReadSignal<f64>,
    view_offset_y: ReadSignal<f64>,
    zoom_level: ReadSignal<f64>,
) -> impl IntoView {
    view! {
        <Starfield view_offset_x={view_offset_x} view_offset_y={view_offset_y} />
        <div class="absolute inset-0 solar-system-viewport" style=move || {
            let z = zoom_level.get();
            format!("transform:scale({}) translate({}%,{}%);transform-origin:center center;", z, view_offset_x.get()/z, view_offset_y.get()/z)
        }>
            <OrbitalPaths /><Sun />
            <Planets planet_angles={planet_angles} target_planet_idx={target_planet_idx} />

            {/* Saturn rings */}
            <div class="absolute pointer-events-none" style=move || {
                let porbits = planet_angles.get();
                let angle = PLANET_INITIAL_ANGLES[5] + porbits[5];
                let r = PLANET_DATA[5].0;
                format!("left:{}%;top:{}%;transform:translate(-50%,-50%);z-index:15;", 50.0+r*angle.cos(), 50.0+r*angle.sin())
            }><SaturnRings planet_angles={planet_angles} /></div>

            {/* Uranus rings */}
            <div class="absolute pointer-events-none" style=move || {
                let porbits = planet_angles.get();
                let angle = PLANET_INITIAL_ANGLES[6] + porbits[6];
                let r = PLANET_DATA[6].0;
                format!("left:{}%;top:{}%;transform:translate(-50%,-50%);z-index:16;", 50.0+r*angle.cos(), 50.0+r*angle.sin())
            }><UranusRings planet_angles={planet_angles} /></div>

            {/* Jupiter bands */}
            <div class="absolute pointer-events-none" style=move || {
                let porbits = planet_angles.get();
                let angle = PLANET_INITIAL_ANGLES[4] + porbits[4];
                let r = PLANET_DATA[4].0;
                format!("left:{}%;top:{}%;transform:translate(-50%,-50%);z-index:12;", 50.0+r*angle.cos(), 50.0+r*angle.sin())
            }><JupiterBands size={(PLANET_DATA[4].1) as i32} /></div>

            <AsteroidBelt />
            <Moons planet_angles={planet_angles} moon_angles={moon_angles} />
            <ShipTrail trail_positions={trail_positions} />
            <GravityWells planet_angles={planet_angles} target_planet_idx={target_planet_idx} is_flying={is_flying} />
            <FlightPath flight_path={flight_path} flight_prediction={flight_prediction} is_flying={is_flying} is_arriving={is_arriving} fly_x={fly_x} fly_y={fly_y} />
            <Ship spaceship_angle={spaceship_angle} target_planet_idx={target_planet_idx} is_flying={is_flying} planet_angles={planet_angles} planet_offset={planet_offset} fly_x={fly_x} fly_y={fly_y} fly_from_x={fly_from_x} fly_from_y={fly_from_y} />
            <UpgradeEffects upgrade_effects={upgrade_effects} />

            {/* Permanent Dyson and Stellar effects */}
            {move || {
                let (sx, sy) = ship_pos!(spaceship_angle.get(), target_planet_idx.get(), is_flying.get(), &planet_angles.get(), planet_offset.get(), fly_x.get(), fly_y.get(), fly_from_x.get(), fly_from_y.get());
                let effs = upgrade_effects.get();
                view! {
                    { if effs.iter().any(|e| e.upgrade_type == UpgradeType::DysonCollector && e.permanent) { Some(view! { <DysonEffect ship_x={sx} ship_y={sy} /> }.into_view()) } else { None }}
                    { if effs.iter().any(|e| e.upgrade_type == UpgradeType::StellarEngine && e.permanent) { Some(view! { <StellarEffect ship_x={sx} ship_y={sy} /> }.into_view()) } else { None }}
                }
            }}
        </div>
    }
}
