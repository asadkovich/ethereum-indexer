use crate::db::DB;
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
    from_block: Option<i64>,
    to_block: Option<i64>,
}

impl Fetcher {
    pub fn new(
        processor: Arc<BlockProcessor>,
        rpc: Arc<RPC>,
        from_block: Option<i64>,
        to_block: Option<i64>,
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
        let from = match self.from_block {
            Some(block) => block,
            None => self.rpc.eth().block_number().await.unwrap().as_u64() as i64,
        };
        let to = match self.to_block {
            Some(block) => block,
            None => 0,
        };

        for num in from..to {
            let block = self
                .rpc
                .eth()
                .block_with_txs(BlockId::Number(BlockNumber::Number(U64::from(num))))
                .await
                .unwrap();

            if let Some(block) = block {
                tokio::spawn({
                    let processor = self.processor.clone();
                    async move { processor.process_block(block).await }
                });
            }
        }

        Ok(())
    }
}
