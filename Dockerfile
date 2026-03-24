# =============================================================================
# Oxidize - Multi-stage Production Build
# =============================================================================
# Stage 1: Builder
# =============================================================================
FROM rust:1.85-slim-bookworm AS builder

# Install build dependencies
RUN apt-get update && apt-get install -y \
    build-essential \
    pkg-config \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /build

# Copy manifests first for dependency caching
COPY Cargo.toml Cargo.lock ./
COPY oxidize-engine/Cargo.toml oxidize-engine/
COPY oxidize-server/Cargo.toml oxidize-server/

# Copy oxidize-engine source (needed by server)
COPY oxidize-engine/src oxidize-engine/src
COPY oxidize-engine/Cargo.toml oxidize-engine/
COPY oxidize-engine/benches oxidize-engine/benches

# Copy oxidize-server source
COPY oxidize-server/src oxidize-server/src
COPY oxidize-server/Cargo.toml oxidize-server/

# Copy frontend assets
COPY oxidize-ui/dist oxidize-ui/dist

# Copy assets to build stage for runtime copying
RUN mkdir -p /build/oxidize-ui && cp -r oxidize-ui/dist/* /build/oxidize-ui/

# Create dummy source for workspace
RUN mkdir -p oxidize-ui/src && \
    echo "pub fn lib() {}" > oxidize-ui/src/lib.rs && \
    echo "[package]" > oxidize-ui/Cargo.toml && \
    echo "name = \"oxidize-ui\"" >> oxidize-ui/Cargo.toml && \
    echo 'version = "0.1.0"' >> oxidize-ui/Cargo.toml && \
    echo 'edition = "2021"' >> oxidize-ui/Cargo.toml

# Build dependencies
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
COPY --from=builder /build/target/release/oxidize-server /app/oxidize-server

# Copy frontend assets
COPY --from=builder /build/oxidize-ui/dist /app/oxidize-ui/dist

# Create data directory with proper permissions for non-root user
RUN mkdir -p /app/data && chown -R oxidize:oxidize /app

# Switch to non-root user
USER oxidize

# Expose default port
EXPOSE 7412



# Run as non-root
ENTRYPOINT ["./oxidize-server"]
