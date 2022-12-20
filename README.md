# Ethereum indexer

Crawls data from the ethereum blockchain to the PostgreSQL database.

## Install

You need [Rust](https://www.rust-lang.org/) + [Cargo](https://doc.rust-lang.org/cargo/) installed.

1. Clone source files
```bash
$ git clone https://github.com/asadkovich/ethereum-indexer
```

2. Build and install binary
```bash
$ cargo install --path .
```

## Usage

Start indexer with fetching the entire data history and subscribing to new blocks:
```bash
$ ethereum-indexer --migrate --dsn <POSTGRES_CONN_STRING> --rpc-url <ETHEREUM_WS_ENDPOINT>
```

See help for more:
```bash
$ ethereum-indexer --help
```

## Run docker example
```bash
$ RPC_URL=<ETHEREUM_WS_ENDPOINT> docker-compose -f ./deployments/docker-compose.yaml up --build
```

### Metrics
Open [http://localhost:3000](http://localhost:3000) to see grafana dashboards
<p align="center">
  <img src="https://user-images.githubusercontent.com/32710511/208610346-584177f5-c4d9-4b27-9dc6-f5018fc13bf0.jpg" height="350"/>
</p>
