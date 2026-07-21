//! Minimal executable entry point for local wallet-service experiments.
//!
use wallet_service::{Asset, AssetId, Chain, ChainId, DomainError, LedgerModel};

fn main() -> Result<(), DomainError> {
    let bitcoin_chain_id = ChainId::try_from("bitcoin-mainnet")?;
    let bitcoin = Chain::new(bitcoin_chain_id, LedgerModel::Utxo);
    let btc_id = AssetId::try_from("BTC")?;
    let btc = Asset::native(btc_id, &bitcoin, "BTC", 8);

    println!("Configured asset: {btc:?}");

    let ethereum_chain_id = ChainId::try_from("ethereum-mainnet")?;
    let ethereum = Chain::new(ethereum_chain_id, LedgerModel::Account);
    let eth_id = AssetId::try_from("ETH")?;
    let eth = Asset::native(eth_id, &ethereum, "ETH", 18);

    println!("Configured asset: {eth:?}");

    Ok(())
}
