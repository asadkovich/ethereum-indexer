use crate::db;
use crate::entities::Block;
use crate::repository::Repository;

impl Repository {
    pub async fn save_block<'c, C: db::Querier<'c>>(
        &self,
        db: C,
        block: Block,
    ) -> Result<(), crate::Error> {
        sqlx::query(
            "
                INSERT INTO blocks (
                    hash, parent_hash, number, timestamp, nonce, difficulty, gas_limit,
                    gas_used, miner, extra_data, logs_bloom, transactions_root,
                    state_root, receipts_root, sha3_uncles, size, total_difficulty,
                    uncles
                )
                VALUES (
                    $1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15,
                    $16, $17, $18
                )
                ON CONFLICT (hash) DO NOTHING
            ",
        )
        .bind(block.hash)
        .bind(block.parent_hash)
        .bind(block.number)
        .bind(block.timestamp)
        .bind(block.nonce)
        .bind(block.difficulty)
        .bind(block.gas_limit)
        .bind(block.gas_used)
        .bind(block.miner)
        .bind(block.extra_data)
        .bind(block.logs_bloom)
        .bind(block.transactions_root)
        .bind(block.state_root)
        .bind(block.receipts_root)
        .bind(block.sha3_uncles)
        .bind(block.size)
        .bind(block.total_difficulty)
        .bind(block.uncles)
        .execute(db)
        .await?;

        Ok(())
    }
}
