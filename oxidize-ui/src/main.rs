mod api;
mod components;

use leptos::*;
use oxidize_engine::architect::{generate_quip, QuipTrigger};
use oxidize_engine::factions::get_sync_interval;
use oxidize_engine::{Faction, PlayerState, UpgradeType};
use wasm_bindgen::JsCast;

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
                    .unwrap_or_else(|_| PlayerState::new(Faction::Orange));
            }
        }
    }
    PlayerState::new(Faction::Orange)
}

#[component]
fn App() -> impl IntoView {
    let player_uuid = get_player_uuid();
    let mut initial_state = load_state();
    initial_state.calculate_offline_progress(js_sys::Date::now() as u64);

    let (state, set_state) = create_signal(initial_state);
    let (theme, set_theme) = create_signal("orange".to_string());
    let (show_leaderboard, set_show_leaderboard) = create_signal(false);
    let (show_how_to_play, set_show_how_to_play) = create_signal(false);
    let (show_shared_view, set_show_shared_view) = create_signal(false);
    let (leaderboard_entries, set_leaderboard_entries) = create_signal(Vec::new());
    let (architect_message, set_architect_message) = create_signal(None as Option<String>);
    let (global_stats, set_global_stats) = create_signal(None as Option<api::GlobalStats>);
    let (show_achievements, set_show_achievements) = create_signal(false);
    let (achievement_list, set_achievement_list) = create_signal(Vec::new());

    let (last_purchase_time, set_last_purchase_time) = create_signal(0u64);
    let (architect_faction, set_architect_faction) = create_signal(Faction::Orange);

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
        let sync_interval = get_sync_interval(architect_faction.get());
        let interval = gloo_timers::callback::Interval::new(sync_interval, move || {
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
                match api::sync_state(&uuid_clone, &state_clone).await {
                    Ok(response) => {
                        if !response.newly_unlocked_achievements.is_empty() {
                            set_achievement_list.set(response.newly_unlocked_achievements);
                            set_show_achievements.set(true);
                        }
                    }
                    Err(_) => {}
                }
            });
        });
        interval.forget();
    });

    create_effect(move |_| {
        let last_purchase = last_purchase_time.get();
        let faction = architect_faction.get();
        let interval = gloo_timers::callback::Interval::new(10000, move || {
            let current_time = js_sys::Date::now() as u64;
            let seconds_idle = (current_time - last_purchase) as f64 / 1000.0;
            if last_purchase > 0 && seconds_idle >= 60.0 {
                let quip = generate_quip(faction, QuipTrigger::Idle);
                set_architect_message.set(Some(quip));
            }
        });
        interval.forget();
    });

    let buy_upgrade = move |upgrade: UpgradeType| {
        let current_time = js_sys::Date::now() as u64;
        set_state.update(|s| {
            s.buy_upgrade(upgrade, current_time);
        });
        set_last_purchase_time.set(current_time);
        let faction = architect_faction.get();
        let quip = generate_quip(faction, QuipTrigger::Purchase(upgrade));
        set_architect_message.set(Some(quip));
    };

    view! {
        <div class="flex flex-col h-full bg-transparent text-app-text overflow-hidden transition-all duration-500 font-mono">
            <components::GameHeader state={state}/>

            <components::NeonOrb intensity={Some(state.get().energy_per_second() as i32)}/>

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
                    <div class="flex gap-4 mt-2">
                        <button on:click={move |_| { set_theme.set("red".to_string()); set_architect_faction.set(Faction::Red); }} class="w-10 h-10 rounded-xl bg-red-500 hover:scale-110 transition-all"></button>
                        <button on:click={move |_| { set_theme.set("orange".to_string()); set_architect_faction.set(Faction::Orange); }} class="w-10 h-10 rounded-xl bg-orange-500 hover:scale-110 transition-all"></button>
                        <button on:click={move |_| { set_theme.set("yellow".to_string()); set_architect_faction.set(Faction::Yellow); }} class="w-10 h-10 rounded-xl bg-yellow-400 hover:scale-110 transition-all"></button>
                        <button on:click={move |_| { set_theme.set("green".to_string()); set_architect_faction.set(Faction::Green); }} class="w-10 h-10 rounded-xl bg-green-500 hover:scale-110 transition-all"></button>
                        <button on:click={move |_| { set_theme.set("blue".to_string()); set_architect_faction.set(Faction::Blue); }} class="w-10 h-10 rounded-xl bg-blue-500 hover:scale-110 transition-all"></button>
                        <button on:click={move |_| { set_theme.set("purple".to_string()); set_architect_faction.set(Faction::Purple); }} class="w-10 h-10 rounded-xl bg-purple-500 hover:scale-110 transition-all"></button>
                    </div>
                </div>
            </div>

            <Show when={move || architect_message.get().is_some()}>
                <components::ArchitectToast
                    message={architect_message.get().unwrap_or_default()}
                    on_close={Callback::from(move |_| set_architect_message.set(None))}
                />
            </Show>

            <Show when={move || show_achievements.get()}>
                <components::AchievementToast
                    achievements={achievement_list.get()}
                    on_close={Callback::from(move |_| set_show_achievements.set(false))}
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
