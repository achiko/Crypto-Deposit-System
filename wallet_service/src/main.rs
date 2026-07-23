//! Working Bitcoin and Ethereum implementations of the wallet interface.

use wallet_service::{BitcoinWallet, EthereumWallet, Wallet, WalletError};

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), WalletError> {
    let wallets: Vec<Box<dyn Wallet>> = vec![
        Box::new(BitcoinWallet::mainnet()),
        Box::new(EthereumWallet::mainnet()),
    ];

    for wallet in wallets {
        let asset = wallet.asset();
        println!(
            "Initialized {} on {} ({:?})",
            asset.asset_id, asset.chain_id, asset.ledger_model
        );

        let keypair = wallet.generate_keypair().await?;

        println!("Generated keypair: ================================");
        println!("Generated address: {}", keypair.address);
        println!("Public key: {} bytes", keypair.public_key.len());
        println!("Private key: {}", keypair.private_key);
    }

    Ok(())
}
