use crate::db::DB;
use crate::repository::Repository;
use crate::{entities, Service};
use std::sync::Arc;
use web3::types::{Block, Transaction};

#[derive(Debug)]
pub struct BlockProcessor {
    db: Arc<DB>,
    repo: Repository,
}

impl BlockProcessor {
    pub fn new(db: Arc<DB>) -> Self {
        let repo = Repository::new();

        BlockProcessor { db, repo }
    }

    pub async fn process_block(&self, block: Block<Transaction>) {
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

        let block = entities::Block {
            hash: block_hash.to_string(),
            parent_hash,
            number: block_number as i64,
            timestamp: timestamp as i64,
            nonce: block.nonce.unwrap().to_string(),
            difficulty: block.difficulty.to_string(),
            gas_limit: block.gas_limit.as_u64() as i64,
            gas_used: block.gas_used.as_u64() as i64,
            miner: block.author.to_string(),
            extra_data: serde_json::to_string(&block.extra_data).unwrap(),
            logs_bloom: serde_json::to_string(&block.logs_bloom).unwrap(),
            transactions_root: block.transactions_root.to_string(),
            state_root: block.state_root.to_string(),
            receipts_root: block.receipts_root.to_string(),
            sha3_uncles: block.uncles.iter().map(|u| u.to_string()).collect(),
            size: block.size.unwrap().as_u64() as i64,
            total_difficulty: block.total_difficulty.unwrap().to_string(),
            uncles: serde_json::to_string(&block.uncles).unwrap(),
        };

        let mut tx = self.db.begin().await.unwrap();

        self.repo.save_txs(&mut tx, transactions).await.unwrap();
        self.repo.save_block(&mut tx, block).await.unwrap();

        tx.commit().await.unwrap();
    }
}
