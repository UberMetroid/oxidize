use gloo_timers::callback::Interval;
use wasm_bindgen::JsCast;
use std::cell::RefCell;
use leptos::SignalSet;

const THRUST: f64 = 0.05;
const BRAKE: f64 = 0.03;
const ROTATE: f64 = 3.0;
const DRAG: f64 = 0.998;
const MISSILE_SPEED: f64 = 1.0;
const MAX_CHARGE: f64 = 3.0;
const CHARGE_POWER: f64 = 1.5;
const CHARGE_RADIUS: f64 = 1.5;
const NORMAL_LIFE: f64 = 0.25;
const CHARGED_LIFE: f64 = 0.25;

// ── Entities ─────────────────────────────────────────────────────────────────

struct Missile { x: f64, y: f64, vx: f64, vy: f64, age: f64, charged: bool }

struct Particle { x: f64, y: f64, vx: f64, vy: f64, age: f64, max_age: f64, color: &'static str, size: f64 }

struct SpawnRing { x: f64, y: f64, age: f64, max_age: f64 }

#[derive(Clone)]
struct Asteroid {
    x: f64, y: f64,
    vx: f64, vy: f64,
    rot: f64,       // current rotation degrees
    rot_v: f64,     // rotation velocity deg/s
    size: u8,       // 0=large, 1=medium, 2=small
    verts: Vec<(f64,f64)>, // local-space polygon vertices
    pts_str: String, // cached points string (computed once on creation)
}

impl Asteroid {
    fn radius(&self) -> f64 {
        match self.size { 0 => 6.0, 1 => 3.0, _ => 1.5 }
    }

    fn new_large(x: f64, y: f64) -> Self {
        let verts = Self::gen_verts();
        let pts_str = verts.iter()
            .map(|(x,y)| format!("{:.2},{:.2}", x, y))
            .collect::<Vec<_>>().join(" ");
        let angle = js_sys::Math::random() * 360.0;
        let spd = 0.05 + js_sys::Math::random() * 0.1;
        Self { x, y, vx: angle.to_radians().cos()*spd, vy: angle.to_radians().sin()*spd,
               rot: js_sys::Math::random()*360.0, rot_v: (js_sys::Math::random()-0.5)*40.0,
               size: 0, verts, pts_str }
    }

    fn split(&self) -> Option<(Asteroid, Asteroid)> {
        if self.size >= 2 { return None; }
        let ns = self.size + 1;
        let spd = 0.08 + js_sys::Math::random() * 0.12;
        let a1 = js_sys::Math::random() * 360.0;
        let a2 = a1 + 120.0 + js_sys::Math::random() * 120.0;
        let va = Self::gen_verts();
        let vb = Self::gen_verts();
        let pts_a = va.iter().map(|(x,y)| format!("{:.2},{:.2}",x,y)).collect::<Vec<_>>().join(" ");
        let pts_b = vb.iter().map(|(x,y)| format!("{:.2},{:.2}",x,y)).collect::<Vec<_>>().join(" ");
        Some((
            Self { x: self.x, y: self.y, vx: a1.to_radians().cos()*spd, vy: a1.to_radians().sin()*spd, rot: js_sys::Math::random()*360.0, rot_v: (js_sys::Math::random()-0.5)*60.0, size: ns, verts: va, pts_str: pts_a },
            Self { x: self.x, y: self.y, vx: a2.to_radians().cos()*spd, vy: a2.to_radians().sin()*spd, rot: js_sys::Math::random()*360.0, rot_v: (js_sys::Math::random()-0.5)*60.0, size: ns, verts: vb, pts_str: pts_b },
        ))
    }

    fn gen_verts() -> Vec<(f64,f64)> {
        let n = 8 + (js_sys::Math::random() * 4.0) as i32;
        (0..n).map(|i| {
            let angle = (i as f64 / n as f64) * std::f64::consts::TAU;
            let r = 1.0 * (0.7 + js_sys::Math::random() * 0.6);
            (angle.cos() * r, angle.sin() * r)
        }).collect()
    }
}

// ── Game State ────────────────────────────────────────────────────────────────

struct GameState {
    ship: (f64, f64, f64, f64, f64), // x, y, vx, vy, angle_deg
    missiles: Vec<Missile>,
    asteroids: Vec<Asteroid>,
    particles: Vec<Particle>,
    spawn_rings: Vec<SpawnRing>,
    keys: (bool, bool, bool, bool),   // up, down, left, right
    charging: bool,
    charge: f64,
    score: u32,
    started: bool,
    spawn_flash: f64, // seconds of spawn flash effect (0 = not spawning)
    level: u32,
    warp_transition: f64, // seconds of warp/level-clear transition (0 = not warping)
}

fn spawn_wave(state: &mut GameState, count: usize) {
    for _ in 0..count {
        let x = 5.0 + js_sys::Math::random() * 90.0;
        let y = 5.0 + js_sys::Math::random() * 90.0;
        // Keep away from center spawn zone (ship can spawn 20-80 range)
        let dx = x - 50.0;
        let dy = y - 50.0;
        if (dx*dx + dy*dy) < 225.0 { continue; } // exclude within 15 units of center
        state.asteroids.push(Asteroid::new_large(x, y));
    }
}

fn find_safe_spawn(asteroids: &[(f64, f64)], min_dist: f64) -> (f64, f64) {
    let attempts = 100;
    for _ in 0..attempts {
        let x = 20.0 + js_sys::Math::random() * 60.0;
        let y = 20.0 + js_sys::Math::random() * 60.0;
        let mut safe = true;
        for &(ax, ay) in asteroids {
            let dx = x - ax;
            let dy = y - ay;
            if (dx*dx + dy*dy) < min_dist * min_dist {
                safe = false;
                break;
            }
        }
        if safe { return (x, y); }
    }
    // Fallback: try center area anyway
    (50.0, 50.0)
}

fn spawn_explosion(particles: &mut Vec<Particle>, x: f64, y: f64, color: &'static str, count: usize, speed: f64, max_age: f64, size: f64) {
    for _ in 0..count {
        let angle = js_sys::Math::random() * std::f64::consts::TAU;
        let spd = speed * (0.5 + js_sys::Math::random() * 0.8);
        particles.push(Particle {
            x, y,
            vx: angle.cos() * spd,
            vy: angle.sin() * spd,
            age: 0.0,
            max_age,
            color,
            size,
        });
    }
}

// ── Main ──────────────────────────────────────────────────────────────────────

fn main() {
    let document = web_sys::window().unwrap().document().unwrap();
    let root = document.get_element_by_id("root").expect("no #root");
    leptos::mount_to(root.dyn_into::<web_sys::HtmlElement>().unwrap(), || leptos::view! { <Game/> });
}

#[leptos::component]
fn Game() -> impl leptos::IntoView {
    let stars: String = (0..150).map(|_| {
        let x = js_sys::Math::random() * 100.0;
        let y = js_sys::Math::random() * 100.0;
        format!("<circle cx=\"{:.1}\" cy=\"{:.1}\" r=\"0.1\" fill=\"#fff\" opacity=\"0.5\"/>", x, y)
    }).collect::<Vec<_>>().join("");

    let mut init_state = GameState {
        ship: (50.0, 50.0, 0.0, 0.0, 90.0),
        missiles: Vec::new(),
        asteroids: Vec::new(),
        particles: Vec::new(),
        keys: (false, false, false, false),
        charging: false,
        charge: 0.0,
        score: 0,
        started: false,
        spawn_flash: 0.0,
        spawn_rings: Vec::new(),
        level: 1,
        warp_transition: 0.0,
    };
    spawn_wave(&mut init_state, 5); // Level 1

    let state = std::sync::Arc::new(RefCell::new(init_state));

    // Reactive score signal
    let (score_sig, set_score) = leptos::create_signal(0u32);
    let (level_sig, set_level) = leptos::create_signal(1u32);

    // DOM handles
    let ship_el   = std::sync::Arc::new(RefCell::new(None::<web_sys::Element>));
    let missile_g = std::sync::Arc::new(RefCell::new(None::<web_sys::Element>));
    let charge_el = std::sync::Arc::new(RefCell::new(None::<web_sys::Element>));
    let asteroid_g= std::sync::Arc::new(RefCell::new(None::<web_sys::Element>));
    let particle_g= std::sync::Arc::new(RefCell::new(None::<web_sys::Element>));
    let rings_g   = std::sync::Arc::new(RefCell::new(None::<web_sys::Element>));
    let prompt_el = std::sync::Arc::new(RefCell::new(None::<web_sys::Element>));

    // ── Keyboard ─────────────────────────────────────────────────────────────
    let s2 = state.clone();
    let ss2 = set_score.clone();
    let sl2 = set_level.clone();
    let down_cb = wasm_bindgen::closure::Closure::<dyn FnMut(_)>::wrap(Box::new(move |ev: web_sys::KeyboardEvent| {
        match ev.key().as_str() {
            "ArrowUp"    => { s2.borrow_mut().keys.0 = true; ev.prevent_default(); }
            "ArrowDown"  => { s2.borrow_mut().keys.1 = true; ev.prevent_default(); }
            "ArrowLeft"  => { s2.borrow_mut().keys.2 = true; ev.prevent_default(); }
            "ArrowRight" => { s2.borrow_mut().keys.3 = true; ev.prevent_default(); }
            " " => {
                if !ev.repeat() {
                    let mut s = s2.borrow_mut();
                    if !s.started {
                        // Start or restart game
                        s.started = true;
                        let safe = find_safe_spawn(
                            &s.asteroids.iter().map(|a| (a.x, a.y)).collect::<Vec<_>>(),
                            12.0,
                        );
                        s.ship = (safe.0, safe.1, 0.0, 0.0, 90.0);
                        s.missiles.clear();
                        s.particles.clear();
                        s.asteroids.clear();
                        s.score = 0;
                        s.level = 1;
                        s.spawn_flash = 1.5; // teleport flash duration
                        s.warp_transition = 0.0;
                        ss2.set(0);
                        sl2.set(1);
                        // Spawn protection: clear asteroids near center before wave spawns
                        s.asteroids.clear();
                        spawn_wave(&mut s, 5); // Level 1
                        // Spawn rings (multiple staggered portal rings) — arrival portal at ship position
                        for i in 0..5 {
                            s.spawn_rings.push(SpawnRing {
                                x: safe.0, y: safe.1,
                                age: 0.0,
                                max_age: 0.5 + (i as f64 * 0.1),
                            });
                        }
                        // Warp-in particles: spiral inward then scatter, cyan/white
                        for i in 0..20 {
                            let base_angle = (i as f64 / 20.0) * std::f64::consts::TAU;
                            // Start from a wide ring, spiral inward
                            let start_r = 4.0 + (i as f64 / 20.0) * 2.0;
                            let x = safe.0 + base_angle.cos() * start_r;
                            let y = safe.1 + base_angle.sin() * start_r;
                            // Move toward center (inward spiral)
                            let spd = 0.4 + (i as f64 / 20.0) * 0.3;
                            let color = if i % 4 == 0 { "#00ffcc" }
                                        else if i % 4 == 1 { "#ffffff" }
                                        else if i % 4 == 2 { "#88ffee" }
                                        else { "#44ffdd" };
                            s.particles.push(Particle {
                                x, y,
                                vx: -base_angle.cos() * spd,
                                vy: -base_angle.sin() * spd,
                                age: 0.0,
                                max_age: 0.5 + (i as f64 / 20.0) * 0.3,
                                color,
                                size: 0.18,
                            });
                        }
                        // Bright flash particles burst outward briefly at moment of arrival
                        spawn_explosion(&mut s.particles, safe.0, safe.1, "#00ffcc", 8, 0.6, 0.3, 0.1);
                    } else if !s.charging {
                        s.charging = true;
                        s.charge = 0.0;
                    }
                }
                ev.prevent_default();
            }
            _ => {}
        }
    }));

    let s3 = state.clone();
    let up_cb = wasm_bindgen::closure::Closure::<dyn FnMut(_)>::wrap(Box::new(move |ev: web_sys::KeyboardEvent| {
        match ev.key().as_str() {
            "ArrowUp"    => { s3.borrow_mut().keys.0 = false; }
            "ArrowDown"  => { s3.borrow_mut().keys.1 = false; }
            "ArrowLeft"  => { s3.borrow_mut().keys.2 = false; }
            "ArrowRight" => { s3.borrow_mut().keys.3 = false; }
            " " => { s3.borrow_mut().charging = false; }
            _ => {}
        }
    }));

    let window = web_sys::window().unwrap();
    window.add_event_listener_with_callback("keydown", &down_cb.as_ref().unchecked_ref()).ok();
    window.add_event_listener_with_callback("keyup",   &up_cb.as_ref().unchecked_ref()).ok();
    down_cb.forget();
    up_cb.forget();

    // ── Game loop ───────────────────────────────────────────────────────────
    let s4  = state.clone();
    let se  = ship_el.clone();
    let mg  = missile_g.clone();
    let ce  = charge_el.clone();
    let ag  = asteroid_g.clone();
    let pg  = particle_g.clone();
    let rg  = rings_g.clone();
    let pr  = prompt_el.clone();
    let ss4 = set_score.clone();
    let sl4 = set_level.clone();

    Interval::new(16, move || {
        let dt = 0.016;

        // Lazy DOM lookup
        for (arc_el, id) in [(se.clone(), "#ship"), (mg.clone(), "#missiles"),
                              (ce.clone(), "#charge"), (ag.clone(), "#asteroids"),
                              (pg.clone(), "#particles"), (rg.clone(), "#spawn-rings"),
                              (pr.clone(), "#prompt")] {
            let id = id.to_string();
            let mut el = arc_el.borrow_mut();
            if el.is_none() {
                if let Ok(Some(e)) = web_sys::window().unwrap().document().unwrap().query_selector(&id) {
                    *el = Some(e);
                }
            }
        }

        // ── Spawn flash + warp countdown (always ticks) ─────────────────────
        {
            let mut s = s4.borrow_mut();
            if s.spawn_flash > 0.0 {
                s.spawn_flash = (s.spawn_flash - dt).max(0.0);
            }
            if s.warp_transition > 0.0 {
                s.warp_transition = (s.warp_transition - dt).max(0.0);
                if s.warp_transition == 0.0 && s.started {
                    // Warp complete — advance to next level
                    s.level += 1;
                    sl4.set(s.level);
                    s.spawn_flash = 1.5;
                    let count = 5 + (s.level as usize) * 2;
                    // Spawn wave FIRST so we know where asteroids will be
                    spawn_wave(&mut s, count);
                    // Then find safe spawn considering new asteroids
                    let safe = find_safe_spawn(
                        &s.asteroids.iter().map(|a| (a.x, a.y)).collect::<Vec<_>>(),
                        12.0,
                    );
                    s.ship = (safe.0, safe.1, 0.0, 0.0, s.ship.4);
                    // Arrival effect at new position
                    for i in 0..5 {
                        s.spawn_rings.push(SpawnRing { x: safe.0, y: safe.1, age: 0.0, max_age: 0.5 + (i as f64 * 0.1) });
                    }
                    for i in 0..20 {
                        let base_angle = (i as f64 / 20.0) * std::f64::consts::TAU;
                        let start_r = 4.0 + (i as f64 / 20.0) * 2.0;
                        let x = safe.0 + base_angle.cos() * start_r;
                        let y = safe.1 + base_angle.sin() * start_r;
                        let spd = 0.4 + (i as f64 / 20.0) * 0.3;
                        let color = if i % 4 == 0 { "#00ffcc" } else if i % 4 == 1 { "#ffffff" } else if i % 4 == 2 { "#88ffee" } else { "#44ffdd" };
                        s.particles.push(Particle { x, y, vx: -base_angle.cos() * spd, vy: -base_angle.sin() * spd, age: 0.0, max_age: 0.5 + (i as f64 / 20.0) * 0.3, color, size: 0.18 });
                    }
                    spawn_explosion(&mut s.particles, safe.0, safe.1, "#00ffcc", 8, 0.6, 0.3, 0.1);
                }
            }
        }

        // ── Ship physics + charge ─────────────────────────────────────────────
        let (rad, charge_level) = {
            let mut s = s4.borrow_mut();
            if !s.started {
                // Frozen on title screen
                (s.ship.4.to_radians(), 0.0)
            } else {
                let (up, down, left, right) = s.keys;
                if left  { s.ship.4 -= ROTATE; }
                if right { s.ship.4 += ROTATE; }
                let a = s.ship.4.to_radians();
                if up {
                    s.ship.2 += a.cos() * THRUST;
                    s.ship.3 += a.sin() * THRUST;
                }
                if down {
                    s.ship.2 -= a.cos() * BRAKE;
                    s.ship.3 -= a.sin() * BRAKE;
                }
                s.ship.2 *= DRAG;
                s.ship.3 *= DRAG;
                s.ship.0 = (s.ship.0 + s.ship.2 + 100.0) % 100.0;
                s.ship.1 = (s.ship.1 + s.ship.3 + 100.0) % 100.0;

                if s.charging {
                    s.charge = (s.charge + dt).min(MAX_CHARGE);
                }
                let level = s.charge;
                if !s.charging && level > 0.0 {
                    let tip_x = s.ship.0 + a.cos() * 3.5;
                    let tip_y = s.ship.1 + a.sin() * 3.5;
                    let power = level / MAX_CHARGE;
                    let speed = MISSILE_SPEED * (1.0 + power * CHARGE_POWER);
                    s.missiles.push(Missile {
                        x: tip_x, y: tip_y,
                        vx: a.cos() * speed, vy: a.sin() * speed,
                        age: 0.0,
                        charged: power > 0.2,
                    });
                    s.charge = 0.0;
                }
                (a, level)
            }
        };

        // ── Update DOM (ship + charge) ───────────────────────────────────────
        let (ship_state, spawn_flash) = {
            let s = s4.borrow();
            (s.ship, s.spawn_flash)
        };

        if let Some(ref el) = *ship_el.borrow() {
            let started = s4.borrow().started;
            if !started {
                // Move completely off-screen, invisible
                let _ = el.set_attribute("transform", "translate(-100,-100)");
                let _ = el.set_attribute("opacity", "0");
            } else {
                let scale = if spawn_flash > 0.0 {
                    let emerge = 1.0 - (spawn_flash / 1.5);
                    emerge.max(0.0).min(1.0)
                } else {
                    1.0
                };
                let flicker = spawn_flash > 0.0 && ((spawn_flash * 12.0) as i32) % 2 == 0;
                let _ = el.set_attribute("transform",
                    &format!("translate({:.2} {:.2}) rotate({:.0}) scale({:.2})",
                        ship_state.0, ship_state.1, ship_state.4, scale));
                if spawn_flash > 0.0 {
                    let _ = el.set_attribute("opacity", if flicker { "1" } else { "0.2" });
                    let _ = el.set_attribute("stroke", if flicker { "#00ffcc" } else { "#ffffff" });
                } else {
                    let _ = el.set_attribute("opacity", "1");
                    let _ = el.set_attribute("stroke", "#0f0");
                }
            }
        }
        if let Some(ref el) = *charge_el.borrow() {
            if !s4.borrow().started {
                // Hidden before spawn
                let _ = el.set_attribute("opacity", "0");
            } else {
                let r = charge_level / MAX_CHARGE;
                let nose_x = ship_state.0 + rad.cos() * 3.0;
                let nose_y = ship_state.1 + rad.sin() * 3.0;
                let _ = el.set_attribute("cx", &format!("{:.2}", nose_x));
                let _ = el.set_attribute("cy", &format!("{:.2}", nose_y));
                let _ = el.set_attribute("r", &format!("{:.1}", CHARGE_RADIUS * r));
                let _ = el.set_attribute("opacity", &format!("{:.2}", if charge_level > 0.0 { 0.3 + 0.5 * r } else { 0.0 }));
                let _ = el.set_attribute("fill", if charge_level >= MAX_CHARGE { "#ff4444" } else { "#ffaa00" });
            }
        }

        // ── Missiles & Asteroids update ──────────────────────────────────────
        let score_delta = {
            let mut s = s4.borrow_mut();

            // Move missiles
            let mut dead_m = Vec::new();
            for (i, m) in s.missiles.iter_mut().enumerate() {
                m.x = (m.x + m.vx + 100.0) % 100.0;
                m.y = (m.y + m.vy + 100.0) % 100.0;
                m.age += dt;
                let max_age = if m.charged { CHARGED_LIFE } else { NORMAL_LIFE };
                if m.age > max_age { dead_m.push(i); }
            }

            // Move asteroids (always — even on title/death screen)
            for a in s.asteroids.iter_mut() {
                a.x = (a.x + a.vx + 100.0) % 100.0;
                a.y = (a.y + a.vy + 100.0) % 100.0;
                a.rot = (a.rot + a.rot_v * dt + 360.0) % 360.0;
            }

            // Collision: collect positions for collision checks (avoid nested borrows)
            let missile_data: Vec<(usize, f64, f64, bool)> = s.missiles.iter()
                .enumerate()
                .map(|(i, m)| (i, m.x, m.y, m.charged))
                .collect();

            let asteroid_snapshot: Vec<(usize, f64, f64, u8)> = s.asteroids.iter()
                .enumerate()
                .map(|(i, a)| (i, a.x, a.y, a.size))
                .collect();

            let ship_pos = (s.ship.0, s.ship.1);
            let spawn_flash_now = s.spawn_flash;
            let mut dead_a = Vec::new();
            let mut new_asteroids = Vec::new();
            let mut pts: i32 = 0;
            let mut explosions: Vec<(f64, f64, &'static str, usize, f64, f64, f64)> = Vec::new();
            let mut ship_dead = false;

            // Check missile-asteroid collisions using snapshots
            for (mi, mx, my, charged) in &missile_data {
                if dead_m.contains(mi) { continue; }
                for (ai, ax, ay, a_size) in &asteroid_snapshot {
                    if dead_a.contains(ai) { continue; }
                    let dx = mx - ax;
                    let dy = my - ay;
                    let hit_r = match a_size { 0 => 6.0, 1 => 3.0, _ => 1.5 };
                    if (dx*dx + dy*dy).sqrt() < hit_r {
                        if !dead_m.contains(mi) { dead_m.push(*mi); }
                        if !dead_a.contains(ai) { dead_a.push(*ai); }

                        if *charged {
                            pts += match a_size { 0 => 50, 1 => 100, _ => 150 };
                            let count = match a_size { 0 => 24, 1 => 16, _ => 10 };
                            explosions.push((*ax, *ay, "#ffaa00", count, 0.4, 0.8, 0.3));
                            explosions.push((*ax, *ay, "#ff4400", count/2, 0.6, 0.5, 0.2));
                        } else {
                            pts += match a_size { 0 => 20, 1 => 50, _ => 100 };
                            let count = match a_size { 0 => 12, 1 => 8, _ => 5 };
                            explosions.push((*ax, *ay, "#aaaaaa", count, 0.25, 0.6, 0.2));
                            // Split: look up original asteroid
                            if let Some(orig) = s.asteroids.get(*ai) {
                                if let Some((k1, k2)) = orig.split() {
                                    new_asteroids.push(k1);
                                    new_asteroids.push(k2);
                                }
                            }
                        }
                    }
                }
            }

            // Ship-asteroid collision check
            for (ai, ax, ay, a_size) in &asteroid_snapshot {
                let dx = ship_pos.0 - ax;
                let dy = ship_pos.1 - ay;
                let hit_r = match a_size { 0 => 6.0, 1 => 3.0, _ => 1.5 } + 1.5;
                if (dx*dx + dy*dy).sqrt() < hit_r {
                    if spawn_flash_now > 0.0 {
                        // Spawn protection: destroy asteroid instead of killing ship
                        if !dead_a.contains(ai) {
                            dead_a.push(*ai);
                            explosions.push((*ax, *ay, "#00ffcc", 6, 0.3, 0.4, 0.15));
                        }
                    } else {
                        ship_dead = true;
                        break;
                    }
                }
            }

            // Apply removals
            for i in dead_a.into_iter().rev() { s.asteroids.remove(i); }
            for na in new_asteroids { s.asteroids.push(na); }
            for i in dead_m.into_iter().rev() { s.missiles.remove(i); }

            // Fire explosions
            for (x, y, col, cnt, spd, age, sz) in explosions {
                spawn_explosion(&mut s.particles, x, y, col, cnt, spd, age, sz);
            }

            if ship_dead {
                let sx = s.ship.0;
                let sy = s.ship.1;
                spawn_explosion(&mut s.particles, sx, sy, "#00ff44", 20, 0.5, 1.0, 0.25);
                spawn_explosion(&mut s.particles, sx, sy, "#88ffaa", 12, 0.3, 0.7, 0.15);
                s.ship = (50.0, 50.0, 0.0, 0.0, 90.0);
                s.asteroids.clear();
                spawn_wave(&mut s, 5); // Level 1
                s.score = 0;
                s.level = 1;
                s.warp_transition = 0.0;
                s.started = false;
                ss4.set(0);
                sl4.set(1);
            } else {
                s.score = s.score.saturating_add(pts as u32);
                if pts > 0 { ss4.set(s.score); }
                // Level clear check — all asteroids destroyed
                if s.asteroids.is_empty() && s.warp_transition == 0.0 {
                    s.warp_transition = 2.5; // 2.5s warp transition
                    // Spawn warp-out effect at ship position
                    let sx = s.ship.0;
                    let sy = s.ship.1;
                    for i in 0..5 {
                        s.spawn_rings.push(SpawnRing {
                            x: sx, y: sy,
                            age: 0.0,
                            max_age: 0.5 + (i as f64 * 0.1),
                        });
                    }
                    spawn_explosion(&mut s.particles, sx, sy, "#00ffcc", 12, 0.4, 0.5, 0.15);
                    spawn_explosion(&mut s.particles, sx, sy, "#ffffff", 6, 0.6, 0.3, 0.1);
                }
            }
            pts
        };

        // Update score signal
        if score_delta > 0 {
            ss4.set(s4.borrow().score);
        }

        // Update prompt visibility
        if let Some(ref el) = *prompt_el.borrow() {
            let started = s4.borrow().started;
            let warp_tran = s4.borrow().warp_transition;
            let lvl = s4.borrow().level;
            let text = if !started {
                "PRESS SPACE".to_string()
            } else if warp_tran > 0.0 {
                format!("LEVEL {} CLEAR\nWARPING...", lvl)
            } else {
                String::new()
            };
            let _ = el.set_text_content(Some(&text));
        }

        // ── Render asteroids ─────────────────────────────────────────────────
        if let Some(ref el) = *asteroid_g.borrow() {
            while let Some(child) = el.first_child() {
                let _ = el.remove_child(&child);
            }
            let ns = "http://www.w3.org/2000/svg";
            let doc = web_sys::window().unwrap().document().unwrap();
            for a in s4.borrow().asteroids.iter() {
                if let Ok(p) = doc.create_element_ns(Some(ns), "polygon") {
                    let _ = p.set_attribute("points", &a.pts_str);
                    let _ = p.set_attribute("fill", "none");
                    let _ = p.set_attribute("stroke", "#888");
                    let _ = p.set_attribute("stroke-width", match a.size { 0 => "0.3", 1 => "0.25", _ => "0.2" });
                    let _ = p.set_attribute("transform", &format!("translate({:.2} {:.2}) rotate({:.1})", a.x, a.y, a.rot));
                    let _ = el.append_child(&p);
                }
            }
        }

        // ── Render missiles ─────────────────────────────────────────────────
        if let Some(ref el) = *missile_g.borrow() {
            while let Some(child) = el.first_child() {
                let _ = el.remove_child(&child);
            }
            let ns = "http://www.w3.org/2000/svg";
            let doc = web_sys::window().unwrap().document().unwrap();
            for m in s4.borrow().missiles.iter() {
                if m.charged {
                    let trail_len = 4;
                    for t in 0..trail_len {
                        let tfrac = t as f64 / trail_len as f64;
                        let tx = m.x - m.vx * tfrac * 2.0;
                        let ty = m.y - m.vy * tfrac * 2.0;
                        if let Ok(c) = doc.create_element_ns(Some(ns), "circle") {
                            let _ = c.set_attribute("cx", &format!("{:.2}", tx));
                            let _ = c.set_attribute("cy", &format!("{:.2}", ty));
                            let _ = c.set_attribute("r", &format!("{:.2}", 0.4 * (1.0 - tfrac * 0.6)));
                            let alpha = 1.0 - tfrac * 0.8;
                            let green = (80.0 * (1.0 - tfrac)) as u8;
                            let _ = c.set_attribute("fill", &format!("rgb(255,{},0)", green));
                            let _ = c.set_attribute("opacity", &format!("{:.1}", alpha));
                            let _ = el.append_child(&c);
                        }
                    }
                    if let Ok(c) = doc.create_element_ns(Some(ns), "circle") {
                        let _ = c.set_attribute("cx", &format!("{:.2}", m.x));
                        let _ = c.set_attribute("cy", &format!("{:.2}", m.y));
                        let _ = c.set_attribute("r", "0.4");
                        let _ = c.set_attribute("fill", "#ffffff");
                        let _ = el.append_child(&c);
                    }
                } else {
                    if let Ok(c) = doc.create_element_ns(Some(ns), "circle") {
                        let _ = c.set_attribute("cx", &format!("{:.2}", m.x));
                        let _ = c.set_attribute("cy", &format!("{:.2}", m.y));
                        let _ = c.set_attribute("r", "0.18");
                        let _ = c.set_attribute("fill", "#ffff88");
                        let _ = el.append_child(&c);
                    }
                }
            }
        }

        // ── Update particles ───────────────────────────────────────────────
        {
            let mut s = s4.borrow_mut();
            let mut dead = Vec::new();
            for (i, p) in s.particles.iter_mut().enumerate() {
                p.x = (p.x + p.vx + 100.0) % 100.0;
                p.y = (p.y + p.vy + 100.0) % 100.0;
                p.vx *= 0.96;
                p.vy *= 0.96;
                p.age += dt;
                if p.age > p.max_age { dead.push(i); }
            }
            for i in dead.into_iter().rev() { s.particles.remove(i); }
        }

        // ── Render particles ──────────────────────────────────────────────
        if let Some(ref el) = *particle_g.borrow() {
            while let Some(child) = el.first_child() {
                let _ = el.remove_child(&child);
            }
            let ns = "http://www.w3.org/2000/svg";
            let doc = web_sys::window().unwrap().document().unwrap();
            for p in s4.borrow().particles.iter() {
                let alpha = 1.0 - (p.age / p.max_age);
                if alpha <= 0.0 { continue; }
                if let Ok(c) = doc.create_element_ns(Some(ns), "circle") {
                    let _ = c.set_attribute("cx", &format!("{:.2}", p.x));
                    let _ = c.set_attribute("cy", &format!("{:.2}", p.y));
                    let _ = c.set_attribute("r", &format!("{:.2}", p.size * alpha));
                    let _ = c.set_attribute("fill", p.color);
                    let _ = c.set_attribute("opacity", &format!("{:.2}", alpha));
                    let _ = el.append_child(&c);
                }
            }
        }

        // ── Spawn rings update & render ────────────────────────────────────
        {
            let mut s = s4.borrow_mut();
            // Age rings, collect dead
            let mut dead = Vec::new();
            for (i, r) in s.spawn_rings.iter_mut().enumerate() {
                r.age += dt;
                if r.age > r.max_age { dead.push(i); }
            }
            for i in dead.into_iter().rev() { s.spawn_rings.remove(i); }

            // Render rings
            if let Some(ref el) = *rings_g.borrow() {
                while let Some(child) = el.first_child() {
                    let _ = el.remove_child(&child);
                }
                let ns = "http://www.w3.org/2000/svg";
                let doc = web_sys::window().unwrap().document().unwrap();
                for r in s.spawn_rings.iter() {
                    let progress = r.age / r.max_age; // 0=just born, 1=dead
                    let alpha = 1.0 - progress;
                    if alpha <= 0.0 { continue; }
                    // Ring expands outward
                    let radius = 0.5 + progress * 8.0;
                    if let Ok(c) = doc.create_element_ns(Some(ns), "circle") {
                        let _ = c.set_attribute("cx", &format!("{:.1}", r.x));
                        let _ = c.set_attribute("cy", &format!("{:.1}", r.y));
                        let _ = c.set_attribute("r", &format!("{:.1}", radius));
                        let _ = c.set_attribute("fill", "none");
                        let _ = c.set_attribute("stroke", "#00ffcc");
                        let _ = c.set_attribute("stroke-width", &format!("{:.2}", 0.3 * alpha));
                        let _ = c.set_attribute("opacity", &format!("{:.2}", alpha));
                        let _ = el.append_child(&c);
                    }
                }
            }
        }
    }).forget();

    leptos::view! {
        <div style="width:100vw;height:100vh;background:#000;overflow:hidden;position:relative;">
            <svg width="100%" height="100%" viewBox="0 0 100 100" preserveAspectRatio="none">
                <g id="stars" inner_html={stars}></g>
                <g id="asteroids"></g>
                <g id="missiles"></g>
                <g id="particles"></g>
                <g id="spawn-rings"></g>
                <polygon
                    id="ship"
                    points="-1,-1 3,0 -1,1"
                    fill="none" stroke="#0f0" stroke-width="0.2"
                    transform="translate(50 50) rotate(90)"
                />
                <circle id="charge" cx="50" cy="50" r="0" fill="#ffaa00" opacity="0"/>
            </svg>
            <div style="position:absolute;top:8px;right:12px;color:#0f0;font-family:monospace;font-size:16px;text-shadow:0 0 4px #0f0;pointer-events:none;text-align:right;line-height:1.4;">
                <div>{ level_sig }</div>
                <div>{ score_sig }</div>
            </div>
            <div
                id="prompt"
                style="position:absolute;top:50%;left:50%;transform:translate(-50%,-50%);color:#0f0;font-family:monospace;font-size:14px;text-shadow:0 0 8px #0f0;pointer-events:none;text-align:center;letter-spacing:2px;white-space:pre;"
            >
                PRESS SPACE
            </div>
        </div>
    }
}
