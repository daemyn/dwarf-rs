FROM rust:bookworm AS builder

RUN apt-get update && apt-get install -y --no-install-recommends \
    libpq-dev pkg-config build-essential ca-certificates && \
    rm -rf /var/lib/apt/lists/*

WORKDIR /app

COPY Cargo.toml Cargo.lock ./

RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN cargo build --release

COPY . .

RUN cargo install sqlx-cli

ENV SQLX_OFFLINE=true

RUN cargo build --release

FROM debian:bookworm-slim AS runtime

RUN apt-get update && apt-get install -y --no-install-recommends \
    libpq5 ca-certificates && \
    rm -rf /var/lib/apt/lists/*

RUN useradd -m -u 1000 appuser

WORKDIR /app

COPY --from=builder /app/target/release/dwarf-rs /usr/local/bin/dwarf-rs

COPY --from=builder /usr/local/cargo/bin/sqlx /usr/local/bin/sqlx

COPY --from=builder /app/migrations ./migrations

RUN chown -R appuser:appuser /app

USER appuser

CMD ["dwarf-rs"]
