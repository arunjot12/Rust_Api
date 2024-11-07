use jsonrpsee::core::client::ClientT;
use jsonrpsee::ws_client::WsClientBuilder;
use substrate_api_client::{
    Api,
    GetStorage,
    rpc::JsonrpseeClient,
    ac_primitives::DefaultRuntimeConfig,
};

use diesel::prelude::*;
use dotenvy::dotenv;
use std::env;

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

pub async fn get_blockchain_name() -> String {
    let client = WsClientBuilder::default().build("ws://52.38.225.247:9944").await.expect("REASON");
    let chain_name: String = client
        .request("system_chain", jsonrpsee::core::params::ArrayParams::new()).await
        .expect("Failed to retrieve the chain name");
    chain_name
}

pub async fn get_blocknumber() -> u64 {
    let client = WsClientBuilder::default().build("ws://52.38.225.247:9944").await.expect("REASON");
    let block_number_hex: String = client
        .request("eth_blockNumber", jsonrpsee::core::params::ArrayParams::new()).await
        .expect("Failed to retrieve the chain name");
    let block_number = u64
        ::from_str_radix(block_number_hex.trim_start_matches("0x"), 16)
        .expect("Failed to convert block number to decimal");
    block_number
}

pub async fn total_issuance() -> Option<u128> {
    let client = JsonrpseeClient::new("ws://52.38.225.247:9944").await.expect("REASON");
    let api = Api::<DefaultRuntimeConfig, _>::new(client).await.unwrap();
    let balance: Option<u128> = api
        .get_storage::<u128>("Balances", "TotalIssuance", None).await
        .unwrap();
    balance
}
