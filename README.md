# OXIDIZE

**Oxidize** is a zero-click, brutalist incremental management game built entirely from first-principles in Type-Safe Rust.

You do not click to gather resources. You optimize the simulation. The star decays. The numbers go up. The system mocks you.

---

## 🕹 The System
- **Zero-Click Mechanics**: The energy flows automatically. Your only input is the ruthless optimization of capital via upgrades.
- **Factions**: Choose your faction color. Purely cosmetic, affects UI theme only.
- **Progressive Web App (PWA)**: The client installs locally to your mobile or desktop hardware with a built-in Service Worker for offline calculation.
- **Empirical Persistence**: The backend runs an embedded, transactional `sqlx` SQLite database to guarantee total data integrity.

---

## 🧪 Testing

```bash
# Run all tests
cargo test --all

# Engine unit tests (19 tests)
cargo test --package oxidize-engine

# Server integration tests (6 tests)
cargo test --package oxidize-server

# UI E2E tests (requires Chrome + headless_chrome)
cargo test --package oxidize-ui
```

### Test Coverage
| Package | Tests | Description |
|---------|-------|-------------|
| `oxidize-engine` | 29 | PlayerState (19) + Architect (10) |
| `oxidize-server` | 6 | API: sync, leaderboard, global stats |
| `oxidize-ui` | 5 | E2E: page load, buttons, modals (requires server) |

---

## 🐳 Deployment (Docker)

Oxidize is distributed as a multi-stage, heavily optimized container. It operates on port `3000`.

### **Docker Compose**
```yaml
version: "3.8"
services:
  oxidize:
    image: ghcr.io/ubermetroid/oxidize:latest
    container_name: oxidize
    restart: unless-stopped
    ports:
      - "3000:3000"
    volumes:
      - ./data:/app/data
```

---

## 📁 Architecture

```
oxidize/
├── Cargo.toml              # Workspace
├── oxidize-engine/         # Core game logic (all src files <256 lines)
│   └── src/
│       ├── lib.rs          # Module re-exports
│       ├── types.rs        # Faction, UpgradeType enums
│       ├── player.rs       # PlayerState (19 unit tests)
│       ├── architect.rs    # AI overseer, milestones
│       └── quips.rs        # Faction-specific snark
├── oxidize-server/         # REST API (Axum)
│   ├── src/
│   │   ├── handlers/      # API handlers
│   │   ├── models.rs       # Request/response types
│   │   └── db.rs           # Database schema
│   └── tests/             # Integration tests (6 tests)
└── oxidize-ui/            # WebAssembly frontend (Leptos)
    └── tests/             # E2E browser tests (5 tests)
```

---

## 🔄 Data Flow

```
┌─────────────────────────────────────────────────────────────────┐
│                         oxidize-ui (WASM)                        │
│  ┌─────────────┐   ┌──────────────┐   ┌─────────────────────┐  │
│  │ PlayerState │──▶│  Game Loop   │──▶│  Local Storage      │  │
│  │  (signals)  │   │  (100ms tick)│   │  (persistence)      │  │
│  └─────────────┘   └──────────────┘   └─────────────────────┘  │
│         │                  │                                    │
│         │                  │ Sync every 2s                      │
│         ▼                  ▼                                    │
│  ┌─────────────────────────────────────────────────────────────┐│
│  │              sync_state(uuid, PlayerState)                  ││
│  │  • last_synced_total_energy tracks delta for global stats   ││
│  └─────────────────────────────────────────────────────────────┘│
└─────────────────────────────────────────────────────────────────┘
                                │
                                ▼ HTTP POST /api/sync
┌─────────────────────────────────────────────────────────────────┐
│                      oxidize-server (Axum)                      │
│  ┌─────────────┐   ┌──────────────┐   ┌─────────────────────┐  │
│  │   Players   │   │player_scores │   │   global_sphere     │  │
│  │   Table     │   │    Table     │   │   (aggregates)      │  │
│  └─────────────┘   └──────────────┘   └─────────────────────┘  │
│                            │                    ▲               │
│                            │    Adds delta      │               │
│                            └────────────────────┘               │
└─────────────────────────────────────────────────────────────────┘
                                │
                                ▼ HTTP GET /api/global-stats
┌─────────────────────────────────────────────────────────────────┐
│                      oxidize-ui (WASM)                          │
│  ┌─────────────────────────────────────────────────────────────┐│
│  │              SharedViewModal (DYSPHERE)                      ││
│  │  • total_energy (sum of all player deltas)                  ││
│  │  • total_players (player count)                            ││
│  │  • total_*_sails/tethers/mirrors (upgrade counts)          ││
│  └─────────────────────────────────────────────────────────────┘│
└─────────────────────────────────────────────────────────────────┘
```

### Key Design Decisions

1. **Delta-based Global Stats**: `PlayerState.last_synced_total_energy` tracks the baseline for each sync. The server calculates `delta = current - baseline` to avoid double-counting energy in `global_sphere`.

2. **Offline Progress**: On page load, the UI calculates elapsed time since `last_sync_time` and runs `state.tick(delta)` to simulate offline progress.

3. **Architect AI**: The `Architect` struct tracks milestones per player. Quips are generated via `generate_quip(faction, trigger)` based on idle time or purchases.

---

## 🔧 Development

```bash
# Build all packages
cargo build --all

# Run server (requires SQLite data directory)
cd oxidize-server && cargo run

# Build UI (requires trunk)
cd oxidize-ui && trunk build --release
```

### Prerequisites
- Rust 1.70+
- For UI: `trunk` and `wasm-pack`
- For E2E tests: Chrome/Chromium installed