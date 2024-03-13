ARG RUST_VERSION=1.76


FROM --platform=linux/x86_64 rust:$RUST_VERSION-bookworm AS chef
RUN cargo install cargo-chef@0.1.62
WORKDIR /nocturne


FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef as build-rust
COPY --from=planner /nocturne/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json
COPY . .
RUN cargo build --release


FROM --platform=linux/x86_64 rust:$RUST_VERSION-bookworm as diesel-cli
WORKDIR /diesel
RUN cargo install diesel_cli --no-default-features --features postgres --root .


FROM oven/bun:1.0.30-slim AS build-js
WORKDIR /nocturne
COPY ./frontend .
RUN --mount=type=secret,id=VITE_PUBLIC_GOOGLE_CLIENT_ID \
    export VITE_PUBLIC_GOOGLE_CLIENT_ID="$(cat /run/secrets/VITE_PUBLIC_GOOGLE_CLIENT_ID)" && \
    bun install --frozen-lockfile && bun run build

FROM --platform=linux/x86_64 debian:bookworm-slim AS deps
RUN apt-get update -y && \
    apt-get install -y \
    libcom-err2 \
    libffi8 \
    libgmp10 \
    libgnutls30 \
    libgssapi-krb5-2 \
    libhogweed6 \
    libidn2-0 \
    libk5crypto3 \
    libkeyutils1 \
    libkrb5-3 \
    libkrb5support0 \
    libldap-2.5-0 \
    libmariadb3 \
    libnettle8 \
    libp11-kit0 \
    libpq5 \
    libssl3 \
    libsasl2-2 \
    libtasn1-6 \
    libunistring2 \
    zlib1g


# server
FROM --platform=linux/x86_64 gcr.io/distroless/cc-debian12 AS base
COPY --from=deps /usr/lib/x86_64-linux-gnu/* /usr/lib/x86_64-linux-gnu/


FROM base AS server
WORKDIR /nocturne
COPY --from=build-rust /nocturne/target/release/server .
COPY --from=build-rust /nocturne/target/release/migrate .
COPY --from=diesel-cli /diesel/bin/diesel .
COPY --from=build-js /nocturne/dist/assets ./static
ENTRYPOINT ["./server"]
CMD ["--bind", "0.0.0.0", "--static", "./static", "--production"]
