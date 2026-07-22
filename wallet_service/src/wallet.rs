// wallet.rs

use crate::{errors::WalletError, Asset, Keypair};

/// Chain-specific operations for one configured asset.
///
/// The trait remains object-safe so a future runtime registry can hold
/// heterogeneous wallet adapters behind `Arc<dyn Wallet>`.
///

pub trait Wallet: Send + Sync {
    /// Returns metadata for the asset handled by this adapter.
    fn asset(&self) -> &Asset;

    /// Generates a chain-valid address and signing key.
    ///
    /// # Errors
    ///
    /// Returns [`WalletError`] if secure key generation or address derivation fails.
    async fn generate_keypair(&self) -> Result<Keypair, WalletError>;
}
