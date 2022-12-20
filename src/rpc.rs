use web3::{transports::WebSocket, Web3};

pub type RPC = Web3<WebSocket>;

pub async fn connect(rpc_url: &str) -> Result<RPC, crate::Error> {
    let transport = WebSocket::new(rpc_url).await?;
    let web3 = Web3::new(transport);

    Ok(web3)
}
