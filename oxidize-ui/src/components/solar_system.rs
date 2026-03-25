//! Solar system with 3-layer compositing: fixed background, panning planets layer.

use leptos::*;
use crate::components::*;

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
    upgrade_effects: ReadSignal<Vec<crate::types::UpgradeEffect>>,
    trail_positions: ReadSignal<Vec<(f64, f64, f64)>>,
    flight_path: ReadSignal<Vec<(f64, f64)>>,
    flight_prediction: ReadSignal<Vec<(f64, f64)>>,
    is_arriving: ReadSignal<bool>,
    view_offset_x: ReadSignal<f64>,
    view_offset_y: ReadSignal<f64>,
    zoom_level: ReadSignal<f64>,
) -> impl IntoView {
    view! {
        {/* ── Fixed background: starfield + orbital rings + asteroid belt ──── */}
        <div class="absolute inset-0 flex items-center justify-center pointer-events-none" style="z-index: 0;">
            <div class="relative w-full h-full">
                <Starfield view_offset_x={view_offset_x} view_offset_y={view_offset_y} />
                <OrbitalPaths />
                <AsteroidBelt />
            </div>
        </div>

        {/* ── Panning layer: planets + moons + ship + effects + trails ───── */}
        <div
            class="absolute inset-0"
            style={move || format!(
                "transform: translate({:.1}%, {:.1}%);",
                view_offset_x.get(), view_offset_y.get()
            )}
        >
            <Moons planet_angles={planet_angles} moon_angles={moon_angles} />
            <Planets planet_angles={planet_angles} target_planet_idx={target_planet_idx} zoom_level={zoom_level} />
            <SaturnRings planet_angles={planet_angles} />
            <UranusRings planet_angles={planet_angles} />
            <UpgradeEffects upgrade_effects={upgrade_effects} planet_angles={planet_angles} />
            <Ship ship_angle={spaceship_angle} target_planet_idx={target_planet_idx} is_flying={is_flying} planet_angles={planet_angles} planet_offset={planet_offset} fly_x={fly_x} fly_y={fly_y} fly_from_x={fly_from_x} fly_from_y={fly_from_y} />
            <ShipTrail trail_positions={trail_positions} />
            <FlightPath flight_path={flight_path} flight_prediction={flight_prediction} is_flying={is_flying} is_arriving={is_arriving} fly_x={fly_x} fly_y={fly_y} />
            <GravityWells planet_angles={planet_angles} target_planet_idx={target_planet_idx} is_flying={is_flying} />
        </div>
    }
}
