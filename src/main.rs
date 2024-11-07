mod binance;
pub mod firechain;
use crate::firechain::*;
use tokio::time;
use std::time::Duration;

/// 5irechain
#[tokio::main]
async fn main() {
    // Binance 
    // let url = "https://api.binance.com/api/v1/ticker/24hr";
    // let num = get_node(url);
    // dbg!(num);

    // 5irechain
    let blockchain_name: String = get_blockchain_name().await;
    let total_issuance: u128 = total_issuance().await.unwrap();
    println!("Blockchain chain: {}", blockchain_name);
    println!("Total issuance of {} blockchain is {:?}",blockchain_name,total_issuance);
    let mut interval = time::interval(Duration::from_secs(3));

    // Continuously process each tick
    loop {
        // Wait for the next interval tick
        interval.tick().await;
        // Execute your asynchronous task (get_blocknumber in this case)
        get_blocknumber().await;
    }
}