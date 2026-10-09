# syntax=docker/dockerfile:1

FROM rust:1.98.1-trixie AS builder

ARG DIOXUS_CLI_VERSION=0.7.10
RUN apt-get update \
    && apt-get install -y --no-install-recommends pkg-config libssl-dev \
    && rm -rf /var/lib/apt/lists/*
RUN rustup target add wasm32-unknown-unknown \
    && cargo install dioxus-cli --version "${DIOXUS_CLI_VERSION}" --locked

WORKDIR /build
COPY Cargo.toml Cargo.lock Dioxus.toml ./
COPY src ./src
COPY assets ./assets

RUN dx bundle --web --fullstack true --release --locked --out-dir /dist \
    && test -x /dist/server \
    && test -f /dist/public/index.html

FROM debian:trixie-slim AS runtime

RUN apt-get update \
    && apt-get install -y --no-install-recommends ca-certificates libssl3t64 curl \
    && rm -rf /var/lib/apt/lists/* \
    && useradd --system --uid 10001 --user-group busymonkey

WORKDIR /app
COPY --from=builder --chown=busymonkey:busymonkey /dist/ ./
USER busymonkey

ENV IP=0.0.0.0 PORT=8080
EXPOSE 8080
ENTRYPOINT ["./server"]
CMD ["/app/config.toml"]
