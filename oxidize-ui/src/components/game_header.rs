//! Top HUD: title, energy display, and Saturn Asteroids branding.

use leptos::*;

#[component]
pub fn GameHeader(
    energy: impl Fn() -> f64 + 'static,
    eps: impl Fn() -> f64 + 'static,
) -> impl IntoView {
    view! {
        <div class="flex flex-col items-center pt-4 pb-2 relative z-10 shrink-0 w-full px-4 gap-1">
            <h1 class="text-3xl font-black tracking-widest text-theme-primary">"OXIDIZE"</h1>
            <div class="text-lg text-yellow-400 font-mono text-xs tracking-widest">"SATURN FIELD"</div>
            <div class="flex flex-col items-center">
                <div class="text-2xl font-black text-white">{move || format!("{:.1} MW", energy())}</div>
                <div class="text-xs text-theme-primary tracking-wider font-bold">
                    {move || format!("+ {:.1} MW/s", eps())}
                </div>
            </div>
        </div>
    }
}
