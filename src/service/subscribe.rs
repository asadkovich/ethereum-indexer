use crate::Service;
use std::thread;
use web3::futures::StreamExt;
use web3::types::{BlockId, BlockNumber};

impl Service {
    /// Subscribes to new blocks.
    pub async fn subscribe(&self) {
        let mut stream = self
            .rpc
            .eth_subscribe()
            .subscribe_new_heads()
            .await
            .unwrap();

        while let Some(h) = stream.next().await {
            let header = h.unwrap();

            if header.number.is_none() {
                continue;
            }

            let block_id = BlockId::Number(BlockNumber::Number(header.number.unwrap()));
            let block = self.rpc.eth().block_with_txs(block_id).await.unwrap();

            if block.is_some() {
                thread::scope(|s| {
                    s.spawn(|| self.process_block(block.unwrap()));
                })
            }
        }
    }
}
