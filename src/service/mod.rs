mod fetcher;
mod processor;
mod subscriber;

use crate::db::DB;
use crate::rpc::RPC;
use crate::service::fetcher::Fetcher;
use crate::service::processor::BlockProcessor;
use crate::service::subscriber::Subscriber;
use std::sync::Arc;

/// Service is responsible for fetching historical blocks
/// and subscribing to new blocks.
#[derive(Debug)]
pub struct Service {
    processor: Arc<BlockProcessor>,
    rpc: Arc<RPC>,
    fetch_tasks: Vec<tokio::task::JoinHandle<Result<(), crate::Error>>>,
    subscribe_task: Option<tokio::task::JoinHandle<Result<(), crate::Error>>>,
}

impl Service {
    pub fn new(db: Arc<DB>, rpc: Arc<RPC>) -> Self {
        let processor = Arc::new(BlockProcessor::new(db));

        Service {
            processor,
            rpc,
            fetch_tasks: Vec::new(),
            subscribe_task: None,
        }
    }

    pub fn start_fetching(&mut self, from: Option<i64>, to: Option<i64>) {
        // TODO: maybe we should split the blocks range into smaller chunks.
        let fetch_task = tokio::spawn({
            let fetcher = Fetcher::new(self.processor.clone(), self.rpc.clone(), from, to);
            async move { fetcher.run().await }
        });
        self.fetch_tasks.push(fetch_task);
    }

    pub fn start_subscribing(&mut self) {
        let subscribe_task = tokio::spawn({
            let subscriber = Subscriber::new(self.processor.clone(), self.rpc.clone());
            async move { subscriber.run().await }
        });
        self.subscribe_task = Some(subscribe_task);
    }
}
