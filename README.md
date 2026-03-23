# OXIDIZE

![Rust](https://img.shields.io/badge/Rust-1.82+-orange.svg)
![License](https://img.shields.io/badge/License-MIT-blue.svg)
![Build](https://img.shields.io/github/actions/workflow/status/ubermetroid/oxidize/ci.yml)
![Docker](https://img.shields.io/badge/Docker-ready-2496ed.svg?logo=docker)

**Oxidize** is a zero-click, brutalist incremental management game built entirely from first-principles in Type-Safe Rust.

You do not click to gather resources. You optimize the simulation. The star decays. The numbers go up. The system mocks you.

---

## 🚀 Quick Start

### Prerequisites

- **Docker** and **Docker Compose** (for containerized deployment)
- OR **Rust 1.82+** (for local development)
- **NAS/Server**: Synology NAS, Unraid, or any Linux server with Docker

### Deploy with Docker

```bash
# Clone the repository
git clone https://github.com/ubermetroid/oxidize.git
cd oxidize

# Copy environment template
cp .env.example .env

# Edit .env with your settings (PUID, PGID for NAS)
nano .env

# Start the container
docker compose up -d

# Check logs
docker compose logs -f
```

The server will be available at `http://your-server:7412`

### Health Endpoint

```bash
curl http://localhost:7412/health
# Response: {"status":"ok","database":"connected","version":"0.1.0"}
```

---

## 🕹 The System

- **Zero-Click Mechanics**: The energy flows automatically. Your only input is the ruthless optimization of capital via upgrades.
- **Factions**: Choose your faction color. Each faction has unique mechanics and bonuses.
- **Progressive Web App (PWA)**: The client installs locally to your mobile or desktop hardware with a built-in Service Worker for offline calculation.
- **Empirical Persistence**: The backend runs an embedded, transactional `sqlx` SQLite database to guarantee total data integrity.

---

## 🧪 Testing

```bash
# Run all tests (59 total)
cargo test --workspace

# Engine unit tests + benchmarks
cargo test --package oxidize-engine

# Server integration tests
cargo test --package oxidize-server

# UI E2E tests (requires Chrome + headless_chrome + running server)
cargo test --package oxidize-ui
```

### Test Coverage

| Package | Tests | Description |
|---------|-------|-------------|
| `oxidize-engine` | 47 | PlayerState (20) + Factions (12) + Achievements (3) + Architect (10) + Benchmarks (5) |
| `oxidize-server` | 12 | API: sync, leaderboard, global stats, achievements, streaks |
| `oxidize-ui` | 5 | E2E: page load, buttons, modals (requires server) |

---

## 🔧 Development

### Local Development

```bash
# Build all packages
cargo build --workspace

# Run server
cd oxidize-server && cargo run

# Run tests
cargo test --workspace

# Run benchmarks
cargo bench --package oxidize-engine

# Format code
cargo fmt

# Lint
cargo clippy --workspace
```

### Prerequisites (Local Dev)

- Rust 1.82+
- For UI: `trunk` and `wasm-pack`
- For E2E tests: Chrome/Chromium installed

---

## 📁 Architecture

```
oxidize/
├── Cargo.toml              # Workspace
├── Dockerfile              # Multi-stage production build
├── docker-compose.yml      # Production compose configuration
├── .env.example            # Environment template
│
├── oxidize-engine/         # Core game logic (all src files <256 lines)
│   ├── src/
│   │   ├── lib.rs          # Module re-exports
│   │   ├── types.rs        # Faction, UpgradeType enums
│   │   ├── player/        # PlayerState module (split for <256 lines)
│   │   │   ├── core.rs     # Player state, tick, buy_upgrade
│   │   │   └── calculations.rs  # Energy calculations
│   │   ├── factions.rs     # Faction bonuses and mechanics
│   │   ├── achievements.rs # Achievement definitions
│   │   ├── architect.rs    # AI overseer, milestones
│   │   └── quips/          # Faction-specific snark module
│   ├── benches/            # Criterion benchmarks
│   └── tests/              # Unit tests
│
├── oxidize-server/         # REST API (Axum)
│   ├── src/
│   │   ├── main.rs          # Entry point, graceful shutdown
│   │   ├── lib.rs           # AppState, module exports
│   │   ├── handlers/        # API handlers
│   │   ├── models.rs        # Request/response types
│   │   └── db.rs            # Database schema, operations
│   └── tests/              # Integration tests (12 tests)
│
└── oxidize-ui/             # WebAssembly frontend (Leptos)
    └── ...
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
│         │                  │ Sync every 2s (Yellow: 1.5s)      │
│         ▼                  ▼                                    │
│  ┌─────────────────────────────────────────────────────────────┐│
│  │              sync_state(uuid, PlayerState)                  ││
│  │  • last_synced_total_energy tracks delta for global stats  ││
│  │  • newly_unlocked_achievements returned on each sync      ││
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
└─────────────────────────────────────────────────────────────────┘
```

---

## ⚙️ Environment Variables

| Variable | Default | Description |
|----------|---------|-------------|
| `HOST` | `0.0.0.0` | Host to bind to |
| `PORT` | `7412` | Port to expose |
| `DATA_DIR` | `./data` | Directory for SQLite database |
| `RUST_LOG` | `info` | Log level: trace, debug, info, warn, error |
| `PUID` | `1000` | User ID for NAS permission handling |
| `PGID` | `1000` | Group ID for NAS permission handling |
| `TZ` | `UTC` | Timezone for logs |
| `CPU_LIMIT` | `1.0` | Maximum CPU usage (fraction of core) |
| `MEMORY_LIMIT` | `512M` | Maximum memory usage |

---

## 🐳 Deployment

### Synology NAS

1. Install Docker via Package Center
2. SSH into your NAS
3. Clone and configure:

```bash
git clone https://github.com/ubermetroid/oxidize.git
cd oxidize
cp .env.example .env

# Edit .env with your user PUID/PGID
# Find with: id yourusername
nano .env

# Start
docker compose up -d
```

### Unraid

1. Install Docker Manager from Apps
2. Create new container with Docker Compose template
3. Point to your `docker-compose.yml`

### Cloudflare / Tailscale

The server binds to `0.0.0.0` and is designed to sit behind:
- **Cloudflare Tunnel**: Point tunnel to `:7412`
- **Tailscale**: Use Tailscale HTTPS certificate

No local TLS configuration needed - let the reverse proxy handle it.

---

## 🏛️ Faction Mechanics

| Faction | Bonus |
|---------|-------|
| **Red** | +25% Orbital Mirrors |
| **Orange** | +10% all generation |
| **Yellow** | +30% first 5 upgrades, -10% after, 1.5s sync |
| **Green** | +50% offline, meditation bonus after 2min idle |
| **Blue** | +5% all, +15% costs, +3% efficiency per upgrade |
| **Purple** | +75% Plasma, -20% Sail/Mirror |

---

## 📜 License

MIT License - see LICENSE file for details.

---

## 🤝 Contributing

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Run tests: `cargo test --workspace`
5. Format: `cargo fmt`
6. Lint: `cargo clippy --workspace`
7. Submit a pull request
