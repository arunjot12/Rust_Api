mod binance;
pub mod firechain;
use crate::firechain::*;
use tokio::time;
use std::time::Duration;

/// 5irechain
#[tokio::main]
async fn main() {
    // 5irechain
    let mut interval = time::interval(Duration::from_secs(3));

    // Continuously process each tick
    loop {
        // Wait for the next interval tick
        interval.tick().await;

        // Execute your asynchronous task (get_blocknumber in this case)
        get_blocknumber().await;
    }

    // Binance 
    // let url = "https://api.binance.com/api/v1/ticker/24hr";
    // let num = get_node(url);
    // dbg!(num);
}