use crate::db::DB;
use crate::metrics;
use crate::rpc::RPC;
use crate::service::processor::BlockProcessor;
use crate::{db, entities};
use std::sync::Arc;
use std::thread;
use web3::types::{Block, BlockId, BlockNumber, Transaction, U64};

/// Fetcher is responsible for fetching historical blocks.
pub struct Fetcher {
    processor: Arc<BlockProcessor>,
    rpc: Arc<RPC>,
    from_block: i64,
    to_block: i64,
}

impl Fetcher {
    pub fn new(
        processor: Arc<BlockProcessor>,
        rpc: Arc<RPC>,
        from_block: i64,
        to_block: i64,
    ) -> Self {
        Fetcher {
            processor,
            rpc,
            from_block,
            to_block,
        }
    }

    /// Reads blockchain from top to bottom and stores data in the database.
    pub async fn run(&self) -> Result<(), crate::Error> {
        for num in self.from_block..self.to_block {
            let block = self
                .rpc
                .eth()
                .block_with_txs(BlockId::Number(BlockNumber::Number(U64::from(num))))
                .await
                .unwrap();

            if let Some(block) = block {
                tokio::spawn({
                    let processor = self.processor.clone();
                    async move {
                        processor.process_block(block).await;
                    }
                });

                metrics::CURRENT_BLOCK.set(num);
                metrics::HISTORY_FETCHING_PROGRESS.set(Self::calculate_progress(
                    self.from_block,
                    self.to_block,
                    num,
                ));
            }
        }

        Ok(())
    }

    fn calculate_progress(from: i64, to: i64, current: i64) -> i64 {
        let total = from - to;
        let done = from - current;
        let progress = (done as f64 / total as f64) * 100.0;

        progress.round() as i64
    }
}

#[cfg(test)]
mod tests {
    use crate::service::fetcher::Fetcher;

    #[tokio::test]
    pub async fn test_calculate_progress() {
        let progress = Fetcher::calculate_progress(500, 0, 493);
        assert_eq!(progress, 1);

        let progress = Fetcher::calculate_progress(100, 0, 73);
        assert_eq!(progress, 27);

        let progress = Fetcher::calculate_progress(100, 0, 0);
        assert_eq!(progress, 100);
    }
}
