mod binance;
pub mod firechain;
use crate::binance::*;
use crate::firechain::*;

/// 5irechain
#[tokio::main]
async fn main() {
    // 5irechain
    totat_issuance().await;

    // Binance 

    // let url = "https://api.binance.com/api/v1/ticker/24hr";
    // let num = get_node(url);
    // dbg!(num);
}