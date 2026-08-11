# Stage 1: Build frontend
FROM node:20-alpine AS frontend-builder

WORKDIR /app/frontend

COPY frontend/package.json frontend/package-lock.json ./
RUN npm ci

COPY frontend/ ./
RUN npm run build

# Stage 2: Build Rust backend with cargo-chef
FROM rust:latest AS chef
WORKDIR /app
RUN cargo install cargo-chef --locked

FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder
# Install mold (fast linker) and sccache (shared compilation cache)
RUN apt-get update && apt-get install -y mold && rm -rf /var/lib/apt/lists/*
RUN cargo install sccache --locked

# Use mold as linker and sccache for caching
ENV RUSTFLAGS="-C link-arg=-fuse-ld=mold -D warnings"
ENV SCCACHE_DIR=/app/.sccache

# Build dependencies (this layer is cached!)
COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json

# Build application
COPY . .
RUN sccache --start-server 2>/dev/null || true && cargo build --release -p lievre-api

# Stage 3: Runtime
FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

# Copy backend binary
COPY --from=builder /app/target/release/lievre /usr/local/bin/

# Copy frontend build
COPY --from=frontend-builder /app/frontend/dist /app/static

RUN mkdir -p /app/data

EXPOSE 3000

CMD ["lievre"]
