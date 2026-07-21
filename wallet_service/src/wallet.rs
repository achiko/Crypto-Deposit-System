use crate::Asset;

/// Chain-specific operations for one configured asset.
///
/// The trait remains object-safe so a future runtime registry can hold
/// heterogeneous wallet adapters behind `Arc<dyn Wallet>`.
pub trait Wallet: Send + Sync {
    /// Returns the asset handled by this adapter.
    fn asset(&self) -> &Asset;
}
