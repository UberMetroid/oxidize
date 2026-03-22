use crate::api::LeaderboardEntry;
use leptos::*;

#[component]
pub fn LeaderboardModal(
    entries: impl Fn() -> Vec<LeaderboardEntry> + 'static,
    on_close: Callback<()>,
) -> impl IntoView {
    view! {
        <div class="fixed inset-0 bg-black/80 z-50 flex items-center justify-center p-4"
             on:click={move |_| on_close.call(())}>
            <div class="glass-pad max-w-lg w-full max-h-[80vh] overflow-auto"
                 on:click=|e| { e.stop_propagation(); }>
                <h2 class="text-2xl font-bold text-theme-primary mb-4 text-center">LEADERBOARD</h2>
                <div class="space-y-2">
                    <For each={entries} key=|entry| entry.rank
                        let:entry>
                        <div class="flex items-center gap-4 p-2 border-b border-white/10">
                            <span class="w-8 text-theme-primary font-bold">#{entry.rank}</span>
                            <span class="flex-1 text-sm opacity-70">
                                {format!("{}...", &entry.uuid[..8.min(entry.uuid.len())])}
                            </span>
                            <span class="text-lg font-black text-white">
                                {format!("{:.0} MW", entry.total_energy)}
                            </span>
                        </div>
                    </For>
                </div>
            </div>
        </div>
    }
}
