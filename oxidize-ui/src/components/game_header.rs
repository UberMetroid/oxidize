use leptos::*;

use crate::PlayerState;

#[component]
pub fn GameHeader(state: ReadSignal<PlayerState>) -> impl IntoView {
    view! {
        <div class="flex flex-col items-center pt-8 relative z-10 shrink-0 w-full px-4">
            <h1 class="text-5xl sm:text-6xl font-black tracking-widest text-theme-primary">"OXIDIZE"</h1>
            <div class="mt-8 flex flex-col items-center p-6 w-full max-w-lg pointer-events-auto">
                <div class="text-sm uppercase tracking-widest opacity-60">"STORED CAPITAL"</div>
                <div class="text-4xl sm:text-5xl font-black text-white mt-1 mb-2">
                    {move || format!("{:.1} MW", state.get().energy)}
                </div>
                <div class="text-xs text-theme-primary tracking-wider font-bold">
                    {move || format!("+ {:.1} MW/s", state.get().energy_per_second())}
                </div>
            </div>
        </div>
    }
}
