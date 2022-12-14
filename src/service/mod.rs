mod subscribe;
mod fetch;

use crate::db::DB;
use crate::repository::Repository;
use crate::rpc::RPC;

#[derive(Debug)]
pub struct Service {
    pub repo: Repository,
    pub db: DB,
    pub rpc: RPC
}