# Code Review — oxidize-ui/src/main.rs

## Bugs (Critical)

### ✅ FIXED: duplicate `dead_m` shadow bug
- **Severity**: Critical (gameplay-affecting)
- **Description**: `let mut dead_m = Vec::new()` declared twice. First declaration tracked expired missiles. Second declaration shadowed the first, so missiles were never removed by age — only by collision.
- **Fix**: Removed duplicate declaration.

### ✅ FIXED: duplicate `s.asteroids.clear()` on death
- **Severity**: Minor (unnecessary call)
- **Fix**: Removed duplicate.

### ⚠️ NOT FIXED: `spawn_flash` floating-point comparison
- The warp completion check `s.warp_transition == 0.0` uses floating-point. After `s.warp_transition -= dt`, it may hit exactly 0.0 or go slightly negative.
- Current fix: `.max(0.0)` on subtraction — prevents going negative.
- Still a potential edge case if `dt` doesn't divide evenly into 2.5.

---

## Performance Issues

### ⚠️ OPEN: Full DOM rebuild every frame
- All five render groups (asteroids, missiles, particles, rings, stars) do `removeChild` + recreate ALL elements every 16ms.
- With ~20 asteroids, ~10 missiles, ~50 particles: ~80 DOM nodes created/destroyed per frame = **4800 nodes/second**.
- **Fix approach**: Use dirty-flag rendering:
  - Track `EntityVersion` (incrementing counter per entity)
  - Compare version to previous frame
  - Only update changed entities
  - Reuse SVG circle/polygon elements from a pool

### ⚠️ OPEN: Stars `inner_html` updated by view! macro
- Stars string is computed once in `Game()` as a `String` — this is fine.
- But `inner_html={stars}` re-evaluates the signal every render pass.
- Should be stored in `leptos::store_signal` or just as a static `&str`.

### ⚠️ OPEN: `Asteroid.points_str()` removed, but `verts` field still exists
- `verts` is no longer read after render switched to `pts_str`
- Generates compiler warning: `field 'verts' is never read`
- **Fix**: Remove `verts` field, keep only `pts_str`

---

## Edge Cases & Robustness

### ⚠️ OPEN: `spawn_wave` asteroid count
- If many asteroids fail the distance check, fewer than `count` may spawn.
- With a large exclusion zone (225 units²), many attempts may fail.
- `gen_verts()` uses `js_sys::Math::random()` — different across WASM invocations.
- **Risk**: If `spawn_wave` fails to place all asteroids in 100 attempts, the game may have fewer asteroids than intended. Unlikely but possible on small worlds.

### ⚠️ OPEN: Level warp safety
- When all asteroids are destroyed, warp is triggered, then `spawn_wave` places new asteroids, then `find_safe_spawn` finds safe spot among new asteroids.
- But `find_safe_spawn` only checks `x,y` — it doesn't account for asteroid hit radii. If an asteroid is exactly 11 units from candidate but has radius 6 (effective center distance 5), it's a collision.
- Current safety: `min_dist=12` vs `asteroid.radius() <= 6`, so `effective_min = 12 + 6 = 18`. Safe.

### ⚠️ OPEN: Multiple asteroids destroyed by same missile in same tick
- If one missile hits two asteroids in the same tick (e.g., they overlap), the missile hits both — which is correct behavior.
- But `dead_m` only records the missile once, so it's removed after the first hit.
- The second asteroid in the loop won't find the missile in `dead_m` — but since the missile has already been removed from the game state, this is fine.

### ⚠️ OPEN: `score_delta` variable exists but is re-read from signal
- `score_delta` is computed but not returned/used directly. The score signal is updated separately with `ss4.set(s4.borrow().score)`.
- `score_delta` is computed as a side effect. Could be simplified.

---

## Potential Improvements

1. **DRY: Spawn/warp effect code duplicated 3x** — extract into helper fn
2. **WASM object pooling** for particles/missiles (avoid GC pressure)
3. **Delta encoding** for state sync (only send changed entities, not full state)
4. **RequestAnimationFrame** vs `gloo_timers::Interval` — Interval uses `setTimeout` which may drift. Consider using RAF for the render loop.
5. **Focus management** — game should capture keyboard focus on load, release on blur
6. **Resize handling** — SVG `viewBox` is set correctly (0-100) so this is fine

---

## Security Notes
- No user input reaches server — all game logic is client-authoritative (for single-player)
- In multiplayer: server must validate all inputs, rate-limit connections
- No sensitive data in game state

---

## Code Quality
- ~700 lines in `main.rs` — should be split into modules
- Suggested structure:
  - `src/game.rs` — GameState, constants, helper functions
  - `src/render.rs` — DOM rendering functions
  - `src/input.rs` — keyboard handling
  - `src/main.rs` — wiring + Leptos view
