#[allow(dead_code)]
 // Binance 
    // let url = "https://api.binance.com/api/v1/ticker/24hr";
    // let num = get_node(url);
    // dbg!(num);
use serde::Deserialize;
use std::error::Error;
#[allow(dead_code)]

#[derive(Deserialize, Debug)]
pub struct Peers {
    num: Vec<Peer>,
}
#[allow(dead_code)]
#[derive(Deserialize, Debug)]
pub struct Peer {
    symbol: String,
    count: u32,
}

#[allow(dead_code)]
pub fn get_node(url: &str) -> Result<Peers, Box<dyn Error>> {
    let response = ureq::get(url).call()?.into_string()?;
    let node : Vec<Peer> = serde_json::from_str(&response)?;
    let peers = Peers {
        num: serde_json::from_str(&response)?,
    };
    dbg!(node);
    Ok(peers)
}
