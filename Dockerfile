ARG DSN
ARG RPC_URL

FROM rust:latest as builder

WORKDIR /usr/src/app
COPY . .

RUN cargo install --path .

FROM debian:buster-slim

COPY --from=builder /usr/local/cargo/bin/ethereum-indexer /usr/local/bin/ethereum-indexer

RUN apt-get update && apt-get install -y openssl && rm -rf /var/lib/apt/lists/*

CMD ["ethereum-indexer", "--migrate", "--dsn", "${DSN}", "--rpc-url", "${RPC_URL}"]