FROM rust:1.85-slim AS builder

RUN rustup target add wasm32-wasip1 && \
    rustup component add rust-std --target wasm32-wasip1

WORKDIR /workspace
COPY . .
RUN cargo fetch
RUN cargo build --release --target wasm32-wasip1

FROM scratch
WORKDIR /
COPY --from=builder /workspace/target/wasm32-wasip1/release/release_monitor_id.wasm /plugin.wasm
