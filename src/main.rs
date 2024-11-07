#![feature(trivial_bounds)]
mod binance;
pub mod firechain;
use chrono::prelude::*;
use crate::firechain::*;
use tokio::time;
use std::time::Duration;
pub mod schema;
pub mod models;


/// 5irechain
#[tokio::main]
async fn main() {
    let blockchain_name: String = get_blockchain_name().await;
    let total_issuance: u128 = total_issuance().await.unwrap();
    println!("Blockchain chain: {}", blockchain_name);
    println!("Total Supply of {} blockchain is {:?}",blockchain_name,total_issuance);
    let mut interval = time::interval(Duration::from_secs(3));
    let local: DateTime<Local> = Local::now();

    loop 
    {
        interval.tick().await;
        let block_number = get_blocknumber().await;
        println!("Current block on {} blockchains is number: {} at time {:?}",blockchain_name, block_number,local);
    }
}