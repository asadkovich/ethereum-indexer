mod fetch;
mod subscribe;

use crate::db::DB;
use crate::repository::Repository;
use crate::rpc::RPC;

#[derive(Debug)]
pub struct Service {
    repo: Repository,
    db: DB,
    rpc: RPC,
    verbose: bool,
}

impl Service {
    pub fn new(db: DB, rpc: RPC, verbose: bool) -> Self {
        let repo = Repository::new();

        Service {
            repo,
            db,
            rpc,
            verbose,
        }
    }
}
