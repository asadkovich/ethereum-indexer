mod save_block;
mod save_txs;

/// Repository is responsible for saving data in the database.
#[derive(Debug, Clone)]
pub struct Repository {}

impl Repository {
    pub fn new() -> Self {
        Repository {}
    }
}
