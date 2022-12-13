use std::thread;
use web3::futures::future::ok;
use web3::types::{Block, BlockId, BlockNumber, Transaction, U64};
use crate::service::Service;

impl Service {
    /// fetch reads blockchain from top to bottom and stores data in the database.
    pub fn fetch(&self, from_block: Option<u128>, to_block: Option<u128>) -> Result<(), Err> {
        let from = match from_block {
            Some(block) => block,
            None => self.rpc.eth().block_number().await? as u128,
        };
        let to = match to_block {
            Some(block) => block,
            None => 0,
        };

        let mut handles = Vec::new();

        for num in from..to {
            let block = self.rpc
                .eth()
                .block_with_txs(BlockId::Number(BlockNumber::Number(num as U64)))
                .await?
                .unwrap();

            let handle = thread::spawn(|| {
                self.process_block(block)
            });

            handles.push(handle);
        }

        for handle in handles {
            handle.join().unwrap();
        }

        Ok(())
    }

    pub fn process_block(&self, block: Block<Transaction>) {

    }
}