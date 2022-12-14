use crate::{entities, Service};
use web3::types::{Block, Transaction};

impl Service {
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
