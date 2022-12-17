FROM rust:latest as builder

WORKDIR /usr/src/app
COPY . .

RUN cargo install --path .

FROM debian:buster-slim

ARG DSN
ARG RPC_URL

COPY --from=builder /usr/local/cargo/bin/ethereum-indexer /usr/local/bin/ethereum-indexer

RUN apt-get update && apt-get install -y openssl ca-certificates && rm -rf /var/lib/apt/lists/*

# Set environment variables because args only can be used at build time,
# and the CMD is executing at run time.
ENV ENV_DSN=$DSN
ENV ENV_RPC_URL=$RPC_URL

CMD ethereum-indexer --migrate --dsn ${ENV_DSN} --rpc-url ${ENV_RPC_URL}
