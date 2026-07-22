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

/// Blockchain address associated with an asset or generated keypair.
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
