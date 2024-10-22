use jsonrpsee::core::client::ClientT;
use jsonrpsee::ws_client::WsClientBuilder;
use substrate_api_client::{ Api, GetChainInfo };
use substrate_api_client::rpc::JsonrpseeClient;
use substrate_api_client::GetStorage;
use substrate_api_client::ac_primitives::DefaultRuntimeConfig;
#[allow(unused)]

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

// pub async fn get_blocknumber() -> u64 {
//     // Create a WebSocket client and connect to the node
//     let client = WsClientBuilder::default().build("ws://52.38.225.247:9944").await.expect("REASON");
//     // Call the `system_chain` method to get the blockchain name
//     let block_number_hex: String = client
//     .request("eth_blockNumber",jsonrpsee::core::params::ArrayParams::new())
//     .await
//     .expect("Failed to retrieve the chain name");

//     let block_number = u64::from_str_radix(block_number_hex.trim_start_matches("0x"), 16)
//     .expect("Failed to convert block number to decimal");

//     println!("Current block on 5irechain number: {}", block_number);
//     block_number
// }
pub async fn totat_issuance() {
    // Create a WebSocket client and connect to the node
    let client = JsonrpseeClient::new("ws://52.38.225.247:9944").await.expect("REASON");
    let api = Api::<DefaultRuntimeConfig, _>::new(client).await.unwrap();
    let storage = api.get_genesis_block();
    let balance: Option<u128> = api.get_storage::<u128>("Balances", "TotalIssuance", None).await.unwrap();
    println!("Total Issuance is {:?}",balance);
}
