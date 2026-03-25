//! Saturn Asteroids arena component.

use gloo_timers::callback::Interval;
use leptos::*;
use web_sys::KeyboardEvent;
use super::entity::EntityStyle;
use super::init::initial_entities;
use super::keys::{create_key_tracker, Keys};
use super::physics::{check_collisions, update_entities, update_ship, ShipState, FIRE_COOLDOWN};
use std::cell::Cell;

fn handle_key(ev: KeyboardEvent, set_keys: WriteSignal<Keys>, down: bool) {
    let key = ev.key();
    set_keys.update(|k| match key.as_str() {
        "ArrowUp" | "w" | "W" => k.up = down,
        "ArrowDown" | "s" | "S" => k.down = down,
        "ArrowLeft" | "a" | "A" => k.left = down,
        "ArrowRight" | "d" | "D" => k.right = down,
        " " => k.fire = down,
        _ => {}
    });
    if down { ev.prevent_default(); }
}

#[component]
pub fn AsteroidsArena(state_set: WriteSignal<oxidize_engine::PlayerState>) -> impl IntoView {
    let (keys, set_keys) = create_key_tracker();
    let (ship, set_ship) = create_signal(ShipState { x: 50.0, y: 50.0, ..Default::default() });
    let (entities, set_entities) = create_signal(initial_entities());
    let (firing, set_firing) = create_signal(false);
    let started: Cell<bool> = Cell::new(false);

    // Start the game loop once
    create_effect(move |_| {
        if started.get() { return; }
        started.set(true);
        let tick = Interval::new(16, move || {
            let k = keys.get();
            set_ship.update(|sp| {
                let mut ns = ShipState {
                    x: sp.x, y: sp.y, vx: sp.vx, vy: sp.vy,
                    angle: sp.angle, firing: sp.firing, fire_timer: sp.fire_timer,
                };
                update_ship(&mut ns, &k, 0.016);
                sp.x = ns.x; sp.y = ns.y;
                sp.vx = ns.vx; sp.vy = ns.vy;
                sp.angle = ns.angle;
                sp.firing = ns.firing; sp.fire_timer = ns.fire_timer;
            });
            set_firing.set(ship.get().firing);
            set_entities.update(|ents| update_entities(ents, 0.016));
            let collected = check_collisions(&ship.get(), &entities.get());
            if collected > 0.0 {
                state_set.update(|st| { st.energy += collected; st.total_energy_generated += collected; });
            }
        });
        tick.forget();
    });

    let entity_views = move || {
        entities.get().iter().filter_map(|e| {
            if e.collected { return None; }
            let (col, op) = match e.style {
                EntityStyle::Moon => ("#d4a574", 0.9),
                EntityStyle::Debris => ("#9ca3af", 0.7),
            };
            Some(view! { <circle cx={e.x} cy={e.y} r={e.radius} fill={col} opacity={op} filter="url(#glow)"/> }.into_view())
        }).collect::<Vec<_>>()
    };

    let fire_view = move || {
        if firing.get() {
            view! { <circle cx={ship.get().x} cy={ship.get().y} r={FIRE_COOLDOWN * 30.0} fill="none" stroke="#f97316" stroke-width="0.15" opacity="0.4" stroke-dasharray="0.5,0.5"/> }.into_view()
        } else { view! { <g></g> }.into_view() }
    };

    let ship_view = move || {
        let s = ship.get();
        let fx = s.angle.cos() * 3.0;
        let fy = s.angle.sin() * 3.0;
        let thrust = if keys.get().up { "inline" } else { "none" };
        view! {
            <g transform={format!("translate({},{}) rotate({}deg)", s.x, s.y, s.angle.to_degrees())}>
                <polygon points="-1.5,-1 3.5,0 -1.5,1" fill="#ef4444"/>
                <line x1="-1.5" y1="0" x2={fx} y2={fy} stroke="#f97316" stroke-width="0.4" opacity="0.7" display={thrust}/>
            </g>
        }.into_view()
    };

    let sk = set_keys;
    let su = set_keys;

    view! {
        <div class="relative w-full h-full overflow-hidden bg-black"
            tabindex="0"
            on:keydown={move |ev: KeyboardEvent| { handle_key(ev, su, true); }}
            on:keyup={move |ev: KeyboardEvent| { handle_key(ev, sk, false); }}>
            <svg viewBox="0 0 100 100" preserveAspectRatio="xMidYMid slice" class="absolute inset-0 w-full h-full">
                <defs>
                    <filter id="glow">
                        <feGaussianBlur stdDeviation="0.4" result="blur"/>
                        <feMerge><feMergeNode in="blur"/><feMergeNode in="SourceGraphic"/></feMerge>
                    </filter>
                    <radialGradient id="sg">
                        <stop offset="0%" stop-color="#fde047"/>
                        <stop offset="100%" stop-color="#ca8a04"/>
                    </radialGradient>
                </defs>
                <circle cx="50" cy="50" r="5" fill="url(#sg)"/>
                <ellipse cx="50" cy="50" rx="9" ry="2.5" fill="none" stroke="#d4a574" stroke-width="0.4" opacity="0.5" transform="rotate(-15,50,50)"/>
                <ellipse cx="50" cy="50" rx="7.5" ry="2" fill="none" stroke="#c0a060" stroke-width="0.2" opacity="0.3" transform="rotate(-15,50,50)"/>
                {entity_views}
                {fire_view}
                {ship_view}
            </svg>
            <div class="absolute bottom-2 left-2 text-xs text-gray-600 font-mono">"WASD · SPACE collect"</div>
        </div>
    }
}
