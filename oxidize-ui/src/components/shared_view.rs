use leptos::*;

use crate::api::GlobalStats;

fn format_number(n: f64) -> String {
    if n >= 1_000_000_000.0 {
        format!("{:.1}B", n / 1_000_000_000.0)
    } else if n >= 1_000_000.0 {
        format!("{:.1}M", n / 1_000_000.0)
    } else if n >= 1_000.0 {
        format!("{:.1}K", n / 1_000.0)
    } else {
        format!("{:.0}", n)
    }
}

#[component]
pub fn SharedViewModal(stats: GlobalStats, on_close: Callback<()>) -> impl IntoView {
    view! {
        <div class="fixed inset-0 bg-black/80 z-50 flex items-center justify-center p-4"
             on:click={move |_| on_close.call(())}>
            <div class="glass-pad max-w-lg w-full max-h-[80vh] overflow-auto"
                 on:click=|e| { e.stop_propagation(); }>
                <h2 class="text-2xl font-bold text-theme-primary mb-4 text-center">
                    "SHARED DYSPHERE"
                </h2>

                <div class="space-y-4 text-center">
                    <div>
                        <div class="text-sm uppercase tracking-widest opacity-60">"Total Capital"</div>
                        <div class="text-4xl font-black text-white">
                            {format!("{} MW", format_number(stats.total_energy))}
                        </div>
                    </div>

                    <div>
                        <div class="text-sm uppercase tracking-widest opacity-60">"Active Builders"</div>
                        <div class="text-2xl font-bold text-theme-primary">
                            {stats.total_players}
                        </div>
                    </div>

                    <div class="border-t border-white/10 pt-4 mt-4">
                        <div class="text-sm uppercase tracking-widest opacity-60 mb-2">"Collective Upgrades"</div>
                        <div class="grid grid-cols-3 gap-2 text-center">
                            <div class="glass-pad p-2">
                                <div class="text-xs opacity-60">"Sails"</div>
                                <div class="text-lg font-bold">{format_number(stats.total_solar_sails as f64)}</div>
                            </div>
                            <div class="glass-pad p-2">
                                <div class="text-xs opacity-60">"Tethers"</div>
                                <div class="text-lg font-bold">{format_number(stats.total_plasma_tethers as f64)}</div>
                            </div>
                            <div class="glass-pad p-2">
                                <div class="text-xs opacity-60">"Mirrors"</div>
                                <div class="text-lg font-bold">{format_number(stats.total_orbital_mirrors as f64)}</div>
                            </div>
                        </div>
                    </div>
                </div>

                <button
                    on:click={move |_| on_close.call(())}
                    class="mt-6 w-full px-6 py-2 glass-pad text-sm font-bold tracking-widest hover:scale-105 transition-all text-theme-primary"
                >
                    "CLOSE"
                </button>
            </div>
        </div>
    }
}
