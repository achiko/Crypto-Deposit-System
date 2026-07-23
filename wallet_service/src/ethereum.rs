use async_trait::async_trait;
use secp256k1::{rand, PublicKey, Secp256k1, SecretKey};
use sha3::{Digest, Keccak256};

use crate::{Address, Asset, AssetId, ChainId, Keypair, LedgerModel, Wallet, WalletError};

/// Minimal Ethereum mainnet adapter demonstrating the [`Wallet`] contract.
pub struct EthereumWallet {
    asset: Asset,
}

impl EthereumWallet {
    /// Creates an Ethereum mainnet wallet with ETH metadata.
    #[must_use]
    pub fn mainnet() -> Self {
        Self {
            asset: Asset {
                asset_id: AssetId::from("ETH"),
                chain_id: ChainId::from("ethereum-mainnet"),
                symbol: "ETH".to_owned(),
                decimals: 18,
                ledger_model: LedgerModel::Account,
                token_contract: None,
            },
        }
    }
}

#[async_trait]
impl Wallet for EthereumWallet {
    fn asset(&self) -> &Asset {
        &self.asset
    }

    async fn generate_keypair(&self) -> Result<Keypair, WalletError> {
        let secp = Secp256k1::new();
        let (private_key, public_key) = secp.generate_keypair(&mut rand::thread_rng());

        Ok(keypair_from_parts(private_key, public_key))
    }
}

fn keypair_from_parts(private_key: SecretKey, public_key: PublicKey) -> Keypair {
    let public_key = public_key.serialize_uncompressed();
    let public_key = &public_key[1..];
    let hash = Keccak256::digest(public_key);
    let address = format!("0x{}", hex::encode(&hash[12..]));

    Keypair {
        address: Address::from(address),
        public_key: public_key.to_vec(),
        private_key: format!("0x{}", hex::encode(private_key.secret_bytes())),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mainnet_wallet_initializes_ethereum_metadata() {
        let wallet = EthereumWallet::mainnet();
        let asset = wallet.asset();

        assert_eq!(asset.asset_id.as_str(), "ETH");
        assert_eq!(asset.chain_id.as_str(), "ethereum-mainnet");
        assert_eq!(asset.symbol, "ETH");
        assert_eq!(asset.decimals, 18);
        assert_eq!(asset.ledger_model, LedgerModel::Account);
        assert_eq!(asset.token_contract, None);
    }

    #[tokio::test(flavor = "current_thread")]
    async fn trait_object_generates_matching_mainnet_keypair() {
        let wallet: Box<dyn Wallet> = Box::new(EthereumWallet::mainnet());
        let keypair = wallet
            .generate_keypair()
            .await
            .expect("Ethereum key generation should succeed");
        let private_hex = keypair
            .private_key
            .strip_prefix("0x")
            .expect("generated Ethereum private key should have a 0x prefix");
        let private_bytes =
            hex::decode(private_hex).expect("generated Ethereum private key should be valid hex");
        let private_key = SecretKey::from_slice(&private_bytes)
            .expect("generated Ethereum private key should be a valid secp256k1 scalar");
        let secp = Secp256k1::new();
        let public_key = PublicKey::from_secret_key(&secp, &private_key);
        let expected = keypair_from_parts(private_key, public_key);

        assert_eq!(keypair.public_key, expected.public_key);
        assert_eq!(keypair.address, expected.address);
        assert_eq!(keypair.public_key.len(), 64);
        assert_eq!(keypair.address.as_str().len(), 42);
        assert!(keypair.address.as_str().starts_with("0x"));
        assert_eq!(keypair.private_key.len(), 66);
    }

    #[test]
    fn derives_known_ethereum_address_vector() {
        let private_bytes =
            hex::decode("0000000000000000000000000000000000000000000000000000000000000001")
                .expect("known private key vector should be valid hex");
        let private_key = SecretKey::from_slice(&private_bytes)
            .expect("known private key vector should be valid secp256k1 data");
        let secp = Secp256k1::new();
        let public_key = PublicKey::from_secret_key(&secp, &private_key);
        let keypair = keypair_from_parts(private_key, public_key);

        assert_eq!(
            hex::encode(&keypair.public_key),
            concat!(
                "79be667ef9dcbbac55a06295ce870b07029bfcdb2dce28d959f2815b16f81798",
                "483ada7726a3c4655da4fbfc0e1108a8fd17b448a68554199c47d08ffb10d4b8"
            )
        );
        assert_eq!(
            keypair.address.as_str(),
            "0x7e5f4552091a69125d5dfcb7b8c2659029395bdf"
        );
        assert_eq!(
            keypair.private_key,
            "0x0000000000000000000000000000000000000000000000000000000000000001"
        );
    }
}
