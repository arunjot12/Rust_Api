use jsonrpsee::core::client::ClientT;
use jsonrpsee::ws_client::WsClientBuilder;
#[allow(unused)]

#[tokio::main]
async fn main() {
    // get_blockchain_name().await;
    get_blocknumber().await;
}

// async fn get_blockchain_name() -> String {
//     // Create a WebSocket client and connect to the node
//     let client = WsClientBuilder::default().build("ws://52.38.225.247:9944").await.expect("REASON");
//     // Call the `system_chain` method to get the blockchain name
//     let chain_name: String = client
//     .request("system_chain", jsonrpsee::core::params::ArrayParams::new())
//     .await
//     .expect("Failed to retrieve the chain name");

//     println!("Connected to chain: {}", chain_name);

//     chain_name
// }

pub async fn get_blocknumber() -> u64 {
    // Create a WebSocket client and connect to the node
    let client = WsClientBuilder::default().build("ws://52.38.225.247:9944").await.expect("REASON");
    // Call the `system_chain` method to get the blockchain name
    let block_number_hex: String = client
    .request("eth_blockNumber",jsonrpsee::core::params::ArrayParams::new())
    .await
    .expect("Failed to retrieve the chain name");

    let block_number = u64::from_str_radix(block_number_hex.trim_start_matches("0x"), 16)
    .expect("Failed to convert block number to decimal");
     
    println!("Current block on 5irechain number: {}", block_number);
    block_number
}
