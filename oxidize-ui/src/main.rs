use gloo_timers::callback::Interval;
use leptos::*;
use wasm_bindgen::JsCast;
use std::cell::Cell;

const THRUST: f64 = 0.15;
const ROTATE: f64 = 0.08;
const DRAG: f64 = 0.99;

#[derive(Default, Clone, Copy)]
struct Ship { x: f64, y: f64, vx: f64, vy: f64, angle: f64 }

#[derive(Clone, Copy)]
struct Bullet { x: f64, y: f64, vx: f64, vy: f64 }

fn main() {
    console_error_panic_hook::set_once();
    let document = web_sys::window().unwrap().document().unwrap();
    let root = document.get_element_by_id("root").unwrap();
    leptos::mount_to(root.dyn_into::<web_sys::HtmlElement>().unwrap(), || view! { <App/> });
}

#[component]
fn App() -> impl IntoView {
    let (keys, set_keys) = create_signal((false, false, false, false, false));
    let (ship, set_ship) = create_signal(Ship { x: 50.0, y: 50.0, ..Default::default() });
    let (bullets, set_bullets) = create_signal(Vec::<Bullet>::new());
    let started = Cell::new(false);

    let stars: Vec<(f64, f64)> = (0..100)
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
            if fire {
                let s = ship.get();
                set_bullets.update(|b| {
                    b.push(Bullet {
                        x: s.x, y: s.y,
                        vx: s.angle.cos() * 0.8,
                        vy: s.angle.sin() * 0.8,
                    });
                });
            }
            set_bullets.update(|b| {
                for bullet in b.iter_mut() {
                    bullet.x = (bullet.x + bullet.vx + 100.0) % 100.0;
                    bullet.y = (bullet.y + bullet.vy + 100.0) % 100.0;
                }
                if b.len() > 30 { *b = b[b.len()-30..].to_vec(); }
            });
        }).forget();
    });

    let sk = set_keys.clone();
    let su = set_keys.clone();

    view! {
        <div class="w-screen h-screen bg-black overflow-hidden" tabindex="0"
            on:keydown={move |ev: web_sys::KeyboardEvent| {
                match ev.key().as_str() {
                    "ArrowUp" | "w" | "W" => set_keys.update(|k| k.0 = true),
                    "ArrowDown" | "s" | "S" => set_keys.update(|k| k.1 = true),
                    "ArrowLeft" | "a" | "A" => set_keys.update(|k| k.2 = true),
                    "ArrowRight" | "d" | "D" => set_keys.update(|k| k.3 = true),
                    " " => set_keys.update(|k| k.4 = true),
                    _ => {}
                }
                ev.prevent_default();
            }}
            on:keyup={move |ev: web_sys::KeyboardEvent| {
                match ev.key().as_str() {
                    "ArrowUp" | "w" | "W" => set_keys.update(|k| k.0 = false),
                    "ArrowDown" | "s" | "S" => set_keys.update(|k| k.1 = false),
                    "ArrowLeft" | "a" | "A" => set_keys.update(|k| k.2 = false),
                    "ArrowRight" | "d" | "D" => set_keys.update(|k| k.3 = false),
                    " " => set_keys.update(|k| k.4 = false),
                    _ => {}
                }
            }}>
            <svg viewBox="0 0 100 100" preserveAspectRatio="none" class="w-full h-full">
                {stars.iter().map(|(x, y)| view! { <circle cx={*x} cy={*y} r="0.08" fill="white" opacity="0.4"/> }).collect::<Vec<_>>()}
                {bullets.get().iter().map(|b| view! { <circle cx={b.x} cy={b.y} r="0.2" fill="#fff"/> }).collect::<Vec<_>>()}
                <g transform={format!("translate({},{}) rotate({}deg)",
                    ship.get().x, ship.get().y, ship.get().angle.to_degrees())}>
                    <polygon points="-2,-1.5 4,0 -2,1.5" fill="none" stroke="#0f0" stroke-width="0.15"/>
                </g>
            </svg>
        </div>
    }
}
