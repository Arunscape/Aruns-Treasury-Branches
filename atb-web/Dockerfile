FROM docker.io/rust:slim as builder
#FROM docker.io/rustlang/rust:nightly-slim as builder
WORKDIR /app
RUN apt-get update && apt-get install -y npm pkg-config
RUN cargo install cargo-leptos
COPY .cargo .cargo
COPY rust-toolchain.toml rust-toolchain.toml
COPY src src
COPY public public
COPY style style
COPY tailwind.config.js tailwind.config.js
COPY Cargo.lock Cargo.lock
COPY Cargo.toml Cargo.toml
COPY package.json package.json
COPY package-lock.json package-lock.json
RUN npm install
RUN cargo leptos build --release

#FROM docker.io/rustlang/rust:nightly-slim
FROM gcr.io/distroless/cc
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
