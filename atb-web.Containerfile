FROM docker.io/rust:latest as builder
#FROM ghcr.io/rust-lang/rust:nightly as builder

RUN apt-get update && apt-get install -y npm
#RUN cargo install cargo-leptos

# Install cargo-binstall, which makes it easier to install other
# cargo extensions like cargo-leptos
RUN wget https://github.com/cargo-bins/cargo-binstall/releases/latest/download/cargo-binstall-x86_64-unknown-linux-musl.tgz
RUN tar -xvf cargo-binstall-x86_64-unknown-linux-musl.tgz
RUN cp cargo-binstall /usr/local/cargo/bin

RUN cargo binstall cargo-leptos -y
RUN rustup target add wasm32-unknown-unknown

WORKDIR /app
COPY .cargo .cargo
COPY atb-web/.cargo atb-web/.cargo
COPY . .
WORKDIR /app/atb-web
RUN npm install
RUN cargo leptos build --release -vv

#FROM docker.io/rustlang/rust:nightly-slim
#FROM docker.io/debian:stable-slim
FROM gcr.io/distroless/cc:latest
# Copy the server binary to the /app directory
COPY --from=builder /app/target/server/release/atb-web /app/
# /target/site contains our JS/WASM/CSS, etc.
COPY --from=builder /app/target/site /app/site
# Copy Cargo.toml if it’s needed at runtime
COPY --from=builder /app/Cargo.toml /app/

WORKDIR /app
ENV RUST_LOG="info"
ENV APP_ENVIRONMENT="production"
ENV LEPTOS_SITE_ADDR="0.0.0.0:8080"
ENV LEPTOS_SITE_ROOT="site"
EXPOSE 8080
# Run the server
CMD ["/app/atb-web"]
