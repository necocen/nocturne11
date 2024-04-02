ARG RUST_VERSION=1.77


FROM rust:$RUST_VERSION-bookworm AS chef
RUN cargo install cargo-chef@0.1.62
WORKDIR /diesel
RUN cargo install diesel_cli@2.1.1 --no-default-features --features postgres --root .
WORKDIR /nocturne

FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef as build-rust
COPY --from=planner /nocturne/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json
COPY . .
RUN cargo build --release \
    && mkdir /tmp-lib \
    && ldd /nocturne/target/release/server | grep "=> /" | awk '{print $3}' | xargs -I '{}' cp -v '{}' /tmp-lib/ \
    && ldd /diesel/bin/diesel | grep "=> /" | awk '{print $3}' | xargs -I '{}' cp -v '{}' /tmp-lib/

FROM oven/bun:1.0.30-slim AS build-js
WORKDIR /nocturne
COPY ./frontend .
RUN --mount=type=secret,id=VITE_PUBLIC_GOOGLE_CLIENT_ID \
    export VITE_PUBLIC_GOOGLE_CLIENT_ID="$(cat /run/secrets/VITE_PUBLIC_GOOGLE_CLIENT_ID)" \
    && bun install --frozen-lockfile \
    && bun run build

# server
FROM gcr.io/distroless/cc-debian12 AS server
WORKDIR /nocturne
COPY --from=build-rust /tmp-lib /tmp-lib
COPY --from=build-rust /nocturne/target/release/server .
COPY --from=build-rust /diesel/bin/diesel .
COPY --from=build-js /nocturne/dist/assets ./static
COPY ./infrastructure/migrations /nocturne/migrations
COPY ./infrastructure/diesel.toml /nocturne/diesel.toml
ENV LD_LIBRARY_PATH=/tmp-lib
ENTRYPOINT ["./server"]
CMD ["--bind", "0.0.0.0", "--static", "./static", "--production"]
