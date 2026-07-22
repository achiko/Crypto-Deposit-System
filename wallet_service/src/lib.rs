//! Chain-neutral domain types and wallet abstractions for crypto deposits.
//!
//! The current API provides asset metadata, generated-key output, ledger
//! classification, and an object-safe wallet adapter boundary.
//!
//! # Example
//!
//! ```
//! use wallet_service::{BitcoinWallet, Wallet, WalletError};
//!
//! # #[tokio::main(flavor = "current_thread")]
//! # async fn main() -> Result<(), WalletError> {
//! let wallet: Box<dyn Wallet> = Box::new(BitcoinWallet::mainnet());
//! let keypair = wallet.generate_keypair().await?;
//!
//! assert!(keypair.address.as_str().starts_with("bc1q"));
//! # Ok(())
//! # }
//! ```

mod bitcoin;
mod domain;
mod errors;
mod wallet;

pub use bitcoin::BitcoinWallet;
pub use domain::{Address, Asset, AssetId, ChainId, Keypair, LedgerModel};

pub use errors::WalletError;

pub use wallet::Wallet;
