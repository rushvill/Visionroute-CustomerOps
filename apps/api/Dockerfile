# Render free instances often OOM on parallel Rust release builds.
FROM rust:1-bookworm AS builder
WORKDIR /app

ENV CARGO_HOME=/usr/local/cargo \
    CARGO_TERM_COLOR=never \
    CARGO_INCREMENTAL=0 \
    CARGO_BUILD_JOBS=1 \
    RUSTFLAGS="-C debuginfo=0"

COPY Cargo.toml Cargo.lock ./
COPY src ./src
COPY migrations ./migrations

RUN cargo build --release -j 1 \
  && strip /app/target/release/customer-ops-api || true

FROM debian:bookworm-slim AS runtime
RUN useradd --system --create-home --uid 10001 appuser \
  && apt-get update \
  && apt-get install -y --no-install-recommends ca-certificates \
  && rm -rf /var/lib/apt/lists/*
WORKDIR /app
COPY --from=builder /app/target/release/customer-ops-api /app/customer-ops-api
USER 10001
EXPOSE 8080
# Render injects PORT; our Config already falls back from PORT → API_PORT.
CMD ["/app/customer-ops-api"]
