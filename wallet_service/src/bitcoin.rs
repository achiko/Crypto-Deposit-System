use ::bitcoin::secp256k1::Secp256k1;
use ::bitcoin::{Address as BitcoinAddress, CompressedPublicKey, Network, PrivateKey};
use async_trait::async_trait;

use crate::{Address, Asset, AssetId, ChainId, Keypair, LedgerModel, Wallet, WalletError};

/// Minimal Bitcoin mainnet adapter demonstrating the [`Wallet`] contract.
pub struct BitcoinWallet {
    asset: Asset,
    network: Network,
}

impl BitcoinWallet {
    /// Creates a Bitcoin mainnet wallet with BTC metadata.
    #[must_use]
    pub fn mainnet() -> Self {
        Self {
            asset: Asset {
                asset_id: AssetId::from("BTC"),
                chain_id: ChainId::from("bitcoin-mainnet"),
                symbol: "BTC".to_owned(),
                decimals: 8,
                ledger_model: LedgerModel::Utxo,
                token_contract: None,
            },
            network: Network::Bitcoin,
        }
    }
}

#[async_trait]
impl Wallet for BitcoinWallet {
    fn asset(&self) -> &Asset {
        &self.asset
    }

    async fn generate_keypair(&self) -> Result<Keypair, WalletError> {
        let secp = Secp256k1::new();
        let private_key = PrivateKey::generate(self.network);
        let public_key =
            CompressedPublicKey::from_private_key(&secp, &private_key).map_err(|source| {
                WalletError::KeyGeneration {
                    reason: source.to_string(),
                }
            })?;
        let address = BitcoinAddress::p2wpkh(&public_key, self.network);

        Ok(Keypair {
            address: Address::from(address.to_string()),
            public_key: public_key.to_bytes().to_vec(),
            private_key: private_key.to_wif(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mainnet_wallet_initializes_bitcoin_metadata() {
        let wallet = BitcoinWallet::mainnet();
        let asset = wallet.asset();

        assert_eq!(asset.asset_id.as_str(), "BTC");
        assert_eq!(asset.chain_id.as_str(), "bitcoin-mainnet");
        assert_eq!(asset.symbol, "BTC");
        assert_eq!(asset.decimals, 8);
        assert_eq!(asset.ledger_model, LedgerModel::Utxo);
        assert_eq!(asset.token_contract, None);
    }

    #[tokio::test(flavor = "current_thread")]
    async fn trait_object_generates_matching_mainnet_keypair() {
        let wallet: Box<dyn Wallet> = Box::new(BitcoinWallet::mainnet());
        let keypair = wallet
            .generate_keypair()
            .await
            .expect("Bitcoin key generation should succeed");
        let private_key = PrivateKey::from_wif(&keypair.private_key)
            .expect("generated Bitcoin private key should be valid WIF");
        assert!(private_key.compressed);
        let secp = Secp256k1::new();
        let public_key = CompressedPublicKey::from_private_key(&secp, &private_key)
            .expect("generated Bitcoin public key should be compressed");
        let expected_address = BitcoinAddress::p2wpkh(&public_key, Network::Bitcoin);

        assert_eq!(keypair.public_key, public_key.to_bytes());
        assert_eq!(keypair.address.as_str(), expected_address.to_string());
        assert!(keypair.address.as_str().starts_with("bc1q"));
        assert!(
            keypair.private_key.starts_with('K') || keypair.private_key.starts_with('L'),
            "compressed Bitcoin mainnet WIF should start with K or L"
        );
    }
}
