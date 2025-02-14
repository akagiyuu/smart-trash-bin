FROM lukemathwalker/cargo-chef:latest-rust-1 AS chef
WORKDIR /app

FROM chef AS planner
COPY Cargo.toml Cargo.lock ./
COPY src/ ./src
COPY .sqlx/ ./.sqlx
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder 
COPY --from=planner /app/recipe.json recipe.json
# Build dependencies - this is the caching Docker layer!
RUN cargo chef cook --release --recipe-path recipe.json
# Build application
COPY Cargo.toml Cargo.lock ./
COPY src/ ./src
COPY .sqlx/ ./.sqlx
RUN cargo build --release 

FROM debian:bookworm

RUN apt-get update \
    && apt-get install libssl3 -y \
    && apt-get clean

WORKDIR /app

COPY --from=builder /app/target/release/backend .

CMD ["./backend"]
