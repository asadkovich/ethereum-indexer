use crate::db;
use crate::repository::Repository;

impl Repository {
    pub async fn get_saved_blocks_count<'c, C: db::Querier<'c>>(
        &self,
        db: C,
    ) -> Result<i64, crate::Error> {
        let count = sqlx::query_scalar("SELECT COUNT(*) FROM blocks")
            .fetch_one(db)
            .await?;

        Ok(count)
    }

    pub async fn get_saved_transactions_count<'c, C: db::Querier<'c>>(
        &self,
        db: C,
    ) -> Result<i64, crate::Error> {
        let count = sqlx::query_scalar("SELECT COUNT(*) FROM transactions")
            .fetch_one(db)
            .await?;

        Ok(count)
    }
}
