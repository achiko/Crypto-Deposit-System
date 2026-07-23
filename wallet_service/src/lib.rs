//! Chain-neutral domain types and wallet abstractions for crypto deposits.
//!
//! The current API provides asset metadata, generated-key output, ledger
//! classification, and an object-safe wallet adapter boundary.
//!
//! # Example
//!
//! ```
//! use wallet_service::{BitcoinWallet, EthereumWallet, Wallet, WalletError};
//!
//! # #[tokio::main(flavor = "current_thread")]
//! # async fn main() -> Result<(), WalletError> {
//! let wallets: Vec<Box<dyn Wallet>> = vec![
//!     Box::new(BitcoinWallet::mainnet()),
//!     Box::new(EthereumWallet::mainnet()),
//! ];
//!
//! for wallet in wallets {
//!     let keypair = wallet.generate_keypair().await?;
//!     assert!(!keypair.private_key.is_empty());
//! }
//! # Ok(())
//! # }
//! ```

mod bitcoin;
mod domain;
mod errors;
mod ethereum;
mod wallet;

pub use bitcoin::BitcoinWallet;
pub use domain::{Address, Asset, AssetId, ChainId, Keypair, LedgerModel};
pub use ethereum::EthereumWallet;

pub use errors::WalletError;

pub use wallet::Wallet;
