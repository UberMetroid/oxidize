# OXIDIZE

**Oxidize** is a zero-click, brutalist incremental management game built entirely from first-principles in Type-Safe Rust. 

You do not click to gather resources. You optimize the simulation. The star decays. The numbers go up. The system mocks you.

---

## 🕹 The System
- **Zero-Click Mechanics**: The energy flows automatically. Your only input is the ruthless optimization of capital via upgrades.
- **Generational Tribalism**: Agents assign themselves to a biological faction (Gen Alpha, Gen Z, Millennials, Gen X, Boomers). You are building a massive, shared Dyson Sphere around a single star. 
- **The Architect**: A cynical AI overseer that actively mocks your mathematical inefficiencies and offline habits using the localized dialect of your selected tribe.
- **Progressive Web App (PWA)**: The client installs locally to your mobile or desktop hardware with a built-in Service Worker for offline calculation.
- **Empirical Persistence**: The backend runs an embedded, transactional `sqlx` SQLite database to guarantee total data integrity.

---

## 🐳 Deployment (Docker)

Oxidize is distributed as a multi-stage, heavily optimized container. It operates on port `9531`.

### **Docker Compose**
```yaml
version: "3.8"
services:
  oxidize:
    image: ghcr.io/ubermetroid/oxidize:latest
    container_name: oxidize
    restart: unless-stopped
    ports:
      - "9531:9531"
    volumes:
      - ./data:/app/data
```