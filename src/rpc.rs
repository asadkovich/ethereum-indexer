use web3::{transports::WebSocket, Web3};

pub type RPC = Web3<WebSocket>;

pub async fn connect(rpc_url: &str) -> Result<RPC, &'static str> {
    let transport = WebSocket::new(rpc_url).await.unwrap();
    let web3 = Web3::new(transport);

    Ok(web3)
}
