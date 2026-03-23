# =============================================================================
# Oxidize - Multi-stage Production Build
# =============================================================================
# Stage 1: Builder
# =============================================================================
FROM rust:1.82-slim-bookworm AS builder

# Install build dependencies
RUN apt-get update && apt-get install -y \
    build-essential \
    pkg-config \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copy manifests first for dependency caching
COPY Cargo.toml Cargo.lock ./
COPY oxidize-engine/Cargo.toml oxidize-engine/
COPY oxidize-server/Cargo.toml oxidize-server/
COPY oxidize-ui/Cargo.toml oxidize-ui/

# Create dummy source files to cache dependency compilation
RUN mkdir -p src && \
    echo "fn main() {}" > src/main.rs && \
    echo "pub fn lib() {}" > src/lib.rs

# Build dependencies (cached if Cargo.toml unchanged)
RUN cargo build --release -p oxidize-server --bin oxidize-server 2>/dev/null || true

# Copy actual source code
COPY . .

# Build the server binary
RUN cargo build --release -p oxidize-server --bin oxidize-server

# =============================================================================
# Stage 2: Runtime
# =============================================================================
FROM debian:bookworm-slim

# Install runtime dependencies only
RUN apt-get update && apt-get install -y \
    ca-certificates \
    libsqlite3-0 \
    && rm -rf /var/lib/apt/lists/* \
    && apt-get clean

# Create non-root user for security
RUN groupadd --gid 1000 oxidize && \
    useradd --uid 1000 --gid oxidize --shell /bin/false --create-home oxidize

WORKDIR /app

# Copy binary from builder
COPY --from=builder /app/target/release/oxidize-server /app/oxidize-server

# Copy data directory structure
RUN mkdir -p /app/data && chown oxidize:oxidize /app

# Switch to non-root user
USER oxidize

# Expose default port
EXPOSE 7412

# Health check
HEALTHCHECK --interval=30s --timeout=10s --start-period=5s --retries=3 \
    CMD curl -f http://localhost:7412/health || exit 1

# Run as non-root
ENTRYPOINT ["./oxidize-server"]
