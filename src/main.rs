use serde::Deserialize;
use std::error::Error;

#[derive(Deserialize, Debug)]
struct Peers {
    num: Vec<Peer>,
}

#[derive(Deserialize, Debug)]
struct Peer {
    symbol: String,
    count: u32,
}

fn get_node(url: &str) -> Result<Peers, Box<dyn Error>> {
    let response = ureq::get(url).call()?.into_string()?;
    let node : Vec<Peer> = serde_json::from_str(&response)?;
    let peers = Peers {
        num: serde_json::from_str(&response)?,
    };
    dbg!(node);
    Ok(peers)
}

fn main() {
    let url = "https://api.binance.com/api/v1/ticker/24hr";
    let num = get_node(url);
    dbg!(num);
}