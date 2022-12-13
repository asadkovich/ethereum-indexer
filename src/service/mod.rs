mod subscribe;
mod fetch;

use crate::db::DB;
use crate::rpc::RPC;

#[derive(Debug)]
pub struct Service {
    pub db: DB,
    pub rpc: RPC
}