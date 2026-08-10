# Stage 1: Build frontend
FROM node:20-alpine as frontend-builder

WORKDIR /app/frontend

COPY frontend/package.json frontend/package-lock.json ./
RUN npm ci

COPY frontend/ ./
RUN npm run build

# Stage 2: Build Rust backend with cargo-chef
FROM rust:latest as chef
WORKDIR /app
RUN cargo install cargo-chef --locked

FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder
# Build dependencies (this layer is cached!)
COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json

# Build application
COPY . .
RUN cargo build --release -p lievre-api

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
