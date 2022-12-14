use crate::entities;
use crate::service::Service;
use std::thread;
use web3::types::{Block, BlockId, BlockNumber, Transaction, U64};

impl Service {
    /// Reads blockchain from top to bottom and stores data in the database.
    pub async fn fetch(
        &self,
        from_block: Option<i64>,
        to_block: Option<i64>,
    ) -> Result<(), &'static str> {
        let from = match from_block {
            Some(block) => block,
            None => self.rpc.eth().block_number().await.unwrap().as_u64() as i64,
        };
        let to = match to_block {
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

            if block.is_some() {
                thread::scope(|s| {
                    s.spawn(|| self.process_block(block.unwrap()));
                })
            }
        }

        Ok(())
    }
}
