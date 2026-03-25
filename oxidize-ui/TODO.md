# TODO — Oxidize Asteroids

## Phase 1: Critical Bugs ✅
- [x] Fix duplicate `dead_m` declaration (shadow bug — missiles don't expire)
- [x] Remove duplicate `s.asteroids.clear()` on death
- [x] Fix `spawn_wave` safe spawn check

## Phase 2: Performance
- [ ] Batch DOM updates — don't remove+recreate every frame
- [ ] Cache star positions (static, computed once)
- [ ] Cache asteroid `points_str()` on creation
- [ ] Object pooling for particles (reuse SVG elements)

## Phase 3: Code Quality
- [ ] Extract spawn/warp effect into helper fn (DRY)
- [ ] Extract level-advance logic
- [ ] Add game constants struct
- [ ] Refactor collision system

## Phase 4: Testing Infrastructure
- [ ] Add `wasm-bindgen-test` to Cargo.toml
- [ ] Add `trunk.toml` with test runner config
- [ ] Test: physics (ship inertia, drag, rotation)
- [ ] Test: collision detection (circle-circle, point-in-polygon approximations)
- [ ] Test: missile lifecycle (charge, fire, expiry)
- [ ] Test: asteroid splitting
- [ ] Test: level progression / warp transition

## Phase 5: Networking / Multiplayer Prep
- [ ] Design network architecture (WebSocket? WebRTC?)
- [ ] Define game state sync protocol (positions, hits, scores)
- [ ] Server-side game state reconciliation
- [ ] Client-side prediction + reconciliation
- [ ] Player spawn/respawn coordination
- [ ] API endpoint design (Rust Axum server?)
- [ ] Latency compensation strategy

## Phase 6: Polish
- [ ] Sound effects (Web Audio API)
- [ ] High score persistence (localStorage)
- [ ] Pause menu
- [ ] Mobile touch controls
