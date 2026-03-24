use leptos::*;

use crate::PlayerState;

#[component]
pub fn GameHeader(
    state: ReadSignal<PlayerState>,
    target_planet_idx: ReadSignal<Option<usize>>,
    on_select_sun: impl Fn() + Clone + 'static,
    on_select_planet: impl Fn(usize) + Clone + 'static,
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
            {/* Planet Selector Buttons */}
            <div class="flex justify-center gap-1 px-2 flex-wrap">
                <button
                    on:click={move |_| { on_select_sun(); }}
                    class="glass-pad text-xs font-bold px-2 py-0.5 transition-all hover:scale-105"
                    class:text-yellow-400={target_planet_idx.get().is_none()}
                    class:text-white={target_planet_idx.get().is_some()}
                    class:opacity-100={target_planet_idx.get().is_none()}
                    class:opacity-60={target_planet_idx.get().is_some()}
                >
                    Sun
                </button>
                <button
                    on:click={{let op = on_select_planet.clone(); move |_| { op(0); }}}
                    class="glass-pad text-xs font-bold px-2 py-0.5 transition-all hover:scale-105"
                    class:text-white={target_planet_idx.get() == Some(0)}
                    class:text-gray-400={target_planet_idx.get() != Some(0)}
                    class:opacity-100={target_planet_idx.get() == Some(0)}
                    class:opacity-60={target_planet_idx.get() != Some(0)}
                >
                    Mercury
                </button>
                <button
                    on:click={{let op = on_select_planet.clone(); move |_| { op(1); }}}
                    class="glass-pad text-xs font-bold px-2 py-0.5 transition-all hover:scale-105"
                    class:text-yellow-300={target_planet_idx.get() == Some(1)}
                    class:text-white={target_planet_idx.get() != Some(1)}
                    class:opacity-100={target_planet_idx.get() == Some(1)}
                    class:opacity-60={target_planet_idx.get() != Some(1)}
                >
                    Venus
                </button>
                <button
                    on:click={{let op = on_select_planet.clone(); move |_| { op(2); }}}
                    class="glass-pad text-xs font-bold px-2 py-0.5 transition-all hover:scale-105"
                    class:text-blue-400={target_planet_idx.get() == Some(2)}
                    class:text-white={target_planet_idx.get() != Some(2)}
                    class:opacity-100={target_planet_idx.get() == Some(2)}
                    class:opacity-60={target_planet_idx.get() != Some(2)}
                >
                    Earth
                </button>
                <button
                    on:click={{let op = on_select_planet.clone(); move |_| { op(3); }}}
                    class="glass-pad text-xs font-bold px-2 py-0.5 transition-all hover:scale-105"
                    class:text-red-500={target_planet_idx.get() == Some(3)}
                    class:text-white={target_planet_idx.get() != Some(3)}
                    class:opacity-100={target_planet_idx.get() == Some(3)}
                    class:opacity-60={target_planet_idx.get() != Some(3)}
                >
                    Mars
                </button>
                <button
                    on:click={{let op = on_select_planet.clone(); move |_| { op(4); }}}
                    class="glass-pad text-xs font-bold px-2 py-0.5 transition-all hover:scale-105"
                    class:text-orange-500={target_planet_idx.get() == Some(4)}
                    class:text-white={target_planet_idx.get() != Some(4)}
                    class:opacity-100={target_planet_idx.get() == Some(4)}
                    class:opacity-60={target_planet_idx.get() != Some(4)}
                >
                    Jupiter
                </button>
                <button
                    on:click={{let op = on_select_planet.clone(); move |_| { op(5); }}}
                    class="glass-pad text-xs font-bold px-2 py-0.5 transition-all hover:scale-105"
                    class:text-yellow-400={target_planet_idx.get() == Some(5)}
                    class:text-white={target_planet_idx.get() != Some(5)}
                    class:opacity-100={target_planet_idx.get() == Some(5)}
                    class:opacity-60={target_planet_idx.get() != Some(5)}
                >
                    Saturn
                </button>
                <button
                    on:click={{let op = on_select_planet.clone(); move |_| { op(6); }}}
                    class="glass-pad text-xs font-bold px-2 py-0.5 transition-all hover:scale-105"
                    class:text-cyan-400={target_planet_idx.get() == Some(6)}
                    class:text-white={target_planet_idx.get() != Some(6)}
                    class:opacity-100={target_planet_idx.get() == Some(6)}
                    class:opacity-60={target_planet_idx.get() != Some(6)}
                >
                    Uranus
                </button>
                <button
                    on:click={{let op = on_select_planet.clone(); move |_| { op(7); }}}
                    class="glass-pad text-xs font-bold px-2 py-0.5 transition-all hover:scale-105"
                    class:text-indigo-400={target_planet_idx.get() == Some(7)}
                    class:text-white={target_planet_idx.get() != Some(7)}
                    class:opacity-100={target_planet_idx.get() == Some(7)}
                    class:opacity-60={target_planet_idx.get() != Some(7)}
                >
                    Neptune
                </button>
            </div>
        </div>
    }
}
