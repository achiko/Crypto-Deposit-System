//! Minimal executable entry point for local wallet-service experiments.
//!
// use wallet_service::{Asset, AssetId,  ChainId, DomainError, LedgerModel};

fn main() -> Result<(), ()> {
    let chain_id = String::from("bitcoin-mainnet");
    println!("Chain ID: {chain_id}");

    // let bitcoin = Chain::new(bitcoin_chain_id, LedgerModel::Utxo);
    // let btc_id = AssetId::try_from("BTC")?;
    // let btc = Asset::native(btc_id, &bitcoin, "BTC", 8);
    // s

    Ok(())
}
