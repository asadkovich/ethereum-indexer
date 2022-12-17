mod db;
mod entities;
mod errors;
mod repository;
mod rpc;
mod service;

pub use errors::Error;
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

    /// Database pool size limit (default is 0, which means there is no limit)
    /// This is useful when you want to limit the number of connections to the database.
    #[arg(long, default_value_t = 0)]
    pool_size: u32,
}

#[tokio::main]
async fn main() {
    let args = Args::parse();

    let db = db::connect(&args.dsn, args.pool_size).await.unwrap();
    let rpc = rpc::connect(&args.rpc_url).await.unwrap();

    if args.migrate {
        db::migrate(&db).await.unwrap();
        log::info!("[OK] Migrations completed");
    }

    let mut service = Service::new(Arc::new(db), Arc::new(rpc));

    if args.fetch {
        service.start_fetching(args.from, args.to);
        log::info!("[OK] Fetcher started");
    }

    if args.subscribe {
        service.start_subscribing();
        log::info!("[OK] Subscriber started");
    }

    tokio::signal::ctrl_c().await.unwrap();
}
