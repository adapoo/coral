# syntax=docker/dockerfile:1

# Build stage
FROM rust:1.92-bookworm AS builder

WORKDIR /app

# Install build dependencies
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

# Copy workspace manifest and lockfile
COPY Cargo.docker.toml ./Cargo.toml
COPY Cargo.lock ./

# Copy crates needed for coral-api and coral-admin
COPY crates/hypixel ./crates/hypixel
COPY crates/blacklist ./crates/blacklist
COPY crates/clients ./crates/clients
COPY crates/database ./crates/database
COPY crates/render ./crates/render
COPY crates/coral-api ./crates/coral-api
COPY crates/coral-admin ./crates/coral-admin
COPY crates/migration ./crates/migration

# Build API and Admin binaries
RUN --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=/usr/local/cargo/git \
    --mount=type=cache,target=/app/target \
    cargo build --release --bin coral-api --bin coral-admin \
    && cp target/release/coral-api target/release/coral-admin /usr/local/bin/

# Runtime stage for coral-api
FROM debian:bookworm-slim AS coral-api

RUN apt-get update && apt-get install -y \
    ca-certificates \
    libssl3 \
    curl \
    && rm -rf /var/lib/apt/lists/*

COPY --from=builder /usr/local/bin/coral-api /usr/local/bin/

ENV RUST_LOG=info
EXPOSE 8000

CMD ["coral-api"]

# Runtime stage for coral-admin
FROM debian:bookworm-slim AS coral-admin

RUN apt-get update && apt-get install -y \
    ca-certificates \
    libssl3 \
    && rm -rf /var/lib/apt/lists/*

COPY --from=builder /usr/local/bin/coral-admin /usr/local/bin/

ENV RUST_LOG=info
EXPOSE 8080

CMD ["coral-admin"]

# Postgres with migrations baked in
FROM postgres:16-alpine AS coral-postgres

COPY migrations/*.sql /docker-entrypoint-initdb.d/
