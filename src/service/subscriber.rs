use crate::rpc::RPC;
use crate::service::processor::BlockProcessor;
use std::sync::Arc;
use std::thread;
use web3::futures::StreamExt;
use web3::types::{BlockId, BlockNumber};

pub struct Subscriber {
    processor: Arc<BlockProcessor>,
    rpc: Arc<RPC>,
}

impl Subscriber {
    pub fn new(processor: Arc<BlockProcessor>, rpc: Arc<RPC>) -> Self {
        Subscriber { processor, rpc }
    }

    /// Subscribes to new blocks.
    pub async fn run(&self) -> Result<(), crate::Error> {
        let mut stream = self.rpc.eth_subscribe().subscribe_new_heads().await?;

        while let Some(h) = stream.next().await {
            let header = h.unwrap();

            if header.number.is_none() {
                continue;
            }

            let block_id = BlockId::Number(BlockNumber::Number(header.number.unwrap()));
            let block = self.rpc.eth().block_with_txs(block_id).await?;

            if block.is_some() {
                thread::scope(|s| {
                    s.spawn(|| self.processor.process_block(block.unwrap()));
                });
            }
        }

        Ok(())
    }
}
