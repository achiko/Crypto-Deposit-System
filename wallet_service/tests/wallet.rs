//! Public wallet-trait behavior.

use wallet_service::{Asset, AssetId, Chain, ChainId, LedgerModel, Wallet};

struct TestWallet {
    asset: Asset,
}

impl Wallet for TestWallet {
    fn asset(&self) -> &Asset {
        &self.asset
    }
}

#[test]
fn wallet_trait_object_should_return_configured_asset() {
    let chain_id =
        ChainId::try_from("bitcoin-mainnet").expect("test chain identifier should be valid");
    let bitcoin = Chain::new(chain_id, LedgerModel::Utxo);
    let btc_id = AssetId::try_from("BTC").expect("test asset identifier should be valid");
    let wallet = TestWallet {
        asset: Asset::native(btc_id.clone(), &bitcoin, "BTC", 8),
    };
    let wallet: &dyn Wallet = &wallet;

    assert_eq!(wallet.asset().id(), &btc_id);
}
