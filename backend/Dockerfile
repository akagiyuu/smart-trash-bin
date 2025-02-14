# Use a Rust base image with Cargo installed
FROM rust:latest AS builder

WORKDIR /usr/src/app

COPY ./Cargo.toml ./Cargo.lock ./

RUN cargo build --release

FROM debian:buster-slim

WORKDIR /usr/src/app

COPY --from=builder /usr/src/app/target/release/backend .

CMD ["./backend"]
