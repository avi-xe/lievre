# Stage 1: Build frontend
FROM node:20-alpine as frontend-builder

WORKDIR /app/frontend

COPY frontend/package.json frontend/package-lock.json ./
RUN npm ci

COPY frontend/ ./
RUN npm run build

# Stage 2: Build Rust backend
FROM rust:latest as backend-builder

WORKDIR /app

# Cache dependencies
COPY Cargo.toml Cargo.lock ./
COPY crates/*/Cargo.toml crates/
RUN mkdir -p crates/*/src && \
    for crate in crates/*/; do touch "$crate/src/lib.rs"; done

# Build dependencies (cached)
RUN cargo build --release || true

# Copy source
COPY . .

# Build application
RUN cargo build --release

# Stage 3: Runtime
FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

# Copy backend binary
COPY --from=backend-builder /app/target/release/lievre /usr/local/bin/

# Copy frontend build
COPY --from=frontend-builder /app/frontend/dist /app/static

RUN mkdir -p /app/data

EXPOSE 3000

CMD ["lievre"]
