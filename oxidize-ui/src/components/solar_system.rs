//! Main SolarSystem component
//! 
//! Orchestrates layout of all child components - passes signals down
//! to child components. Contains viewport transform div that applies
//! pan and zoom based on target planet.

use leptos::*;

use crate::types::UpgradeEffect;

use super::asteroid_belt::AsteroidBelt;
use super::flight_path::FlightPath;
use super::moons::Moons;
use super::orbital_paths::OrbitalPaths;
use super::planets::Planets;
use super::saturn_rings::SaturnRings;
use super::ship::Ship;
use super::ship_trail::ShipTrail;
use super::starfield::Starfield;
use super::sun::Sun;
use super::uranus_rings::UranusRings;
use super::upgrade_effects::UpgradeEffects;

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
        {/* Star field background with parallax */}
        <Starfield view_offset_x={view_offset_x} view_offset_y={view_offset_y} />

        {/* Solar system content with viewport transform */}
        <div 
            class="absolute inset-0 solar-system-viewport"
            style=move || {
                let z = zoom_level.get();
                // Divide by scale because CSS applies translate AFTER scale
                let ox = view_offset_x.get() / z;
                let oy = view_offset_y.get() / z;
                format!(
                    "transform: scale({}) translate({}%, {}%); transform-origin: center center;",
                    z, ox, oy
                )
            }
        >
            {/* Orbital path rings */}
            <OrbitalPaths />

            {/* Sun with corona */}
            <Sun />

            {/* All 8 planets */}
            <Planets planet_angles={planet_angles} target_planet_idx={target_planet_idx} />

            {/* Saturn rings (positioned with planet) */}
            <div class="absolute pointer-events-none" style=move || {
                let porbits = planet_angles.get();
                let angle = crate::constants::PLANET_INITIAL_ANGLES[5] + porbits[5];
                let r = crate::constants::PLANET_DATA[5].0;
                let x = 50.0 + r * angle.cos();
                let y = 50.0 + r * angle.sin();
                format!("left: {}%%; top: {}%%; transform: translate(-50%%, -50%%); z-index: 15;", x, y)
            }>
                <SaturnRings planet_angles={planet_angles} />
            </div>

            {/* Uranus rings (positioned with planet) */}
            <div class="absolute pointer-events-none" style=move || {
                let porbits = planet_angles.get();
                let angle = crate::constants::PLANET_INITIAL_ANGLES[6] + porbits[6];
                let r = crate::constants::PLANET_DATA[6].0;
                let x = 50.0 + r * angle.cos();
                let y = 50.0 + r * angle.sin();
                format!("left: {}%%; top: {}%%; transform: translate(-50%%, -50%%); z-index: 16;", x, y)
            }>
                <UranusRings planet_angles={planet_angles} />
            </div>

            {/* Asteroid belt */}
            <AsteroidBelt />

            {/* All moons */}
            <Moons planet_angles={planet_angles} moon_angles={moon_angles} />

            {/* Ship trail particles */}
            <ShipTrail trail_positions={trail_positions} />

            {/* Flight path and prediction */}
            <FlightPath 
                flight_path={flight_path}
                flight_prediction={flight_prediction}
                is_flying={is_flying}
                is_arriving={is_arriving}
                planet_angles={planet_angles}
                target_planet_idx={target_planet_idx}
                fly_x={fly_x}
                fly_y={fly_y}
            />

            {/* Spaceship */}
            <Ship 
                spaceship_angle={spaceship_angle}
                target_planet_idx={target_planet_idx}
                is_flying={is_flying}
                planet_angles={planet_angles}
                planet_offset={planet_offset}
                fly_x={fly_x}
                fly_y={fly_y}
                fly_from_x={fly_from_x}
                fly_from_y={fly_from_y}
            />

            {/* Upgrade effects */}
            <UpgradeEffects upgrade_effects={upgrade_effects} />
        </div>
    }
}
