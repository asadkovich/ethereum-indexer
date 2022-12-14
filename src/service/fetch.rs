use crate::entities;
use crate::service::Service;
use std::thread;
use web3::types::{Block, BlockId, BlockNumber, Transaction, U64};

impl Service {
    /// fetch reads blockchain from top to bottom and stores data in the database.
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

    async fn process_block(&self, block: Block<Transaction>) {
        let block_hash = block.hash.unwrap();
        let block_number = block.number.unwrap().as_u64();
        let parent_hash = block.parent_hash.to_string();
        let timestamp = block.timestamp.as_u64();

        let mut transactions = Vec::new();

        for tx in block.transactions {
            let transaction = entities::Transaction {
                hash: tx.hash.to_string(),
                block_hash: tx.block_hash.unwrap().to_string(),
                block_number: tx.block_number.unwrap().as_u64(),
                from: tx.from.unwrap().to_string(),
                to: tx.to.unwrap().to_string(),
                value: tx.value.to_string(),
                gas: tx.gas.as_u64() as u128,
                gas_price: tx.gas_price.to_string(),
                input: serde_json::to_string(&tx.input).unwrap(),
                nonce: tx.nonce.as_u64() as u128,
                transaction_index: tx.transaction_index.unwrap().as_u64() as u128,
                v: tx.v.unwrap().to_string(),
                r: tx.r.unwrap().to_string(),
                s: tx.s.unwrap().to_string(),
            };

            transactions.push(transaction);
        }

        let tx_hashes: Vec<String> = transactions.iter().map(|tx| tx.hash.clone()).collect();

        let block = entities::Block {
            hash: block_hash.to_string(),
            number: block_number,
            parent_hash,
            timestamp,
            transactions: tx_hashes,
        };

        let mut tx = self.db.begin().await.unwrap();

        self.repo.save_txs(&mut tx, transactions).await.unwrap();

        tx.commit().await.unwrap();
    }
}
