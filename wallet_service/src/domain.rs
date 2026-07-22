/// use num_bigint::BigUint;

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
    /// Contract or mint address for token assets.
    pub token_contract: Option<Address>,
}

/// Generated keypair for a configured asset adapter.
#[derive(Debug)]
pub struct Keypair {
    /// Generated chain address.
    pub address: Address,
    /// Encoded public key.
    pub public_key: Vec<u8>,
    /// Secret signing key.
    pub private_key: Option<Vec<u8>>,
}

/// Stable identifier for a configured blockchain network.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ChainId(String);

/// Stable identifier for an asset, such as `BTC` or `USDC_ETHEREUM`.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct AssetId(String);

/// Asset Blockchain address.  TODO: requires type revison
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Address(String);

/// How a blockchain represents spendable state.
#[derive(Debug, Clone)]
pub enum LedgerModel {
    /// Unspent transaction outputs, as used by Bitcoin.
    Utxo,
    /// Address-based accounts, as used by Ethereum and Solana.
    Account,
}

// Collection behavior selected when an asset is constructed.
// #[derive(Debug, Clone)]
// pub enum CollectionModel {
//     /// Collect one or more spendable UTXOs into the master wallet.
//     Utxo,
//     /// Transfer a native account-based asset to the master wallet.
//     Account,
//     /// Fund fees if required, then transfer a token to the master wallet.
//     Token,
// }

// Static identity and ledger behavior of one blockchain network.
// #[derive(Debug, Clone)]
// pub struct Chain {
//     /// Stable network identifier.
//     pub id: ChainId,
//     /// How the chain represents spendable state.
//     pub ledger_model: LedgerModel,
// }

// Static metadata for one configured asset adapter.
// #[derive(Debug, Clone)]
// pub struct Asset {
//     // TODO: Not sure if we need it here
//     /// Application level Id .
//     pub id: AssetId,
//     // TODO: need to be changed.
//     /// Blockchain on which the asset exists.
//     pub chain_id: ChainId,
//     /// ticker symbol (BTC , ETH ... )
//     pub symbol: String,
//     /// Number of decimal places used for display.
//     pub decimals: u8,
//     /// Whether the asset is native or a token.
//     pub kind: AssetKind,
// }

// An asset-specific blockchain address without duplicating all asset metadata.
// #[derive(Debug, Clone, PartialEq, Eq, Hash)]
// TODO: need to be changed. String is not good type for Blockhain address
// pub struct AssetAddress(String);

// Integer monetary amount denominated in an asset's smallest unit.
// #[derive(Debug, Clone, PartialEq, Eq)]
// pub struct Amount(Uint128);
