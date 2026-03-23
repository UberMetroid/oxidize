use crate::api::AchievementInfo;
use leptos::*;

#[component]
pub fn AchievementToast(
    achievements: Vec<AchievementInfo>,
    on_close: Callback<()>,
) -> impl IntoView {
    view! {
        <div class="fixed bottom-20 left-1/2 transform -translate-x-1/2 z-50 px-6 py-4 glass-pad achievement-toast text-center max-w-[90vw]">
            <div class="text-lg text-theme-primary font-mono tracking-wide mb-2">"🏆 Achievement Unlocked!"</div>
            <div class="space-y-2">
                {achievements.iter().map(|_a| {
                    view! {
                        <div class="text-sm sm:text-base text-theme-primary font-mono">
                            <span class="text-yellow-400">"{_a.name}"</span>
                            <br/>
                            <span class="text-xs opacity-75">"{_a.description}"</span>
                        </div>
                    }
                }).collect::<Vec<_>>()}
            </div>
            <button
                on:click={move |_| on_close.call(())}
                class="absolute top-1 right-2 text-xs opacity-50 hover:opacity-100"
            >
                "×"
            </button>
        </div>
    }
}
