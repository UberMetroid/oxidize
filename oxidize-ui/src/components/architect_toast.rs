use leptos::*;

#[component]
pub fn ArchitectToast(message: String, on_close: Callback<()>) -> impl IntoView {
    view! {
        <div class="fixed bottom-20 left-1/2 transform -translate-x-1/2 z-50 px-6 py-3 glass-pad snarky-toast text-center max-w-[90vw]">
            <div class="text-sm sm:text-base text-theme-primary font-mono tracking-wide">
                {message}
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
