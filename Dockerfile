FROM rust:1.77-slim

WORKDIR /app
COPY . .

RUN apt-get update && apt-get install -y pkg-config libssl-dev && \
    cargo build --release

CMD ["./target/release/rust_trading_bot"]
