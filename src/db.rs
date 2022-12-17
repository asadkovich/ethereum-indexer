use log::error;
use sqlx::{self, postgres::PgPoolOptions, Executor, Pool, Postgres, Transaction};
use std::time::Duration;

pub type DB = Pool<Postgres>;
pub trait Querier<'c>: Executor<'c, Database = Postgres> {}

impl<'c> Querier<'c> for &Pool<Postgres> {}
impl<'c> Querier<'c> for &'c mut Transaction<'_, Postgres> {}

pub async fn connect(dsn: &str, pool_size: u32) -> Result<DB, crate::Error> {
    PgPoolOptions::new()
        .max_connections(pool_size)
        .max_lifetime(Duration::from_secs(30 * 60)) // 30 mins
        .connect(&dsn)
        .await
        .map_err(|err| {
            error!("Failed to connect to database: {}", err);
            err.into()
        })
}

pub async fn migrate(db: &DB) -> Result<(), crate::Error> {
    match sqlx::migrate!("./migrations").run(db).await {
        Ok(_) => Ok(()),
        Err(err) => {
            error!("db.migrate: migrating: {}", &err);
            Err(err)
        }
    }?;

    Ok(())
}
