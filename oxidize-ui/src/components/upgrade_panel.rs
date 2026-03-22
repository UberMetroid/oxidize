use leptos::*;

use crate::{PlayerState, UpgradeType};

#[component]
pub fn UpgradePanel(
    state: ReadSignal<PlayerState>,
    on_buy: Callback<UpgradeType>,
) -> impl IntoView {
    view! {
        <div class="flex flex-col sm:flex-row gap-4 w-full max-w-3xl px-4">
            <button
                on:click={move |_| on_buy.call(UpgradeType::SolarSail)}
                disabled={move || !state.get().can_afford(UpgradeType::SolarSail)}
                class="flex-1 flex flex-col items-center justify-center p-4 glass-pad hover:scale-[1.02] transition-all disabled:opacity-30 disabled:hover:scale-100 disabled:cursor-not-allowed group relative overflow-hidden"
            >
                <div class="text-xs uppercase opacity-70 mb-1">"Solar Sail"</div>
                <div class="text-xl font-black text-theme-primary group-disabled:text-white">
                    {move || format!("{:.0} MW", UpgradeType::SolarSail.calculate_cost(state.get().count_for_upgrade(UpgradeType::SolarSail)))}
                </div>
                <div class="absolute top-2 right-2 text-[10px] font-bold opacity-50">
                    {move || state.get().count_for_upgrade(UpgradeType::SolarSail)}
                </div>
            </button>
            <button
                on:click={move |_| on_buy.call(UpgradeType::PlasmaTether)}
                disabled={move || !state.get().can_afford(UpgradeType::PlasmaTether)}
                class="flex-1 flex flex-col items-center justify-center p-4 glass-pad hover:scale-[1.02] transition-all disabled:opacity-30 disabled:hover:scale-100 disabled:cursor-not-allowed group relative overflow-hidden"
            >
                <div class="text-xs uppercase opacity-70 mb-1">"Plasma Tether"</div>
                <div class="text-xl font-black text-theme-primary group-disabled:text-white">
                    {move || format!("{:.0} MW", UpgradeType::PlasmaTether.calculate_cost(state.get().count_for_upgrade(UpgradeType::PlasmaTether)))}
                </div>
                <div class="absolute top-2 right-2 text-[10px] font-bold opacity-50">
                    {move || state.get().count_for_upgrade(UpgradeType::PlasmaTether)}
                </div>
            </button>
            <button
                on:click={move |_| on_buy.call(UpgradeType::OrbitalMirror)}
                disabled={move || !state.get().can_afford(UpgradeType::OrbitalMirror)}
                class="flex-1 flex flex-col items-center justify-center p-4 glass-pad hover:scale-[1.02] transition-all disabled:opacity-30 disabled:hover:scale-100 disabled:cursor-not-allowed group relative overflow-hidden"
            >
                <div class="text-xs uppercase opacity-70 mb-1">"Orbital Mirror"</div>
                <div class="text-xl font-black text-theme-primary group-disabled:text-white">
                    {move || format!("{:.0} MW", UpgradeType::OrbitalMirror.calculate_cost(state.get().count_for_upgrade(UpgradeType::OrbitalMirror)))}
                </div>
                <div class="absolute top-2 right-2 text-[10px] font-bold opacity-50">
                    {move || state.get().count_for_upgrade(UpgradeType::OrbitalMirror)}
                </div>
            </button>
        </div>
    }
}
