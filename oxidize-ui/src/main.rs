use leptos::*;
use wasm_bindgen::JsCast;
use oxidize_engine::{PlayerState, Faction, UpgradeType};

#[component]
fn App() -> impl IntoView {
    let (theme, set_theme) = create_signal("orange".to_string());
    let (state, set_state) = create_signal(PlayerState::new(Faction::Orange));

    // Game Loop
    create_effect(move |_| {
        let interval = gloo_timers::callback::Interval::new(100, move || {
            set_state.update(|s| s.tick(0.1));
        });
        interval.forget(); // Keep it running forever
    });

    create_effect(move |_| {
        if let Some(window) = web_sys::window() {
            if let Ok(Some(storage)) = window.local_storage() {
                if let Ok(Some(t)) = storage.get_item("color-theme") {
                    set_theme.set(t.clone());
                } else {
                    let _ = storage.set_item("color-theme", &theme.get());
                }
            }
        }
    });

    create_effect(move |_| {
        let t = theme.get();
        if let Some(window) = web_sys::window() {
            if let Some(doc) = window.document() {
                if let Some(el) = doc.document_element() {
                    let _ = el.set_attribute("class", &format!("theme-{}", t));
                }
            }
        }
    });

    view! {
        <div class="flex flex-col h-full bg-app-bg text-app-text overflow-hidden transition-all duration-500 relative font-mono">
            {/* TOP HEADER */}
            <div class="flex flex-col items-center pt-8 relative z-10 pointer-events-none shrink-0 w-full px-4">
                <h1 class="text-5xl sm:text-6xl font-black tracking-widest text-theme-primary">
                    "OXIDIZE"
                </h1>
                
                {/* ENERGY METRICS */}
                <div class="mt-8 flex flex-col items-center p-6 glass-pad w-full max-w-lg pointer-events-auto">
                    <div class="text-sm uppercase tracking-widest opacity-60">"STORED CAPITAL"</div>
                    <div class="text-4xl sm:text-5xl font-black text-white mt-1 mb-2">
                        {move || format!("{:.1} MW", state.get().energy)}
                    </div>
                    <div class="text-xs text-theme-primary tracking-wider font-bold">
                        {move || format!("+ {:.1} MW/s", state.get().energy_per_second())}
                    </div>
                </div>
            </div>

            {/* MIDDLE (3D SPACE) */}
            <div class="flex-1 pointer-events-none"></div>

            {/* BOTTOM FOOTER / CONTROLS */}
            <div class="w-full flex flex-col items-center pb-8 shrink-0 relative z-10 pointer-events-auto gap-6">
                
                {/* UPGRADE STORE */}
                <div class="flex flex-col sm:flex-row gap-4 w-full max-w-3xl px-4">
                    {
                        let upgrades = vec![UpgradeType::SolarSail, UpgradeType::PlasmaTether, UpgradeType::OrbitalMirror];
                        upgrades.into_iter().map(move |upgrade| {
                            let name = upgrade.name();
                            view! {
                                <button 
                                    on:click=move |_| { set_state.update(|s| { s.buy_upgrade(upgrade); }); }
                                    disabled=move || !state.get().can_afford(upgrade)
                                    class="flex-1 flex flex-col items-center justify-center p-4 glass-pad hover:scale-[1.02] transition-all disabled:opacity-30 disabled:hover:scale-100 disabled:cursor-not-allowed group relative overflow-hidden"
                                >
                                    <div class="text-xs uppercase opacity-70 mb-1">{name}</div>
                                    <div class="text-xl font-black text-theme-primary group-disabled:text-white">
                                        {move || format!("{:.0} MW", upgrade.calculate_cost(state.get().count_for_upgrade(upgrade)))}
                                    </div>
                                    <div class="absolute top-2 right-2 text-[10px] font-bold opacity-50">
                                        {move || state.get().count_for_upgrade(upgrade)}
                                    </div>
                                </button>
                            }
                        }).collect_view()
                    }
                </div>

                {/* FACTION SELECTOR */}
                <div class="flex gap-4 mt-2">
                    {
                        let themes = ["red", "orange", "yellow", "green", "blue", "purple"];
                        themes.into_iter().map(|t| {
                            let bg = match t {
                                "red" => "bg-red-500",
                                "orange" => "bg-orange-500",
                                "yellow" => "bg-yellow-400",
                                "green" => "bg-green-500",
                                "blue" => "bg-blue-500",
                                "purple" => "bg-purple-500",
                                _ => "bg-gray-500"
                            };
                            let t_str = t.to_string();
                            view! {
                                <button 
                                    on:click=move |_| {
                                        set_theme.set(t_str.clone());
                                        if let Some(window) = web_sys::window() {
                                            if let Ok(Some(storage)) = window.local_storage() {
                                                let _ = storage.set_item("color-theme", &t_str);
                                            }
                                        }
                                    }
                                    class=format!("w-10 h-10 clip-hexagon hover:scale-110 hover:brightness-125 transition-all cursor-pointer shadow-[0_0_15px_rgba(255,255,255,0.1)] hover:shadow-[0_0_20px_currentColor] {}", bg)
                                />
                            }
                        }).collect_view()
                    }
                </div>
            </div>
        </div>
    }
}

fn main() {
    console_error_panic_hook::set_once();
    let root = web_sys::window()
        .unwrap()
        .document()
        .unwrap()
        .get_element_by_id("root")
        .expect("could not find #root element");
    mount_to(root.unchecked_into(), || view! { <App/> })
}
