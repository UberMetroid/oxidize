use leptos::*;

#[component]
pub fn HowToPlayModal(on_close: Callback<()>) -> impl IntoView {
    view! {
        <div class="fixed inset-0 bg-black/80 z-50 flex items-center justify-center p-4"
             on:click={move |_| on_close.call(())}>
            <div class="glass-pad max-w-lg w-full max-h-[80vh] overflow-auto"
                 on:click=|e| { e.stop_propagation(); }>
                <h2 class="text-2xl font-bold text-theme-primary mb-4 text-center">HOW TO PLAY</h2>
                <div class="space-y-4 text-sm opacity-80">
                    <p>
                        <span class="text-theme-primary font-bold">OXIDIZE</span> is a zero-click
                        incremental game. Energy flows automatically - you just purchase upgrades.
                    </p>
                    <div>
                        <h3 class="text-theme-primary font-bold mb-2">UPGRADES</h3>
                        <ul class="space-y-2">
                            <li><span class="font-bold">Solar Sail</span> - Cheap, generates 1 MW/s</li>
                            <li><span class="font-bold">Plasma Tether</span> - Moderate, generates 25 MW/s</li>
                            <li><span class="font-bold">Orbital Mirror</span> - Expensive, generates 1000 MW/s</li>
                        </ul>
                    </div>
                    <div>
                        <h3 class="text-theme-primary font-bold mb-2">FACTIONS</h3>
                        <p>Choose your faction color. Purely cosmetic.</p>
                    </div>
                </div>
            </div>
        </div>
    }
}
