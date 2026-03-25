//! Asteroids arena — ship, stars, bullets.

use gloo_timers::callback::Interval;
use leptos::*;
use web_sys::KeyboardEvent;
use super::physics::{Ship, THRUST, ROTATE, DRAG};
use std::cell::Cell;

#[derive(Clone, Copy)]
struct Bullet { x: f64, y: f64, vx: f64, vy: f64 }

fn handle_key(ev: web_sys::KeyboardEvent, set_keys: WriteSignal<(bool,bool,bool,bool,bool)>, down: bool) {
    match ev.key().as_str() {
        "ArrowUp" | "w" | "W" => set_keys.update(|k| k.0 = down),
        "ArrowDown" | "s" | "S" => set_keys.update(|k| k.1 = down),
        "ArrowLeft" | "a" | "A" => set_keys.update(|k| k.2 = down),
        "ArrowRight" | "d" | "D" => set_keys.update(|k| k.3 = down),
        " " => set_keys.update(|k| k.4 = down),
        _ => {}
    }
    if down { ev.prevent_default(); }
}

#[component]
pub fn AsteroidsArena() -> impl IntoView {
    let (keys, set_keys) = create_signal((false, false, false, false, false));
    let (ship, set_ship) = create_signal(Ship { x: 50.0, y: 50.0, ..Default::default() });
    let (bullets, set_bullets) = create_signal(Vec::<Bullet>::new());
    let started = Cell::new(false);

    // Static starfield
    let star_list: Vec<(f64, f64)> = (0..80)
        .map(|_| (js_sys::Math::random() * 100.0, js_sys::Math::random() * 100.0))
        .collect();

    create_effect(move |_| {
        if started.get() { return; }
        started.set(true);
        Interval::new(16, move || {
            let (up, down, left, right, fire) = keys.get();
            set_ship.update(|s| {
                if left  { s.angle -= ROTATE; }
                if right { s.angle += ROTATE; }
                if up {
                    s.vx += s.angle.cos() * THRUST;
                    s.vy += s.angle.sin() * THRUST;
                }
                if down {
                    s.vx -= s.angle.cos() * THRUST * 0.4;
                    s.vy -= s.angle.sin() * THRUST * 0.4;
                }
                s.vx *= DRAG;
                s.vy *= DRAG;
                s.x = (s.x + s.vx + 100.0) % 100.0;
                s.y = (s.y + s.vy + 100.0) % 100.0;
            });
            // Bullets
            if fire {
                let s = ship.get();
                set_bullets.update(|b| {
                    let speed = 0.8;
                    b.push(Bullet {
                        x: s.x, y: s.y,
                        vx: s.angle.cos() * speed,
                        vy: s.angle.sin() * speed,
                    });
                });
            }
            set_bullets.update(|b| {
                for bullet in b.iter_mut() {
                    bullet.x = (bullet.x + bullet.vx + 100.0) % 100.0;
                    bullet.y = (bullet.y + bullet.vy + 100.0) % 100.0;
                }
                b.retain(|b| b.x > 0.0 || b.y > 0.0);
                if b.len() > 20 { *b = b[b.len()-20..].to_vec(); }
            });
        }).forget();
    });

    let ship_view = move || {
        let s = ship.get();
        let fx = s.angle.cos() * 4.0;
        let fy = s.angle.sin() * 4.0;
        let thrust = if keys.get().0 { 1.0 } else { 0.0 };
        view! {
            <g transform={format!("translate({},{}) rotate({}deg)", s.x, s.y, s.angle.to_degrees())}>
                <polygon points="-2,-1.5 4,0 -2,1.5" fill="#ef4444" stroke="#ff6b6b" stroke-width="0.2"/>
                <line x1="-2" y1="0" x2={fx} y2={fy}
                    stroke="#f97316" stroke-width="0.4" opacity={thrust} style={format!("opacity:{}", thrust)}/>
            </g>
        }.into_view()
    };

    let bullets_view = move || {
        bullets.get().iter().map(|b| {
            view! { <circle cx={b.x} cy={b.y} r="0.3" fill="#fef08a"/> }.into_view()
        }).collect::<Vec<_>>()
    };

    let stars_view = move || {
        star_list.iter().map(|(x, y)| {
            view! { <circle cx={*x} cy={*y} r="0.1" fill="white" opacity="0.5"/> }.into_view()
        }).collect::<Vec<_>>()
    };

    let sk = set_keys.clone();
    let su = set_keys.clone();

    view! {
        <div class="relative w-full h-full overflow-hidden bg-black" tabindex="0"
            on:keydown={move |ev: KeyboardEvent| handle_key(ev, su, true)}
            on:keyup={move |ev: KeyboardEvent| handle_key(ev, sk, false)}>
            <svg viewBox="0 0 100 100" preserveAspectRatio="none" class="absolute inset-0 w-full h-full">
                {stars_view}
                {bullets_view}
                {ship_view}
            </svg>
            <div class="absolute bottom-2 left-2 text-xs text-gray-600 font-mono">"WASD · SPACE"</div>
        </div>
    }
}
