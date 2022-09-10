FROM rust:alpine as builder
WORKDIR /app
COPY Cargo.toml Cargo.toml
COPY Cargo.lock Cargo.lock
COPY src/ src/
RUN cargo build --release

FROM alpine
WORKDIR /app
COPY --from=builder /app/target/release/atb_backend /app/atb_backend