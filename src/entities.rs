use sqlx;

#[derive(sqlx::FromRow, Debug, Clone)]
pub struct Block {
    pub hash: String,
    pub number: u64,
    pub parent_hash: String,
    pub timestamp: u64,
    pub transactions: Vec<String>,
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