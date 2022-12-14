use web3::{Web3, transports::Http};

pub type RPC = Web3<Http>;

pub async fn connect(rpc_url: &str) -> Result<RPC, &'static str> {
    let transport = Http::new(rpc_url).unwrap();
    let web3 = Web3::new(transport);

    return Ok(web3);
}