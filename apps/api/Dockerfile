# Tuned for Render free tier / GitHub Actions image builds.
# Prefer building the image on GitHub (more RAM), then run the prebuilt image on Render.
FROM rust:1-bookworm AS builder
WORKDIR /app

ENV CARGO_HOME=/usr/local/cargo \
    CARGO_TERM_COLOR=never \
    CARGO_INCREMENTAL=0 \
    CARGO_BUILD_JOBS=1 \
    CARGO_NET_GIT_FETCH_WITH_CLI=true \
    RUSTFLAGS="-C debuginfo=0"

COPY Cargo.toml Cargo.lock ./
COPY src ./src
COPY migrations ./migrations

# release-free: no LTO, more codegen units → far less peak RAM than --release
RUN cargo build --profile release-free -j 1 \
  && strip target/release-free/customer-ops-api

FROM debian:bookworm-slim AS runtime
RUN useradd --system --create-home --uid 10001 appuser \
  && apt-get update \
  && apt-get install -y --no-install-recommends ca-certificates \
  && rm -rf /var/lib/apt/lists/*
WORKDIR /app
COPY --from=builder /app/target/release-free/customer-ops-api /app/customer-ops-api
USER 10001
EXPOSE 8080
# Render injects PORT; Config falls back from PORT → API_PORT.
CMD ["/app/customer-ops-api"]
