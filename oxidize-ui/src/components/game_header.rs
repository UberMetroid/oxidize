//! Top HUD: title, energy display, and planet selector buttons.

use leptos::*;
use crate::PlayerState;

/// Passes planet_idx signal so game_header can trigger clicks without closure props.
#[component]
pub fn GameHeader(
    state: ReadSignal<PlayerState>,
    target_planet_idx: ReadSignal<Option<usize>>,
    /// Writes planet index to select (Some(usize::MAX) = sun)
    pending_planet_select: WriteSignal<Option<usize>>,
) -> impl IntoView {
    view! {
        <div class="flex flex-col items-center pt-4 pb-2 relative z-10 shrink-0 w-full px-4 gap-2">
            <h1 class="text-3xl font-black tracking-widest text-theme-primary">"OXIDIZE"</h1>
            <div class="flex flex-col items-center">
                <div class="text-2xl font-black text-white">
                    {move || format!("{:.1} MW", state.get().energy)}
                </div>
                <div class="text-xs text-theme-primary tracking-wider font-bold">
                    {move || format!("+ {:.1} MW/s", state.get().energy_per_second())}
                </div>
            </div>
            <div class="flex justify-center gap-1 px-2 flex-wrap">
                <button on:click={move |_| pending_planet_select.set(Some(usize::MAX))}
                    class="glass-pad text-xs font-bold px-2 py-0.5 transition-all hover:scale-105 text-yellow-400">
                    Sun
                </button>
                <button on:click={move |_| pending_planet_select.set(Some(0))}
                    class="glass-pad text-xs font-bold px-2 py-0.5 transition-all hover:scale-105"
                    class:opacity-100={target_planet_idx.get() == Some(0)}
                    class:opacity-60={target_planet_idx.get() != Some(0)}>
                    <span style="color: #9ca3af">Mercury</span>
                </button>
                <button on:click={move |_| pending_planet_select.set(Some(1))}
                    class="glass-pad text-xs font-bold px-2 py-0.5 transition-all hover:scale-105"
                    class:opacity-100={target_planet_idx.get() == Some(1)}
                    class:opacity-60={target_planet_idx.get() != Some(1)}>
                    <span style="color: #fbbf24">Venus</span>
                </button>
                <button on:click={move |_| pending_planet_select.set(Some(2))}
                    class="glass-pad text-xs font-bold px-2 py-0.5 transition-all hover:scale-105"
                    class:opacity-100={target_planet_idx.get() == Some(2)}
                    class:opacity-60={target_planet_idx.get() != Some(2)}>
                    <span style="color: #3b82f6">Earth</span>
                </button>
                <button on:click={move |_| pending_planet_select.set(Some(3))}
                    class="glass-pad text-xs font-bold px-2 py-0.5 transition-all hover:scale-105"
                    class:opacity-100={target_planet_idx.get() == Some(3)}
                    class:opacity-60={target_planet_idx.get() != Some(3)}>
                    <span style="color: #ef4444">Mars</span>
                </button>
                <button on:click={move |_| pending_planet_select.set(Some(4))}
                    class="glass-pad text-xs font-bold px-2 py-0.5 transition-all hover:scale-105"
                    class:opacity-100={target_planet_idx.get() == Some(4)}
                    class:opacity-60={target_planet_idx.get() != Some(4)}>
                    <span style="color: #f97316">Jupiter</span>
                </button>
                <button on:click={move |_| pending_planet_select.set(Some(5))}
                    class="glass-pad text-xs font-bold px-2 py-0.5 transition-all hover:scale-105"
                    class:opacity-100={target_planet_idx.get() == Some(5)}
                    class:opacity-60={target_planet_idx.get() != Some(5)}>
                    <span style="color: #eab308">Saturn</span>
                </button>
                <button on:click={move |_| pending_planet_select.set(Some(6))}
                    class="glass-pad text-xs font-bold px-2 py-0.5 transition-all hover:scale-105"
                    class:opacity-100={target_planet_idx.get() == Some(6)}
                    class:opacity-60={target_planet_idx.get() != Some(6)}>
                    <span style="color: #06b6d4">Uranus</span>
                </button>
                <button on:click={move |_| pending_planet_select.set(Some(7))}
                    class="glass-pad text-xs font-bold px-2 py-0.5 transition-all hover:scale-105"
                    class:opacity-100={target_planet_idx.get() == Some(7)}
                    class:opacity-60={target_planet_idx.get() != Some(7)}>
                    <span style="color: #6366f1">Neptune</span>
                </button>
            </div>
        </div>
    }
}
