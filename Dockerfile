FROM lukemathwalker/cargo-chef:latest-rust-1.88-bookworm@sha256:d184d41fd934afd282bf8f04b602aaa952af88341c48b8738b0c0a85ac7a6cec AS chef

WORKDIR /app

# System dependencies (for prost-build and rocksdb)
RUN apt-get update && apt-get install -y \
    protobuf-compiler \
    clang \
    libclang-dev \
    llvm-dev \
    build-essential \
 && rm -rf /var/lib/apt/lists/*

# Planner Stage
FROM chef AS planner

COPY Cargo.toml Cargo.lock ./
COPY crates ./crates

RUN cargo chef prepare --recipe-path recipe.json

# Builder Stage
FROM chef AS builder

COPY --from=planner /app/recipe.json recipe.json

# Building only the dependencies
RUN cargo chef cook --release --recipe-path recipe.json

COPY Cargo.toml Cargo.lock ./
COPY crates ./crates

# Building the binary
RUN cargo build --release --bin server


# Runtime Stage
FROM debian:bookworm-slim@sha256:f06537653ac770703bc45b4b113475bd402f451e85223f0f2837acbf89ab020a AS runtime

RUN apt-get update && apt-get install -y \
    ca-certificates \
 && rm -rf /var/lib/apt/lists/*

RUN useradd -m vortexdb

WORKDIR /app

RUN mkdir -p /data && chown -R vortexdb:vortexdb /data

COPY --from=builder /app/target/release/server /usr/local/bin/server

# Safe defualts
ENV HTTP_HOST=0.0.0.0
ENV HTTP_PORT=3000
ENV GRPC_HOST=0.0.0.0
ENV GRPC_PORT=50051
ENV STORAGE_TYPE=inmemory
ENV INDEX_TYPE=flat
ENV LOGGING=true    
ENV DISABLE_HTTP=false

EXPOSE 3000
EXPOSE 50051

USER vortexdb

ENTRYPOINT ["server"]
