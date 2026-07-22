use std::fmt;

use zeroize::Zeroizing;

/// Static metadata for one configured asset adapter.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Asset {
    /// Application-level asset identifier.
    pub asset_id: AssetId,
    /// Blockchain on which the asset exists.
    pub chain_id: ChainId,
    /// Human-readable ticker symbol.
    pub symbol: String,
    /// Number of decimal places used for display.
    pub decimals: u8,
    /// How the asset's blockchain represents spendable state.
    pub ledger_model: LedgerModel,
    /// Contract or mint address for token assets.
    pub token_contract: Option<Address>,
}

/// Generated keypair for a configured asset adapter.
pub struct Keypair {
    /// Generated chain address.
    pub address: Address,
    /// Encoded public key.
    pub public_key: Vec<u8>,
    /// Secret signing key bytes, zeroized when dropped.
    pub private_key: Option<Zeroizing<Vec<u8>>>,
}

/// Stable identifier for a configured blockchain network.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ChainId(String);

/// Stable identifier for an asset, such as `BTC` or `USDC_ETHEREUM`.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct AssetId(String);

/// Blockchain address associated with an asset or generated keypair.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Address(String);

// TODO: remove this macro in the prod version, aftyer removeing String types to the chain_id, asset_id, address types
macro_rules! impl_text_value {
    ($type:ident) => {
        impl $type {
            /// Returns the wrapped text value.
            #[must_use]
            pub fn as_str(&self) -> &str {
                &self.0
            }
        }

        impl From<&str> for $type {
            fn from(value: &str) -> Self {
                Self(value.to_owned())
            }
        }

        impl From<String> for $type {
            fn from(value: String) -> Self {
                Self(value)
            }
        }

        impl fmt::Display for $type {
            fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
                formatter.write_str(&self.0)
            }
        }
    };
}

impl_text_value!(ChainId);
impl_text_value!(AssetId);
impl_text_value!(Address);

/// How a blockchain represents spendable state.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LedgerModel {
    /// Unspent transaction outputs, as used by Bitcoin.
    Utxo,
    /// Address-based accounts, as used by Ethereum and Solana.
    Account,
}
