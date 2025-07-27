FROM rust:slim AS builder

RUN apt-get update && apt-get install -y \
    pkg-config libssl-dev build-essential curl git \
    && rm -rf /var/lib/apt/lists/*

RUN rustup target add x86_64-unknown-linux-gnu

WORKDIR /app

COPY . .

# Build for the target
RUN cargo build --release --target x86_64-unknown-linux-gnu
