mod db;
mod entities;
mod errors;
mod metrics;
mod repository;
mod rpc;
mod service;

pub use errors::Error;
use std::str::FromStr;
use std::sync::Arc;

use crate::service::Service;
use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Postgres connection string.
    #[arg(long)]
    dsn: String,

    /// Ethereum RPC URL.
    #[arg(long)]
    rpc_url: String,

    /// Enables subscription to new blocks (true by default).
    #[arg(long, short, default_value_t = true)]
    subscribe: bool,

    /// Enables fetching of historical blocks (true by default).
    /// If set to false, only new blocks will be fetched.
    #[arg(long, short, default_value_t = true)]
    fetch: bool,

    /// Run migrations (false by default).
    #[arg(long, short, default_value_t = false)]
    migrate: bool,

    /// Specifies the starting block number.
    /// If not specified, the fetcher will start from the latest block.
    #[arg(long)]
    from: Option<i64>,

    /// Specifies the ending block number.
    /// If not specified, the fetcher will fetch all the way to the genesis block
    /// (which is block number 0).
    #[arg(long)]
    to: Option<i64>,

    /// Disables or enables verbose mode (true by default).
    #[arg(long, short, default_value_t = true)]
    verbose: bool,

    /// Database pool size limit.
    /// This is useful when you want to limit the number of connections to the database.
    #[arg(long, default_value_t = 30)]
    pool_size: u32,

    /// Enables or disables metrics (true by default).
    #[arg(long, default_value_t = true)]
    metrics: bool,

    /// Specifies the metrics port (0.0.0.0:7070 by default).
    #[arg(long, default_value = "0.0.0.0:7070")]
    metrics_addr: String,
}

#[tokio::main]
async fn main() {
    let args: Args = Args::parse();

    let db = Arc::new(db::connect(&args.dsn, args.pool_size).await.unwrap());
    let rpc = Arc::new(rpc::connect(&args.rpc_url).await.unwrap());

    if args.migrate {
        db::migrate(&db).await.unwrap();
        log::info!("[OK] Migrations completed");
    }

    if args.metrics {
        let metrics_addr =
            std::net::SocketAddr::from_str(&args.metrics_addr).expect("Invalid metrics address");
        metrics::start_prometheus_server(metrics_addr);
        metrics::register();

        tokio::task::spawn(metrics::start_data_collector(db.clone()));

        log::info!(
            "[OK] Started Prometheus HTTP endpoint at {}",
            args.metrics_addr
        );
    }

    let mut service = Service::new(db, rpc);

    if args.fetch {
        service.start_fetching(args.from, args.to).await.unwrap();
        log::info!("[OK] Fetcher started");
    }

    if args.subscribe {
        service.start_subscribing().unwrap();
        log::info!("[OK] Subscriber started");
    }

    tokio::signal::ctrl_c().await.unwrap();
}
