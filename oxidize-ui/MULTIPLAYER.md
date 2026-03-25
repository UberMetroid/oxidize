# Multiplayer Architecture — Oxidize

## Current State
- Single-player only, no networking
- Server: Axum REST API (player sync, leaderboards, achievements)
- WebSocket: Not yet implemented

---

## Design Goals
1. **Low latency** — 60fps game loop, <100ms perceived latency
2. **Authoritative server** — server is truth, clients predict locally
3. **Graceful degradation** — works fine as single-player
4. **Horizontal scaling** — sessions are stateless and splittable

---

## Transport: WebSocket over HTTP

WebSocket (via `tokio-tungstenite`) over the same Axum server on port 7413.

### Why WebSocket over WebRTC?
- Simpler NAT traversal (server is already public)
- Server-authoritative is easier to implement correctly
- WebRTC P2P adds complexity for ~no benefit given server-authoritative model
- WebSocket handles reconnection, framing, routing automatically

### Alternative considered: WebRTC DataChannel
- P2P with server as signaling only
- Better for truly peer-to-peer (like peer-to-peer games)
- Adds signaling complexity
- Not needed for this use case

---

## Session Model

### Room/Lobby System
```
/api/lobby          POST  → create room, returns room_code (4-char alphanumeric)
/api/lobby/:code    GET   → room info, player list
/api/lobby/:code    POST  → join room
/ws?room=CODE&name=JERYD  → WebSocket connection
```

### Room States
```
WAITING    → Players joining, game not started
COUNTDOWN  → 3-2-1 countdown, players locked in
PLAYING    → Game active
FINISHED   → Results displayed, players can rematch
```

---

## WebSocket Message Protocol

All messages are JSON. Direction: C=client→server, S=server→client.

### Connection
```
C: { "type": "join", "room": "ABCD", "name": "Jeryd", "color": "#00ff44" }
S: { "type": "joined", "player_id": "uuid", "players": [...], "state": "WAITING" }
```

### Game Start
```
S: { "type": "game_start", "seed": 12345, "level": 1 }
S: { "type": "countdown", "seconds": 3 }
S: { "type": "countdown", "seconds": 2 }
S: { "type": "countdown", "seconds": 1 }
S: { "type": "game_active" }
```

### In-Game Commands (C → S)
```
C: { "type": "input", "keys": { "up": true, "left": false, ... }, "charge": 0.0, "charging": false }
C: { "type": "fire", "charge_level": 1.5 }
```
- Client sends input snapshot ~20-30 times/sec (every 2-3 frames)
- Server processes inputs and advances game simulation

### Game State Sync (S → C broadcast, ~20 times/sec)
```
S: {
  "type": "state",
  "tick": 1234,
  "players": [
    {
      "id": "uuid",
      "name": "Jeryd",
      "color": "#00ff44",
      "ship": { "x": 50.0, "y": 48.0, "vx": 0.01, "vy": -0.02, "angle": 95.0 },
      "score": 150,
      "level": 2,
      "alive": true
    },
    {
      "id": "uuid2",
      "name": "Vegeta",
      "color": "#ff4444",
      "ship": { "x": 60.0, "y": 45.0, "vx": 0.0, "vy": 0.0, "angle": 180.0 },
      "score": 80,
      "level": 2,
      "alive": false
    }
  ],
  "asteroids": [ { "x": 20.0, "y": 30.0, "vx": 0.05, "vy": -0.02, "rot": 45.0, "size": 0 }, ... ],
  "missiles": [ { "x": 51.0, "y": 48.5, "vx": 0.5, "vy": 0.1, "charged": false, "owner": "uuid" }, ... ],
  "particles": [ ... ]  // only spawned events, not persistent state
}
```

### Interpolation Notes
- Clients receive authoritative positions at ~20Hz (every 50ms)
- Client interpolates between received states for smooth 60fps rendering
- Client can render immediately from predicted local state, reconcile with server periodically

### Collision Events (S → C)
```
S: { "type": "hit", "target": "asteroid_123", "by": "uuid", "asteroid": { "destroyed": true, "split": true, "new_positions": [...] } }
S: { "type": "hit", "target": "player_uuid", "by": "asteroid_id" }
S: { "type": "player_died", "player_id": "uuid" }
```

### Level Clear (S → C)
```
S: { "type": "level_clear", "level": 2, "next_warp_seconds": 2.5 }
S: { "type": "level_start", "level": 3, "asteroid_count": 9 }
```

### Disconnect / Reconnect
```
C (disconnect)
S: { "type": "player_left", "player_id": "uuid", "reason": "disconnect" }
C (reconnect with same room + player_id)
S: { "type": "rejoined", "state": "PLAYING", "tick": 5678 } // server sends current state
```

### Disconnect Handling
- Player removed from game after 5 seconds of no heartbeat
- Game continues with remaining players
- If host disconnects, next player becomes host
- If all players disconnect, game ends (server cleans up session)

---

## Server-Side Game Simulation

### Architecture
```
WebSocket Connections (one per player)
         ↓
   SessionManager (room state machine)
         ↓
    GameSimulation (tick loop, 60 ticks/sec)
         ↓
   CollisionEngine (physics + hits)
         ↓
   BroadcastScheduler (20Hz state broadcasts)
```

### Tick Rate
- Server runs physics at 60 ticks/sec (deterministic)
- Server tick is source of truth
- Client runs at 60fps for rendering, interpolates between server states

### Determinism
- Game uses seeded RNG (server seed broadcast at game start)
- Same seed → identical asteroid positions/velocities on all clients
- Ship movement is deterministic: input + previous state → next state
- Collision resolution is deterministic (first to process wins, server is arbiter)

### Collision Priority
- Server processes collisions in tick order
- Missile hit → asteroid splits/destroyed → score updated
- Asteroid hits player → player dies, respawns
- Multiple missiles hitting same asteroid in same tick → first one wins (by missile ID order)

---

## Client-Side Prediction

### Local Prediction
```
Input received → Apply to local state immediately → Render
                       ↓
              Send to server asynchronously
                       ↓
              Server confirms/rectifies
                       ↓
              If correction needed: snap to server state
```

### Prediction Reconciliation
```
1. Client sends input with client_tick
2. Server processes, returns confirmed_tick
3. Client drops all inputs up to confirmed_tick
4. Client replays remaining inputs on top of server state
5. Visual jump if difference is large → snap
```

### Latency Compensation
- Client renders with predicted state (no delay)
- Latency shown to player: not compensated (it's the player's problem to lead shots)
- Other players' positions are server-authoritative, rendered via interpolation

---

## Data Structures

### Player State (server)
```rust
struct Player {
    id: Uuid,
    name: String,
    color: String,
    state: PlayerState,
    ship: Ship,
    score: u32,
    level: u32,
    last_input: InputState,
    last_tick: u64,
    connected: bool,
}
```

### Ship (shared)
```rust
struct Ship {
    x: f64, y: f64,
    vx: f64, vy: f64,
    angle: f64,
    alive: bool,
    spawn_flash: f64,
}
```

### Session (server)
```rust
struct Session {
    id: String,
    host_id: Uuid,
    state: SessionState,
    players: HashMap<Uuid, Player>,
    game: GameState,
    server_tick: u64,
    broadcast_tick: u64,
    seed: u64,
}
```

### GameState (shared with client)
```rust
struct GameState {
    asteroids: Vec<Asteroid>,
    missiles: Vec<Missile>,
    particles: Vec<ParticleEvent>,
    // (no persistent entities in game state — spawn/despawn handled by events)
}
```

---

## API Endpoints (extended)

### REST (existing + new)
```
GET  /health              → Server health
POST /api/rooms           → Create room { name } → { room_id, code }
GET  /api/rooms/:code     → Room info + player list
POST /api/rooms/:code/join → Join room { name, color } → { player_id, ws_url }
GET  /api/rooms/:code/state → Current game state (for late joiners)
```

### WebSocket
```
WS /ws?room=ABCD&player_id=UUID
```

---

## Client State Machine

```
IDLE
  ├── create_room() → CREATING
  ├── join_room(code) → JOINING
  └── quick_play() → MATCHMAKING

CREATING / JOINING
  └── ws_connected → LOBBY

LOBBY
  ├── host clicks start → WAITING_FOR_PLAYERS
  └── player_disconnect → IDLE

WAITING_FOR_PLAYERS
  ├── countdown starts → COUNTDOWN (if 1+ players)
  └── player_leave → LOBBY

COUNTDOWN (3-2-1)
  └── timer expires → PLAYING

PLAYING
  ├── all asteroids cleared → LEVEL_TRANSITION
  ├── all players dead → GAME_OVER
  └── server sends state → RENDER (interpolate)

LEVEL_TRANSITION
  └── warp complete → PLAYING (next level)

GAME_OVER
  └── rematch → PLAYING
  └── leave → IDLE
```

---

## Performance Considerations

### Server
- Session stored in `DashMap` (sharded HashMap) for lock-free concurrent access
- Game state serialized to JSON once per broadcast tick (20Hz)
- Asteroid/missile counts capped: max 50 asteroids, 20 missiles per player, 200 particles
- Tick loop: 60Hz, bounded by physics + collision. Collision is O(n²) with small n

### Client
- WebSocket receives state at 20Hz
- Renders at 60fps
- Uses `requestAnimationFrame` for render loop
- Interpolates other players between received states (lerp)
- Local player is client-predictive (no interpolation)
- Particle count capped at 100 active

### Bandwidth
- State broadcast: ~2KB per tick (gzip compressed: ~200 bytes)
- 20Hz × 2KB = 40KB/s per player
- 8 players = 320KB/s server egress
- Acceptable for typical connections

---

## Security

- Player IDs are UUIDs (server-assigned, unguessable)
- Room codes are 4-char alphanumeric (6²⁴ ≈ 2M combinations, brute-forceable but rate-limited)
- Server is authoritative: client never sends positions, only inputs
- Server rate-limits input messages (max 60 per second per player)
- Profanity filter on player names (server-side)
- Max 8 players per room

---

## Testing Strategy

### Unit Tests
- Physics deterministic: `f(0, state) == f(f(0, state))` across multiple ticks
- Collision detection correctness (circle-circle, circle-polygon)
- Score calculation edge cases
- RNG seed reproducibility

### Integration Tests
- WebSocket connect/disconnect flow
- Room creation, join, leave
- Game start countdown
- State broadcast correctness
- Player disconnect/reconnect

### Load Tests
- 100 concurrent rooms × 8 players = 800 WebSocket connections
- Measure: tick processing time, broadcast latency, memory usage
- Target: <5ms per tick at 60Hz

---

## Implementation Phases

### Phase 1: Core Multiplayer (MVP)
1. Add `tokio-tungstenite` + `axum-websockets` to server
2. Room creation/join REST endpoints
3. WebSocket handler: join, leave, input messages
4. Server-side game simulation (60 ticks/sec)
5. State broadcast (20Hz)
6. Client: WebSocket client, prediction, interpolation
7. Basic play: 2 players, asteroids, scoring

### Phase 2: Polish
1. Latency display
2. Connection status indicators
3. Reconnection handling
4. Late-joiner state sync

### Phase 3: Advanced
1. Matchmaking (public rooms)
2. Spectator mode
3. Chat / reactions
4. Sound sync across players
5. Leaderboard for multiplayer sessions
