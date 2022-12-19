use crate::{db, repository};
use axum::http::StatusCode;
use axum::{routing::get, Extension, Router};
use prometheus::{IntGauge, Registry, TextEncoder};
use std::net::SocketAddr;
use std::sync::Arc;
use std::time::Duration;

const METRICS_ROUTE: &str = "/metrics";
const COLLECTION_INTERVAL_SEC: u64 = 10;

lazy_static::lazy_static! {
    pub static ref METRICS_REGISTRY: Registry = Registry::new();

    pub static ref FROM_BLOCK: IntGauge = IntGauge::new("from_block", "From block").unwrap();

    pub static ref TO_BLOCK: IntGauge = IntGauge::new("to_block", "To block").unwrap();

    pub static ref CURRENT_BLOCK: IntGauge = IntGauge::new("current_block", "Current block").unwrap();

    pub static ref SAVED_BLOCKS: IntGauge = IntGauge::new(
        "saved_blocks",
        "Number of blocks saved to the database"
    ).unwrap();

    pub static ref SAVED_TRANSACTIONS: IntGauge = IntGauge::new(
        "saved_transactions",
        "Number of transactions saved to the database"
    ).unwrap();

    pub static ref NEW_BLOCKS_PER_MINUTE: IntGauge = IntGauge::new(
        "new_blocks_per_minute",
        "Number of new blocks per minute"
    ).unwrap();

    pub static ref HISTORY_FETCHING_PROGRESS: IntGauge = IntGauge::new(
        "history_fetching_progress",
        "Progress of the history fetching"
    ).unwrap();
}

pub fn register() {
    METRICS_REGISTRY
        .register(Box::new(FROM_BLOCK.clone()))
        .unwrap();

    METRICS_REGISTRY
        .register(Box::new(TO_BLOCK.clone()))
        .unwrap();

    METRICS_REGISTRY
        .register(Box::new(CURRENT_BLOCK.clone()))
        .unwrap();

    METRICS_REGISTRY
        .register(Box::new(SAVED_BLOCKS.clone()))
        .unwrap();

    METRICS_REGISTRY
        .register(Box::new(SAVED_TRANSACTIONS.clone()))
        .unwrap();

    METRICS_REGISTRY
        .register(Box::new(NEW_BLOCKS_PER_MINUTE.clone()))
        .unwrap();

    METRICS_REGISTRY
        .register(Box::new(HISTORY_FETCHING_PROGRESS.clone()))
        .unwrap();
}

pub fn start_prometheus_server(addr: SocketAddr) {
    let app = Router::new().route(METRICS_ROUTE, get(metrics));

    tokio::spawn(async move {
        axum::Server::bind(&addr)
            .serve(app.into_make_service())
            .await
            .unwrap();
    });
}

async fn metrics() -> (StatusCode, String) {
    let metrics_families = METRICS_REGISTRY.gather();
    match TextEncoder.encode_to_string(&metrics_families) {
        Ok(metrics) => (StatusCode::OK, metrics),
        Err(error) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("unable to encode metrics: {error}"),
        ),
    }
}

pub async fn start_data_collector(db: Arc<db::DB>) {
    let repository = repository::Repository::new();
    let mut interval = tokio::time::interval(Duration::from_secs(COLLECTION_INTERVAL_SEC));
    loop {
        interval.tick().await;

        let saved_blocks = repository.get_saved_blocks_count(&*db).await.unwrap();
        let saved_transactions = repository.get_saved_transactions_count(&*db).await.unwrap();

        SAVED_BLOCKS.set(saved_blocks);
        SAVED_TRANSACTIONS.set(saved_transactions);
    }
}
