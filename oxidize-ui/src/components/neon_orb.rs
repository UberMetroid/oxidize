use leptos::*;

#[component]
pub fn NeonOrb(intensity: Option<i32>) -> impl IntoView {
    let intensity_class = intensity
        .map(|i| if i > 100 { "intense" } else { "" })
        .unwrap_or("");

    view! {
        <div class="absolute inset-0 pointer-events-none z-40 flex items-center justify-center">
            <div class="neon-orb" class:intense={!intensity_class.is_empty()}>
                <div class="neon-orb-inner"></div>
            </div>
        </div>
    }
}

/// Energy-reactive NeonOrb that scales glow with energy level
#[component]
pub fn NeonOrbEnergyAware(
    energy_per_second: ReadSignal<f64>,
) -> impl IntoView {
    view! {
        <div class="absolute inset-0 pointer-events-none z-40 flex items-center justify-center">
            <div class="neon-orb-energy-aware" data-eps={move || energy_per_second.get().round() as i32}>
                <div class="neon-orb-inner"></div>
            </div>
        </div>
    }
}
