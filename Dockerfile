ARG RUST_VERSION=1.54

FROM rust:$RUST_VERSION AS planner
WORKDIR /nocturne
RUN cargo install cargo-chef
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM rust:$RUST_VERSION as cacher
WORKDIR /nocturne
RUN cargo install cargo-chef
COPY --from=planner /nocturne/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json

FROM rust:$RUST_VERSION as build-rust
WORKDIR /nocturne
COPY . .
COPY --from=cacher /nocturne/target target
COPY --from=cacher $CARGO_HOME $CARGO_HOME
RUN cargo build --release

FROM rust:$RUST_VERSION as diesel-cli
WORKDIR /diesel
RUN cargo install diesel_cli --no-default-features --features postgres --root .

FROM node:16-alpine AS build-js
ARG SNOWPACK_PUBLIC_GOOGLE_CLIENT_ID
WORKDIR /nocturne
COPY ./frontend/package.json ./package.json
COPY ./frontend/package-lock.json ./package-lock.json
RUN npm ci
COPY ./frontend .
RUN npm ci
RUN npm run build

FROM debian:buster-slim AS deps
RUN apt-get update -y && \
    apt-get install -y \
        libcom-err2 \
        libffi6 \
        libgmp10 \
        libgnutls30 \
        libgssapi-krb5-2 \
        libhogweed4 \
        libidn2-0 \
        libk5crypto3 \
        libkeyutils1 \
        libkrb5-3 \
        libkrb5support0 \
        libldap-2.4-2 \
        libmariadb3 \
        libnettle6 \
        libp11-kit0 \
        libpq5 \
        libsasl2-2 \
        libtasn1-6 \
        libunistring2 \
        zlib1g

# server
FROM gcr.io/distroless/cc-debian10 AS base
COPY --from=deps /lib/x86_64-linux-gnu/libz.so.1 /lib/x86_64-linux-gnu/libz.so.1
COPY --from=deps /lib/x86_64-linux-gnu/libcom_err.so.2 /lib/x86_64-linux-gnu/libcom_err.so.2
COPY --from=deps /lib/x86_64-linux-gnu/libkeyutils.so.1 /lib/x86_64-linux-gnu/libkeyutils.so.1
COPY --from=deps /usr/lib/x86_64-linux-gnu/libffi.so.6 /usr/lib/x86_64-linux-gnu/libffi.so.6
COPY --from=deps /usr/lib/x86_64-linux-gnu/libgmp.so.10 /usr/lib/x86_64-linux-gnu/libgmp.so.10
COPY --from=deps /usr/lib/x86_64-linux-gnu/libgnutls.so.30 /usr/lib/x86_64-linux-gnu/libgnutls.so.30
COPY --from=deps /usr/lib/x86_64-linux-gnu/libgssapi_krb5.so.2 /usr/lib/x86_64-linux-gnu/libgssapi_krb5.so.2
COPY --from=deps /usr/lib/x86_64-linux-gnu/libhogweed.so.4 /usr/lib/x86_64-linux-gnu/libhogweed.so.4
COPY --from=deps /usr/lib/x86_64-linux-gnu/libidn2.so.0 /usr/lib/x86_64-linux-gnu/libidn2.so.0
COPY --from=deps /usr/lib/x86_64-linux-gnu/libk5crypto.so.3 /usr/lib/x86_64-linux-gnu/libk5crypto.so.3
COPY --from=deps /usr/lib/x86_64-linux-gnu/libkrb5.so.3 /usr/lib/x86_64-linux-gnu/libkrb5.so.3
COPY --from=deps /usr/lib/x86_64-linux-gnu/libkrb5support.so.0 /usr/lib/x86_64-linux-gnu/libkrb5support.so.0
COPY --from=deps /usr/lib/x86_64-linux-gnu/liblber-2.4.so.2 /usr/lib/x86_64-linux-gnu/liblber-2.4.so.2
COPY --from=deps /usr/lib/x86_64-linux-gnu/libldap_r-2.4.so.2 /usr/lib/x86_64-linux-gnu/libldap_r-2.4.so.2
COPY --from=deps /usr/lib/x86_64-linux-gnu/libmariadb.so.3 /usr/lib/x86_64-linux-gnu/libmariadb.so.3
COPY --from=deps /usr/lib/x86_64-linux-gnu/libnettle.so.6 /usr/lib/x86_64-linux-gnu/libnettle.so.6
COPY --from=deps /usr/lib/x86_64-linux-gnu/libp11-kit.so.0 /usr/lib/x86_64-linux-gnu/libp11-kit.so.0
COPY --from=deps /usr/lib/x86_64-linux-gnu/libpq.so.5 /usr/lib/x86_64-linux-gnu/libpq.so.5
COPY --from=deps /usr/lib/x86_64-linux-gnu/libsasl2.so.2 /usr/lib/x86_64-linux-gnu/libsasl2.so.2
COPY --from=deps /usr/lib/x86_64-linux-gnu/libtasn1.so.6 /usr/lib/x86_64-linux-gnu/libtasn1.so.6
COPY --from=deps /usr/lib/x86_64-linux-gnu/libunistring.so.2 /usr/lib/x86_64-linux-gnu/libunistring.so.2

FROM base AS server
WORKDIR /nocturne
COPY --from=build-rust /nocturne/target/release/server .
COPY --from=build-js /nocturne/build/src ./static
ENTRYPOINT ["./server"]
CMD ["--bind", "0.0.0.0", "--static", "./static", "--production"]

FROM base AS import
WORKDIR /nocturne
COPY --from=build-rust /nocturne/target/release/import .
ENTRYPOINT ["./import"]

FROM base AS migrate
WORKDIR /nocturne
COPY --from=build-rust /nocturne/target/release/migrate .
COPY --from=diesel-cli /diesel/bin/diesel .
# なんかdieselのためにCargo.tomlが必要なので置いておく。空でよいがtouchがないのでCOPYする
COPY ./Cargo.toml .
ENTRYPOINT ["./migrate"]
