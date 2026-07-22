// lib.rs

//! Chain-neutral domain types and wallet abstractions for crypto deposits.
//!
//! Chain ledger behavior and asset kind are separate concepts represented by
//! plain data structures.
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
