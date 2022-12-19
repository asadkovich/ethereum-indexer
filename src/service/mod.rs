mod fetcher;
mod processor;
mod subscriber;

use crate::db::DB;
use crate::metrics;
use crate::rpc::RPC;
use crate::service::fetcher::Fetcher;
use crate::service::processor::BlockProcessor;
use crate::service::subscriber::Subscriber;
use std::sync::Arc;
use tokio::task;

/// Service is responsible for fetching historical blocks
/// and subscribing to new blocks.
#[derive(Debug)]
pub struct Service {
    processor: Arc<BlockProcessor>,
    rpc: Arc<RPC>,
    fetch_tasks: Vec<task::JoinHandle<Result<(), crate::Error>>>,
    subscribe_task: Option<task::JoinHandle<Result<(), crate::Error>>>,
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

    pub async fn start_fetching(
        &mut self,
        from_block: Option<i64>,
        to_block: Option<i64>,
    ) -> Result<(), crate::Error> {
        let from = match from_block {
            Some(block) => block,
            None => self.rpc.eth().block_number().await?.as_u64() as i64,
        };
        let to = match to_block {
            Some(block) => block,
            None => 0,
        };

        metrics::FROM_BLOCK.set(from);
        metrics::TO_BLOCK.set(to);

        // TODO: maybe we should split the blocks range into smaller chunks.
        let fetch_task = tokio::spawn({
            let fetcher = Fetcher::new(self.processor.clone(), self.rpc.clone(), from, to);
            async move { fetcher.run().await }
        });
        self.fetch_tasks.push(fetch_task);

        Ok(())
    }

    pub fn start_subscribing(&mut self) -> Result<(), crate::Error> {
        let subscribe_task = tokio::spawn({
            let subscriber = Subscriber::new(self.processor.clone(), self.rpc.clone());
            async move { subscriber.run().await }
        });
        self.subscribe_task = Some(subscribe_task);

        Ok(())
    }
}
