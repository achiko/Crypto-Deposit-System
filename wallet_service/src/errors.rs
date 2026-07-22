use thiserror::Error;

use crate::AssetId;

/// Errors returned by asset wallet adapters.
#[derive(Debug, Error)]
#[non_exhaustive]
pub enum WalletError {
    #[error("unsupported asset: {0:?}")]
    /// The requested asset is not supported by this wallet adapter.
    UnsupportedAsset(AssetId),
}
