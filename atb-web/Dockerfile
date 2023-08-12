FROM docker.io/rustlang/rust:nightly-slim as builder
WORKDIR /app
RUN apt-get update && apt-get install -y npm
RUN cargo install cargo-leptos
COPY . .
RUN cargo leptos build --release

FROM docker.io/rustlang/rust:nightly-slim
# Copy the server binary to the /app directory
COPY --from=builder /app/target/server/release/atb-web /app/
# /target/site contains our JS/WASM/CSS, etc.
COPY --from=builder /app/target/site /app/site
# Copy Cargo.toml if it’s needed at runtime
COPY --from=builder /app/Cargo.toml /app/
WORKDIR /app
CMD ["/app/atb-web"]
