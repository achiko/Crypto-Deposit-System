//! Chain-neutral domain types and wallet abstractions for crypto deposits.
//!
//! Chain ledger behavior and asset kind are separate concepts. Each [`Asset`]
//! is constructed against a [`Chain`], so its collection strategy and native
//! fee asset are valid before a wallet adapter can use it.
//!
//! # Example
//!
//! ```
//! use wallet_service::{Asset, AssetId, Chain, ChainId, CollectionModel, DomainError, LedgerModel};
//!
//! # fn main() -> Result<(), DomainError> {
//! let chain_id = ChainId::try_from("bitcoin-mainnet")?;
//! let bitcoin = Chain::new(chain_id, LedgerModel::Utxo);
//! let btc_id = AssetId::try_from("BTC")?;
//! let btc = Asset::native(btc_id, &bitcoin, "BTC", 8);
//!
//! assert_eq!(btc.collection_model(), CollectionModel::Utxo);
//! # Ok(())
//! # }
//! ```

mod domain;
mod wallet;

pub use domain::{
    Address, Amount, Asset, AssetAddress, AssetId, AssetKind, Chain, ChainId, CollectionModel,
    DomainError, LedgerModel,
};
pub use wallet::Wallet;
