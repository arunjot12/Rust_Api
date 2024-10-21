use jsonrpsee::core::client::ClientT;
use jsonrpsee::ws_client::WsClientBuilder;
#[allow(unused)]

#[tokio::main]
async fn main() {
    get_blockchain_name().await;
}

async fn get_blockchain_name() -> String {
    // Create a WebSocket client and connect to the node
    let client = WsClientBuilder::default().build("ws://52.38.225.247:9944").await.expect("REASON");
    // Call the `system_chain` method to get the blockchain name
    let chain_name: String = client
    .request("system_chain", jsonrpsee::core::params::ArrayParams::new())
    .await
    .expect("Failed to retrieve the chain name");

    println!("Connected to chain: {}", chain_name);

    chain_name
}
