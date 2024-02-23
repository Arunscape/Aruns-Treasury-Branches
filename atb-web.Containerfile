#FROM docker.io/rust:latest as builder
##FROM ghcr.io/rust-lang/rust:nightly as builder
#
#RUN apt-get update && apt-get install -y npm
##RUN cargo install cargo-leptos
#
## Install cargo-binstall, which makes it easier to install other
## cargo extensions like cargo-leptos
#RUN wget https://github.com/cargo-bins/cargo-binstall/releases/latest/download/cargo-binstall-x86_64-unknown-linux-musl.tgz
#RUN tar -xvf cargo-binstall-x86_64-unknown-linux-musl.tgz
#RUN cp cargo-binstall /usr/local/cargo/bin
#
#RUN cargo binstall cargo-leptos -y
#RUN rustup target add wasm32-unknown-unknown
#
#WORKDIR /app
#COPY .cargo .cargo
#COPY atb-web/.cargo atb-web/.cargo
#COPY . .
#WORKDIR /app/atb-web
#RUN npm install
#RUN cargo leptos build --release -vv
#
##FROM docker.io/rustlang/rust:nightly-slim
##FROM docker.io/debian:stable-slim
#FROM gcr.io/distroless/cc:latest
## Copy the server binary to the /app directory
#COPY --from=builder /app/target/server/release/atb-web /app/
## /target/site contains our JS/WASM/CSS, etc.
#COPY --from=builder /app/target/site /app/site
## Copy Cargo.toml if it’s needed at runtime
#COPY --from=builder /app/Cargo.toml /app/
#
#WORKDIR /app
#ENV RUST_LOG="info"
#ENV APP_ENVIRONMENT="production"
#ENV LEPTOS_SITE_ADDR="0.0.0.0:8080"
#ENV LEPTOS_SITE_ROOT="site"
#EXPOSE 8080
## Run the server
#CMD ["/app/atb-web"]
# Get started with a build env with Rust nightly




#FROM docker.io/rustlang/rust:nightly-alpine as builder
FROM docker.io/rust:alpine as builder

RUN apk update && \
    apk add --no-cache bash curl npm libc-dev binaryen

RUN npm install -g sass

RUN curl --proto '=https' --tlsv1.2 -LsSf https://github.com/leptos-rs/cargo-leptos/releases/latest/download/cargo-leptos-installer.sh | sh

# Add the WASM target
#RUN rustup target add wasm32-unknown-unknown

WORKDIR /work
COPY . .

RUN sed s/linux-gnu/linux-musl/ -i .cargo/config.toml
#RUN sed s/linux-gnu/linux-musl/ -i atb-web/.cargo/config.toml
RUN cat .cargo/config.toml
WORKDIR /work/atb-web
RUN npm install
RUN cargo leptos build --release -vv

######################################################

#FROM docker.io/rustlang/rust:nightly-alpine as runner
FROM scratch as runner

WORKDIR /app

COPY --from=builder /work/target/release/atb-web /app/
COPY --from=builder /work/target/site /app/site
COPY --from=builder /work/Cargo.toml /app/

ENV RUST_LOG="info"
ENV LEPTOS_SITE_ADDR="0.0.0.0:8080"
ENV LEPTOS_SITE_ROOT=./site
EXPOSE 8080

CMD ["/app/atb-web"]



## Get started with a build env with Rust nightly
##FROM docker.io/rustlang/rust:nightly-bullseye as builder
#FROM docker.io/rust:latest as builder
#
## If you’re using stable, use this instead
## FROM rust:1.74-bullseye as builder
#
## Install cargo-binstall, which makes it easier to install other
## cargo extensions like cargo-leptos
#RUN wget https://github.com/cargo-bins/cargo-binstall/releases/latest/download/cargo-binstall-x86_64-unknown-linux-musl.tgz
#RUN tar -xvf cargo-binstall-x86_64-unknown-linux-musl.tgz
#RUN cp cargo-binstall /usr/local/cargo/bin
#
## Install cargo-leptos
#RUN cargo binstall cargo-leptos -y
#
#RUN apt-get update && apt-get install -y npm
#
## Add the WASM target
##RUN rustup target add wasm32-unknown-unknown
#
## Make an /app dir, which everything will eventually live in
#RUN mkdir -p /app
#WORKDIR /app
#COPY . .
#
## Build the app
#RUN cd atb-web && npm install && cd ..
#RUN cargo leptos build --release -vv
#
#FROM gcr.io/distroless/base:latest as runner
#
## -- NB: update binary name from "leptos_start" to match your app name in Cargo.toml --
## Copy the server binary to the /app directory
##COPY --from=builder /app/target/release/leptos_start /app/
#COPY --from=builder /app/target/release/atb-web /app/
#
## /target/site contains our JS/WASM/CSS, etc.
#COPY --from=builder /app/target/site /app/site
## Copy Cargo.toml if it’s needed at runtime
#COPY --from=builder /app/Cargo.toml /app/
#WORKDIR /app
#
## Set any required env variables and
#ENV RUST_LOG="info"
#ENV LEPTOS_SITE_ADDR="0.0.0.0:8080"
#ENV LEPTOS_SITE_ROOT="site"
#EXPOSE 8080
#
## -- NB: update binary name from "leptos_start" to match your app name in Cargo.toml --
## Run the server
#CMD ["/app/atb-web"]
#
