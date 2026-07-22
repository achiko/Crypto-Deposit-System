//! Working Bitcoin implementation of the chain-neutral wallet interface.

use wallet_service::{BitcoinWallet, Wallet, WalletError};

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), WalletError> {
    let wallet: Box<dyn Wallet> = Box::new(BitcoinWallet::mainnet());
    let asset = wallet.asset();

    println!(
        "Initialized {} on {} ({:?})",
        asset.asset_id, asset.chain_id, asset.ledger_model
    );

    let keypair = wallet.generate_keypair().await?;

    println!("Generated address: {}", keypair.address);
    println!("Compressed public key: {} bytes", keypair.public_key.len());
    println!("Private key generated: {}", keypair.private_key.is_some());

    Ok(())
}
