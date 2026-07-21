use std::{fmt, str::FromStr};

use num_bigint::BigUint;
use thiserror::Error;

macro_rules! non_blank_string_type {
    ($(#[$metadata:meta])* $name:ident, $blank_error:ident) => {
        $(#[$metadata])*
        #[derive(Debug, Clone, PartialEq, Eq, Hash)]
        pub struct $name(String);

        impl $name {
            /// Returns the value as a string slice.
            #[must_use]
            pub fn as_str(&self) -> &str {
                &self.0
            }
        }

        impl TryFrom<&str> for $name {
            type Error = DomainError;

            fn try_from(value: &str) -> Result<Self, Self::Error> {
                if value.trim().is_empty() {
                    Err(DomainError::$blank_error)
                } else {
                    Ok(Self(value.to_owned()))
                }
            }
        }

        impl TryFrom<String> for $name {
            type Error = DomainError;

            fn try_from(value: String) -> Result<Self, Self::Error> {
                if value.trim().is_empty() {
                    Err(DomainError::$blank_error)
                } else {
                    Ok(Self(value))
                }
            }
        }

        impl FromStr for $name {
            type Err = DomainError;

            fn from_str(value: &str) -> Result<Self, Self::Err> {
                Self::try_from(value)
            }
        }

        impl AsRef<str> for $name {
            fn as_ref(&self) -> &str {
                self.as_str()
            }
        }

        impl fmt::Display for $name {
            fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
                formatter.write_str(&self.0)
            }
        }
    };
}

non_blank_string_type!(
    /// Stable, nonblank identifier for a configured blockchain network.
    ChainId,
    BlankChainId
);

non_blank_string_type!(
    /// Stable, nonblank identifier for an asset, such as `BTC` or `USDC_ETHEREUM`.
    AssetId,
    BlankAssetId
);

non_blank_string_type!(
    /// Nonblank blockchain address text.
    ///
    /// This type prevents accidental mixing with arbitrary strings. A wallet
    /// adapter must still validate and normalize the chain-specific syntax.
    Address,
    BlankAddress
);

/// How a blockchain represents spendable state.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LedgerModel {
    /// Unspent transaction outputs, as used by Bitcoin.
    Utxo,
    /// Address-based accounts, as used by Ethereum and Solana.
    Account,
}

/// Whether an asset is native to its chain or implemented by a token program.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AssetKind {
    /// Native asset used by the chain itself, such as BTC or ETH.
    Native,
    /// Token whose balances are managed by a contract or token program.
    Token {
        /// Contract or mint address identifying the token.
        contract: Address,
        /// Native asset used to pay transaction fees.
        fee_asset_id: AssetId,
    },
}

/// Collection behavior selected when an asset is constructed.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CollectionModel {
    /// Collect one or more spendable UTXOs into the master wallet.
    Utxo,
    /// Transfer a native account-based asset to the master wallet.
    Account,
    /// Fund fees if required, then transfer a token to the master wallet.
    Token,
}

/// Static identity and ledger behavior of one blockchain network.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Chain {
    id: ChainId,
    ledger_model: LedgerModel,
}

impl Chain {
    /// Creates a configured blockchain network.
    #[must_use]
    pub const fn new(id: ChainId, ledger_model: LedgerModel) -> Self {
        Self { id, ledger_model }
    }

    /// Returns the stable network identifier.
    #[must_use]
    pub const fn id(&self) -> &ChainId {
        &self.id
    }

    /// Returns how this chain represents spendable state.
    #[must_use]
    pub const fn ledger_model(&self) -> LedgerModel {
        self.ledger_model
    }
}

/// Static metadata for one configured asset adapter.
///
/// Use [`Asset::native`] or [`Asset::token`] so the collection and fee models
/// are validated once, before an adapter can use the asset.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Asset {
    id: AssetId,
    chain_id: ChainId,
    symbol: String,
    decimals: u8,
    kind: AssetKind,
    collection_model: CollectionModel,
}

impl Asset {
    /// Creates a native asset for a chain.
    #[must_use]
    pub fn native(id: AssetId, chain: &Chain, symbol: impl Into<String>, decimals: u8) -> Self {
        let collection_model = match chain.ledger_model {
            LedgerModel::Utxo => CollectionModel::Utxo,
            LedgerModel::Account => CollectionModel::Account,
        };

        Self {
            id,
            chain_id: chain.id.clone(),
            symbol: symbol.into(),
            decimals,
            kind: AssetKind::Native,
            collection_model,
        }
    }

    /// Creates a token on an account-based chain.
    ///
    /// # Errors
    ///
    /// Returns [`DomainError::TokenOnUtxoChain`] if `chain` is UTXO-based,
    /// [`DomainError::FeeAssetChainMismatch`] if `fee_asset` belongs to another
    /// chain, or [`DomainError::TokenFeeAssetNotNative`] if the fee asset is
    /// itself a token.
    pub fn token(
        id: AssetId,
        chain: &Chain,
        symbol: impl Into<String>,
        decimals: u8,
        contract: Address,
        fee_asset: &Self,
    ) -> Result<Self, DomainError> {
        if chain.ledger_model == LedgerModel::Utxo {
            return Err(DomainError::TokenOnUtxoChain {
                asset_id: id,
                chain_id: chain.id.clone(),
            });
        }

        if fee_asset.chain_id != chain.id {
            return Err(DomainError::FeeAssetChainMismatch {
                token_asset_id: id,
                token_chain_id: chain.id.clone(),
                fee_asset_id: fee_asset.id.clone(),
                fee_asset_chain_id: fee_asset.chain_id.clone(),
            });
        }

        if !matches!(&fee_asset.kind, AssetKind::Native) {
            return Err(DomainError::TokenFeeAssetNotNative {
                token_asset_id: id,
                fee_asset_id: fee_asset.id.clone(),
            });
        }

        Ok(Self {
            id,
            chain_id: chain.id.clone(),
            symbol: symbol.into(),
            decimals,
            kind: AssetKind::Token {
                contract,
                fee_asset_id: fee_asset.id.clone(),
            },
            collection_model: CollectionModel::Token,
        })
    }

    /// Returns the stable application-level asset identifier.
    #[must_use]
    pub const fn id(&self) -> &AssetId {
        &self.id
    }

    /// Returns the blockchain on which this asset exists.
    #[must_use]
    pub const fn chain_id(&self) -> &ChainId {
        &self.chain_id
    }

    /// Returns the human-readable ticker symbol.
    #[must_use]
    pub fn symbol(&self) -> &str {
        &self.symbol
    }

    /// Returns the number of decimal places used for display.
    #[must_use]
    pub const fn decimals(&self) -> u8 {
        self.decimals
    }

    /// Returns whether this is a native asset or token.
    #[must_use]
    pub const fn kind(&self) -> &AssetKind {
        &self.kind
    }

    /// Returns the native asset used to pay transaction fees.
    #[must_use]
    pub const fn fee_asset_id(&self) -> &AssetId {
        match &self.kind {
            AssetKind::Native => &self.id,
            AssetKind::Token { fee_asset_id, .. } => fee_asset_id,
        }
    }

    /// Returns the already-validated collection strategy.
    #[must_use]
    pub const fn collection_model(&self) -> CollectionModel {
        self.collection_model
    }
}

/// An asset-specific blockchain address without duplicating all asset metadata.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct AssetAddress {
    asset_id: AssetId,
    address: Address,
}

impl AssetAddress {
    /// Associates a validated address with its asset.
    #[must_use]
    pub const fn new(asset_id: AssetId, address: Address) -> Self {
        Self { asset_id, address }
    }

    /// Returns the asset accepted at this address.
    #[must_use]
    pub const fn asset_id(&self) -> &AssetId {
        &self.asset_id
    }

    /// Returns the validated blockchain address.
    #[must_use]
    pub const fn address(&self) -> &Address {
        &self.address
    }
}

/// Integer monetary amount denominated in an asset's smallest unit.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Amount {
    asset_id: AssetId,
    atoms: BigUint,
}

impl Amount {
    /// Creates an amount from an arbitrary-size non-negative integer.
    #[must_use]
    pub const fn new(asset_id: AssetId, atoms: BigUint) -> Self {
        Self { asset_id, atoms }
    }

    /// Creates an amount from a `u64` base-unit value.
    #[must_use]
    pub fn from_u64(asset_id: AssetId, atoms: u64) -> Self {
        Self::new(asset_id, atoms.into())
    }

    /// Returns the asset in which this amount is denominated.
    #[must_use]
    pub const fn asset_id(&self) -> &AssetId {
        &self.asset_id
    }

    /// Returns the integer quantity in the asset's smallest unit.
    #[must_use]
    pub const fn atoms(&self) -> &BigUint {
        &self.atoms
    }
}

/// Invalid domain input or asset configuration.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum DomainError {
    /// A chain identifier contained no non-whitespace characters.
    #[error("chain identifier cannot be blank")]
    BlankChainId,
    /// An asset identifier contained no non-whitespace characters.
    #[error("asset identifier cannot be blank")]
    BlankAssetId,
    /// An address contained no non-whitespace characters.
    #[error("address cannot be blank")]
    BlankAddress,
    /// A contract token was configured on a UTXO ledger.
    #[error("token asset `{asset_id}` cannot use UTXO chain `{chain_id}`")]
    TokenOnUtxoChain {
        /// Invalid token asset.
        asset_id: AssetId,
        /// UTXO chain on which the token was configured.
        chain_id: ChainId,
    },
    /// A token and its native fee asset were configured on different chains.
    #[error(
        "token `{token_asset_id}` on `{token_chain_id}` cannot use fee asset `{fee_asset_id}` from `{fee_asset_chain_id}`"
    )]
    FeeAssetChainMismatch {
        /// Token being configured.
        token_asset_id: AssetId,
        /// Chain on which the token exists.
        token_chain_id: ChainId,
        /// Proposed native fee asset.
        fee_asset_id: AssetId,
        /// Chain on which the proposed fee asset exists.
        fee_asset_chain_id: ChainId,
    },
    /// A token was selected to fund another token's transaction fees.
    #[error("token `{token_asset_id}` requires native fee asset, but `{fee_asset_id}` is a token")]
    TokenFeeAssetNotNative {
        /// Token being configured.
        token_asset_id: AssetId,
        /// Invalid token fee asset.
        fee_asset_id: AssetId,
    },
}
