mod api;
mod components;

use leptos::*;
use oxidize_engine::architect::{generate_quip, QuipTrigger};
use oxidize_engine::{PlayerState, UpgradeType};
use wasm_bindgen::JsCast;
use std::f64::consts::PI;

fn get_player_uuid() -> String {
    if let Some(window) = web_sys::window() {
        if let Ok(Some(storage)) = window.local_storage() {
            if let Ok(Some(uuid)) = storage.get_item("player-uuid") {
                return uuid;
            }
            let new_uuid = uuid::Uuid::new_v4().to_string();
            let _ = storage.set_item("player-uuid", &new_uuid);
            return new_uuid;
        }
    }
    uuid::Uuid::new_v4().to_string()
}

fn load_state() -> PlayerState {
    if let Some(window) = web_sys::window() {
        if let Ok(Some(storage)) = window.local_storage() {
            if let Ok(Some(s)) = storage.get_item("player-state") {
                return serde_json::from_str::<PlayerState>(&s)
                    .unwrap_or_else(|_| PlayerState::new());
            }
        }
    }
    PlayerState::new()
}

#[derive(Clone, Debug)]
struct LaunchEffect {
    id: u64,
    upgrade_type: UpgradeType,
    start_time: f64,
    progress: f64,
    angle: f64,
    distance: f64,
    shape: String,
    color: String,
}

fn create_launch_effect(upgrade_type: UpgradeType, current_effects: &[LaunchEffect]) -> LaunchEffect {
    let id = js_sys::Date::now() as u64;
    let shape = match upgrade_type {
        UpgradeType::SolarSail => "plane",
        UpgradeType::PlasmaTether => "cylinder",
        UpgradeType::OrbitalMirror => "pyramid",
        UpgradeType::DysonCollector => "sphere",
        UpgradeType::QuantumArray => "cube",
        UpgradeType::StellarEngine => "complex",
    }.to_string();

    let color = match upgrade_type {
        UpgradeType::SolarSail => "#60a5fa",
        UpgradeType::PlasmaTether => "#f59e0b",
        UpgradeType::OrbitalMirror => "#10b981",
        UpgradeType::DysonCollector => "#8b5cf6",
        UpgradeType::QuantumArray => "#ef4444",
        UpgradeType::StellarEngine => "#06b6d4",
    }.to_string();

    let angle_offset = (current_effects.len() % 6) as f64 * (PI / 3.0);

    LaunchEffect {
        id,
        upgrade_type,
        start_time: js_sys::Date::now(),
        progress: 0.0,
        angle: angle_offset,
        distance: 120.0 + (current_effects.len() % 3) as f64 * 40.0,
        shape,
        color,
    }
}

#[component]
fn App() -> impl IntoView {
    let player_uuid = get_player_uuid();
    let mut initial_state = load_state();
    initial_state.calculate_offline_progress(js_sys::Date::now() as u64);

    let (state, set_state) = create_signal(initial_state);
    let (show_leaderboard, set_show_leaderboard) = create_signal(false);
    let (show_how_to_play, set_show_how_to_play) = create_signal(false);
    let (show_shared_view, set_show_shared_view) = create_signal(false);
    let (leaderboard_entries, set_leaderboard_entries) = create_signal(Vec::new());
    let (architect_message, set_architect_message) = create_signal(None as Option<String>);
    let (global_stats, set_global_stats) = create_signal(None as Option<api::GlobalStats>);
    let (launch_effects, set_launch_effects) = create_signal(Vec::new() as Vec<LaunchEffect>);

    let (last_purchase_time, set_last_purchase_time) = create_signal(0u64);

    create_effect(move |_| {
        if let Some(window) = web_sys::window() {
            if let Some(doc) = window.document() {
                if let Some(el) = doc.document_element() {
                    let _ = el.set_attribute("class", "theme-neutral");
                }
            }
        }
    });

    create_effect(move |_| {
        let interval = gloo_timers::callback::Interval::new(100, move || {
            let current_time = js_sys::Date::now() as u64;
            set_state.update(|s| {
                s.tick(0.1, current_time);
                s.last_sync_time = current_time;
            });
        });
        interval.forget();
    });

    create_effect(move |_| {
        let uuid = player_uuid.clone();
        let interval = gloo_timers::callback::Interval::new(2000, move || {
            let mut current_state = state.get();
            current_state.last_synced_total_energy = current_state.total_energy_generated;
            if let Some(window) = web_sys::window() {
                if let Ok(Some(storage)) = window.local_storage() {
                    if let Ok(serialized) = serde_json::to_string(&current_state) {
                        let _ = storage.set_item("player-state", &serialized);
                    }
                }
            }
            let uuid_clone = uuid.clone();
            let state_clone = current_state.clone();
            wasm_bindgen_futures::spawn_local(async move {
                let _ = api::sync_state(&uuid_clone, &state_clone).await;
            });
        });
        interval.forget();
    });

    create_effect(move |_| {
        let last_purchase = last_purchase_time.get();
        let interval = gloo_timers::callback::Interval::new(10000, move || {
            let current_time = js_sys::Date::now() as f64;
            let seconds_idle = (current_time as f64 - last_purchase as f64) / 1000.0;
            if last_purchase > 0 && seconds_idle >= 60.0 {
                let quip = generate_quip(QuipTrigger::Idle);
                set_architect_message.set(Some(quip));
            }
        });
        interval.forget();
    });

    create_effect(move |_| {
        let interval = gloo_timers::callback::Interval::new(16, move || {
            set_launch_effects.update(|effects| {
                let current_time = js_sys::Date::now();
                effects.retain_mut(|effect| {
                    let elapsed = (current_time - effect.start_time) / 1000.0;
                    if elapsed < 3.0 {
                        effect.progress = (elapsed / 3.0).min(1.0);
                        effect.angle += 0.02;
                        true
                    } else {
                        false
                    }
                });
            });
        });
        interval.forget();
    });

    let buy_upgrade = move |upgrade: UpgradeType| {
        let current_time = js_sys::Date::now() as u64;
        set_state.update(|s| {
            s.buy_upgrade(upgrade, current_time);
        });
        set_last_purchase_time.set(current_time);

        let current_effects = launch_effects.get();
        let new_effect = create_launch_effect(upgrade, &current_effects);
        set_launch_effects.update(|effects| {
            effects.push(new_effect);
        });

        let quip = generate_quip(QuipTrigger::Purchase(upgrade));
        set_architect_message.set(Some(quip));
    };

    view! {
        <div class="flex flex-col h-full bg-transparent text-app-text overflow-hidden transition-all duration-500 font-mono">
            <components::GameHeader state={state}/>

            <components::NeonOrb intensity={Some(state.get().energy_per_second() as i32)}/>

            <For
                each=move || launch_effects.get()
                key=|effect| effect.id
                children=move |effect| {
                    let progress = effect.progress;
                    let angle = effect.angle;
                    let distance = effect.distance;
                    let shape = effect.shape.clone();
                    let color = effect.color.clone();

                    let x = 50.0 + distance * angle.cos();
                    let y = 50.0 + distance * angle.sin();

                    let opacity = if progress < 0.2 {
                        progress * 5.0
                    } else if progress > 0.8 {
                        (1.0 - progress) * 5.0
                    } else {
                        1.0
                    };

                    let scale = 0.5 + progress * 0.5;

                    view! {
                        <div
                            class="absolute pointer-events-none z-20"
                            style=move || format!(
                                "left: {}%; top: {}%; transform: translate(-50%, -50%) scale({}); opacity: {};",
                                x, y, scale, opacity
                            )
                        >
                            {match shape.as_str() {
                                "plane" => view! {
                                    <div class="w-8 h-8 border-2 border-current rounded-lg transform rotate-45"
                                         style=format!("color: {};", color)>
                                        <div class="absolute inset-1 bg-current opacity-30 rounded"></div>
                                        <div class="absolute top-0 left-1/2 w-1 h-full bg-current opacity-50 transform -translate-x-1/2"></div>
                                    </div>
                                },
                                "cylinder" => view! {
                                    <div class="w-6 h-10 border-2 border-current rounded-lg relative"
                                         style=format!("color: {};", color)>
                                        <div class="absolute inset-0 bg-current opacity-20 rounded"></div>
                                        <div class="absolute top-1 left-1 right-1 h-1 bg-current opacity-60 rounded"></div>
                                        <div class="absolute bottom-1 left-1 right-1 h-1 bg-current opacity-60 rounded"></div>
                                    </div>
                                },
                                "pyramid" => view! {
                                    <div class="w-8 h-8 relative" style=format!("color: {};", color)>
                                        <div class="absolute inset-0 border-2 border-current transform rotate-45"></div>
                                        <div class="absolute inset-0 bg-current opacity-20 transform rotate-45"></div>
                                        <div class="absolute top-1/2 left-1/2 w-2 h-2 bg-current opacity-60 transform -translate-x-1/2 -translate-y-1/2"></div>
                                    </div>
                                },
                                "sphere" => view! {
                                    <div class="w-8 h-8 border-2 border-current rounded-full relative"
                                         style=format!("color: {};", color)>
                                        <div class="absolute inset-1 bg-current opacity-30 rounded-full"></div>
                                        <div class="absolute top-1 left-1 w-2 h-2 bg-current opacity-50 rounded-full"></div>
                                        <div class="absolute bottom-1 right-1 w-1 h-1 bg-current opacity-40 rounded-full"></div>
                                    </div>
                                },
                                "cube" => view! {
                                    <div class="w-8 h-8 border-2 border-current relative transform rotate-12"
                                         style=format!("color: {};", color)>
                                        <div class="absolute inset-0 bg-current opacity-20"></div>
                                        <div class="absolute top-1 left-1 w-3 h-3 border border-current bg-current opacity-30"></div>
                                    </div>
                                },
                                "complex" => view! {
                                    <div class="w-10 h-10 relative" style=format!("color: {};", color)>
                                        <div class="absolute inset-0 border-2 border-current rounded"></div>
                                        <div class="absolute top-1 left-1 w-2 h-2 border border-current rounded"></div>
                                        <div class="absolute top-1 right-1 w-2 h-2 border border-current rounded"></div>
                                        <div class="absolute bottom-1 left-1/2 w-3 h-1 bg-current opacity-60 transform -translate-x-1/2"></div>
                                        <div class="absolute top-1/2 left-1 w-1 h-3 bg-current opacity-40"></div>
                                    </div>
                                },
                                _ => view! { <div></div> }
                            }}

                            // Energy absorption trail
                            <For
                                each=move || 0..5
                                key=|i| *i
                                children=move |i| {
                                    let trail_angle = angle - (i as f64 * 0.2);
                                    let trail_distance = distance - (i as f64 * 8.0);
                                    let trail_x = 50.0 + trail_distance * trail_angle.cos();
                                    let trail_y = 50.0 + trail_distance * trail_angle.sin();
                                    let trail_opacity = opacity * (1.0 - i as f64 * 0.2);

                                    let trail_color = color.clone();
                                    view! {
                                        <div
                                            class="absolute w-1 h-1 rounded-full"
                                            style=move || format!(
                                                "left: {}%; top: {}%; background-color: {}; opacity: {};",
                                                trail_x, trail_y, trail_color, trail_opacity
                                            )
                                        ></div>
                                    }
                                }
                            />
                        </div>
                    }
                }
            />
            <div class="w-full flex flex-col items-center pb-8 shrink-0 relative z-10 pointer-events-auto gap-6">
                <components::UpgradePanel state={state} on_buy={Callback::from(buy_upgrade)}/>

                <div class="flex flex-col items-center gap-4">
                    <div class="flex gap-4 flex-wrap justify-center">
                        <button on:click={move |_| {
                            set_show_leaderboard.set(true);
                            wasm_bindgen_futures::spawn_local(async move {
                                match api::fetch_leaderboard().await {
                                    Ok(data) => set_leaderboard_entries.set(data.entries),
                                    Err(_) => {}
                                }
                            });
                        }} class="px-6 py-2 glass-pad text-sm font-bold tracking-widest hover:scale-105 transition-all text-theme-primary">"LEADERBOARD"</button>
                        <button on:click={move |_| {
                            set_show_shared_view.set(true);
                            wasm_bindgen_futures::spawn_local(async move {
                                match api::fetch_global_stats().await {
                                    Ok(data) => set_global_stats.set(Some(data)),
                                    Err(_) => {}
                                }
                            });
                        }} class="px-6 py-2 glass-pad text-sm font-bold tracking-widest hover:scale-105 transition-all text-theme-primary">"DYSPHERE"</button>
                        <button on:click={move |_| { set_show_how_to_play.set(true); }} class="px-6 py-2 glass-pad text-sm font-bold tracking-widest hover:scale-105 transition-all text-theme-primary">"HOW TO PLAY"</button>
                    </div>

                </div>
            </div>

            <Show when={move || architect_message.get().is_some()}>
                <components::ArchitectToast
                    message={architect_message.get().unwrap_or_default()}
                    on_close={Callback::from(move |_| set_architect_message.set(None))}
                />
            </Show>

            <Show when={move || show_leaderboard.get()}>
                <components::LeaderboardModal
                    entries={move || leaderboard_entries.get()}
                    on_close={Callback::from(move |_| set_show_leaderboard.set(false))}
                />
            </Show>

            <Show when={move || show_how_to_play.get()}>
                <components::HowToPlayModal on_close={Callback::from(move |_| set_show_how_to_play.set(false))} />
            </Show>

            <Show when={move || show_shared_view.get()}>
                <components::SharedViewModal
                    stats={global_stats.get().unwrap_or(api::GlobalStats {
                        total_energy: 0.0,
                        total_players: 0,
                        total_solar_sails: 0,
                        total_plasma_tethers: 0,
                        total_orbital_mirrors: 0,
                    })}
                    on_close={Callback::from(move |_| set_show_shared_view.set(false))}
                />
            </Show>
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
    mount_to(
        root.unchecked_into::<web_sys::HtmlElement>(),
        || view! { <App/> },
    )
}
