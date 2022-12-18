use sqlx;

#[derive(sqlx::FromRow, Debug, Clone)]
pub struct Block {
    pub hash: String,
    pub parent_hash: String,
    pub number: i64,
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub nonce: String,
    pub difficulty: String,
    pub gas_limit: i64,
    pub gas_used: i64,
    pub miner: String,
    pub extra_data: String,
    pub logs_bloom: String,
    pub transactions_root: String,
    pub state_root: String,
    pub receipts_root: String,
    pub sha3_uncles: Vec<String>,
    pub size: i64,
    pub total_difficulty: String,
}

#[derive(sqlx::FromRow, Debug, Clone)]
pub struct Transaction {
    pub hash: String,
    pub block_hash: String,
    pub block_number: u64,
    pub from: String,
    pub to: String,
    pub value: String,
    pub gas: u128,
    pub gas_price: String,
    pub input: String,
    pub nonce: u128,
    pub transaction_index: u128,
    pub v: String,
    pub r: String,
    pub s: String,
}
