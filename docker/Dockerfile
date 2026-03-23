# syntax = docker/dockerfile:1

# Stage 1: Build Wasm UI and Rust Server
FROM rust:1.94.0-slim-bullseye AS builder
RUN apt-get update && apt-get install -y curl pkg-config libssl-dev build-essential
RUN rustup target add wasm32-unknown-unknown
RUN cargo install trunk

WORKDIR /app
COPY Cargo.toml Cargo.lock ./
COPY oxidize-engine ./oxidize-engine
COPY oxidize-ui ./oxidize-ui
COPY oxidize-server ./oxidize-server

WORKDIR /app/oxidize-ui
RUN trunk build --release

WORKDIR /app/oxidize-server
RUN cargo build --release

# Stage 2: Production Server
FROM debian:bullseye-slim AS prod

RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*

WORKDIR /app

RUN groupadd -g 1001 oxidize && \
    useradd -r -u 1001 -g oxidize oxidize && \
    mkdir -p /app/dist /app/data && \
    chown -R oxidize:oxidize /app

VOLUME /app/data

COPY --from=builder --chown=oxidize:oxidize /app/oxidize-ui/dist /app/dist
COPY --from=builder --chown=oxidize:oxidize /app/target/release/oxidize-server /app/oxidize-server-bin

USER oxidize
EXPOSE 7412

CMD ["/app/oxidize-server-bin"]
