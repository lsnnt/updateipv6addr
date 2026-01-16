# ---------- Build stage ----------
FROM rust:1.92-bullseye AS builder

LABEL org.opencontainers.image.source=https://github.com/lsnnt/updateipv6addr
LABEL org.opencontainers.image.description="updates the ipv6 address for a domain for cloudflare"
LABEL org.opencontainers.image.licenses="AGPL-3.0-or-later"

WORKDIR /app

COPY Cargo.toml Cargo.lock ./
COPY src ./src
RUN cargo build --release


# ---------- Runtime stage ----------
FROM debian:bookworm-slim

RUN apt-get update && \
    apt-get install -y ca-certificates && \
    rm -rf /var/lib/apt/lists/*

WORKDIR /app

COPY --from=builder /app/target/release/updateipv6addr /app/updateipv6addr

USER 1000

CMD ["./updateipv6addr"]
