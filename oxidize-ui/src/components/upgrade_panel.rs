use leptos::*;

use crate::{PlayerState, UpgradeType};

/// Color mapping for upgrade types
fn upgrade_color(upgrade: UpgradeType) -> &'static str {
    match upgrade {
        UpgradeType::SolarSail => "#3b82f6",
        UpgradeType::PlasmaTether => "#a855f7",
        UpgradeType::OrbitalMirror => "#f59e0b",
        UpgradeType::DysonCollector => "#ef4444",
        UpgradeType::QuantumArray => "#22c55e",
        UpgradeType::StellarEngine => "#fef08a",
    }
}

#[component]
pub fn UpgradePanel(
    state: ReadSignal<PlayerState>,
    on_buy: Callback<UpgradeType>,
) -> impl IntoView {
    let upgrades = [
        UpgradeType::SolarSail,
        UpgradeType::PlasmaTether,
        UpgradeType::OrbitalMirror,
    ];

    view! {
        <div class="flex flex-col sm:flex-row gap-4 w-full max-w-3xl px-4" role="group" aria-label="Upgrade Panel">
            {upgrades.iter().map(|upgrade| {
                let upgrade = *upgrade;
                let color = upgrade_color(upgrade);
                let owned = move || state.get().count_for_upgrade(upgrade);
                let affordable = move || state.get().can_afford(upgrade);
                let cost = move || UpgradeType::SolarSail.calculate_cost(state.get().count_for_upgrade(UpgradeType::SolarSail));
                let eps = upgrade.energy_per_second();

                view! {
                    <button
                        on:click={move |_| on_buy.call(upgrade)}
                        disabled={!affordable()}
                        class="flex-1 flex flex-col items-center justify-center p-4 glass-pad hover:scale-[1.02] transition-all disabled:opacity-30 disabled:hover:scale-100 disabled:cursor-not-allowed group relative overflow-hidden"
                        style={format!("border: 1px solid {}33;", color)}
                        aria-label={format!("Purchase {} upgrade", upgrade.name())}
                        aria-disabled={!affordable()}
                    >
                        // Count badge
                        {if owned() > 0 {
                            view! {
                                <div class="absolute -top-2 -right-2 bg-gradient-to-br from-cyan-400 to-blue-500 text-white text-xs font-bold w-6 h-6 rounded-full flex items-center justify-center shadow-lg">
                                    {owned()}
                                </div>
                            }.into_view()
                        } else {
                            view! { <div></div> }.into_view()
                        }}

                        <div class="text-xs uppercase opacity-70 mb-1">{upgrade.name()}</div>
                        <div class="text-xl font-black text-theme-primary group-disabled:text-white">
                            {move || format!("{:.0} MW", cost())}
                        </div>

                        // Energy contribution
                        <div class="text-xs mt-1" style={format!("color: {}", color)}>
                            +{format!("{:.1}", eps)} MW/s each
                        </div>

                        // Glow effect when affordable
                        {if affordable() {
                            view! {
                                <div class="absolute inset-0 opacity-20 pointer-events-none transition-opacity group-hover:opacity-40"
                                    style={format!("background: radial-gradient(circle at center, {} 0%, transparent 70%);", color)}></div>
                            }.into_view()
                        } else {
                            view! { <div></div> }.into_view()
                        }}
                    </button>
                }
            }).collect::<Vec<_>>()}
        </div>
    }
}
