use leptos::*;

#[component]
pub fn NeonOrb() -> impl IntoView {
    view! {
        <div class="flex-1 w-full relative pointer-events-none flex items-center justify-center">
            <div class="neon-orb"><div class="neon-orb-inner"></div></div>
        </div>
    }
}
