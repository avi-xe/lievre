FROM rust:1.77 as builder

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

# Runtime
FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/release/lievre /usr/local/bin/

RUN mkdir -p /app/data

EXPOSE 3000

CMD ["lievre"]
