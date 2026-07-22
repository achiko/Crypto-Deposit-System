//! Chain-neutral domain types and wallet abstractions for crypto deposits.
//!
//! The current API provides asset metadata, generated-key output, ledger
//! classification, and an object-safe wallet adapter boundary.
//!
//! # Example
//!
//! ```
//! use wallet_service::LedgerModel;
//!
//! let ledger_model = LedgerModel::Utxo;
//! assert!(matches!(ledger_model, LedgerModel::Utxo));
//! ```

mod domain;
mod errors;
mod wallet;

pub use domain::{Address, Asset, AssetId, ChainId, Keypair, LedgerModel};

pub use errors::WalletError;

pub use wallet::Wallet;
