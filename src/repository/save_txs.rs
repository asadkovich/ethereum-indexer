use crate::db;
use crate::entities::Transaction;
use crate::repository::Repository;
use sqlx::QueryBuilder;

impl Repository {
    /// save_txs saves transactions in the database.
    pub async fn save_txs<'c, C: db::Querier<'c>>(
        &self,
        db: C,
        txs: Vec<Transaction>,
    ) -> Result<(), crate::Error> {
        let mut builder = QueryBuilder::new(
            "
                INSERT INTO transactions (
                    hash, block_hash, block_number, from, to, value, gas, gas_price,
                    input, nonce, transaction_index, v, r, s
                ) ",
        );

        builder.push_values(txs, |mut b, tx| {
            b.push_bind(tx.hash);
            b.push_bind(tx.block_hash);
            b.push_bind(tx.block_number as i64);
            b.push_bind(tx.from);
            b.push_bind(tx.to);
            b.push_bind(tx.value);
            b.push_bind(tx.gas as i64);
            b.push_bind(tx.gas_price);
            b.push_bind(tx.input);
            b.push_bind(tx.nonce as i64);
            b.push_bind(tx.transaction_index as i64);
            b.push_bind(tx.v);
            b.push_bind(tx.r);
            b.push_bind(tx.s);
        });

        builder.push("ON CONFLICT (hash) DO NOTHING");

        let query = builder.build();
        query.execute(db).await?;

        Ok(())
    }
}
