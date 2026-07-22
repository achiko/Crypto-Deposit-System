//! Minimal executable entry point for local wallet-service experiments.

fn main() -> Result<(), ()> {
    let chain_id = String::from("bitcoin-mainnet");
    println!("Chain ID: {chain_id}");

    Ok(())
}
