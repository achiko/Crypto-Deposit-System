//! Public domain-model behavior.

use wallet_service::{
    Address, Amount, Asset, AssetAddress, AssetId, AssetKind, Chain, ChainId, CollectionModel,
    DomainError, LedgerModel,
};

fn chain_id(value: &str) -> ChainId {
    ChainId::try_from(value).expect("test chain identifier should be valid")
}

fn asset_id(value: &str) -> AssetId {
    AssetId::try_from(value).expect("test asset identifier should be valid")
}

fn address(value: &str) -> Address {
    Address::try_from(value).expect("test address should be nonblank")
}

fn bitcoin() -> Chain {
    Chain::new(chain_id("bitcoin-mainnet"), LedgerModel::Utxo)
}

fn ethereum() -> Chain {
    Chain::new(chain_id("ethereum-mainnet"), LedgerModel::Account)
}

fn bitcoin_asset(chain: &Chain) -> Asset {
    Asset::native(asset_id("BTC"), chain, "BTC", 8)
}

fn ether_asset(chain: &Chain) -> Asset {
    Asset::native(asset_id("ETH"), chain, "ETH", 18)
}

fn usdc_asset(chain: &Chain, fee_asset: &Asset) -> Asset {
    Asset::token(
        asset_id("USDC_ETHEREUM"),
        chain,
        "USDC",
        6,
        address("0xa0b8...eB48"),
        fee_asset,
    )
    .expect("USDC test configuration should be valid")
}

#[test]
fn chain_id_should_reject_blank_value() {
    assert_eq!(ChainId::try_from("  "), Err(DomainError::BlankChainId));
}

#[test]
fn asset_id_should_reject_blank_value() {
    assert_eq!(AssetId::try_from(""), Err(DomainError::BlankAssetId));
}

#[test]
fn address_should_reject_blank_value() {
    assert_eq!(Address::try_from("\t"), Err(DomainError::BlankAddress));
}

#[test]
fn native_asset_should_use_utxo_collection_on_bitcoin() {
    let bitcoin = bitcoin();
    let btc = bitcoin_asset(&bitcoin);

    assert_eq!(btc.collection_model(), CollectionModel::Utxo);
}

#[test]
fn native_asset_should_use_account_collection_on_ethereum() {
    let ethereum = ethereum();
    let eth = ether_asset(&ethereum);

    assert_eq!(eth.collection_model(), CollectionModel::Account);
}

#[test]
fn token_asset_should_use_token_collection_on_account_chain() {
    let ethereum = ethereum();
    let eth = ether_asset(&ethereum);
    let usdc = usdc_asset(&ethereum, &eth);

    assert_eq!(usdc.collection_model(), CollectionModel::Token);
}

#[test]
fn token_asset_should_use_configured_native_fee_asset() {
    let ethereum = ethereum();
    let eth = ether_asset(&ethereum);
    let usdc = usdc_asset(&ethereum, &eth);

    assert_eq!(usdc.fee_asset_id(), eth.id());
}

#[test]
fn token_asset_should_keep_contract_address() {
    let ethereum = ethereum();
    let eth = ether_asset(&ethereum);
    let usdc = usdc_asset(&ethereum, &eth);

    assert!(matches!(
        usdc.kind(),
        AssetKind::Token { contract, .. } if contract.as_str() == "0xa0b8...eB48"
    ));
}

#[test]
fn token_asset_should_reject_utxo_chain() {
    let bitcoin = bitcoin();
    let btc = bitcoin_asset(&bitcoin);
    let result = Asset::token(
        asset_id("INVALID_TOKEN"),
        &bitcoin,
        "INVALID",
        8,
        address("not-a-real-contract"),
        &btc,
    );

    assert_eq!(
        result,
        Err(DomainError::TokenOnUtxoChain {
            asset_id: asset_id("INVALID_TOKEN"),
            chain_id: chain_id("bitcoin-mainnet"),
        })
    );
}

#[test]
fn token_asset_should_reject_fee_asset_from_another_chain() {
    let ethereum = ethereum();
    let bitcoin = bitcoin();
    let btc = bitcoin_asset(&bitcoin);
    let result = Asset::token(
        asset_id("USDC_ETHEREUM"),
        &ethereum,
        "USDC",
        6,
        address("0xa0b8...eB48"),
        &btc,
    );

    assert_eq!(
        result,
        Err(DomainError::FeeAssetChainMismatch {
            token_asset_id: asset_id("USDC_ETHEREUM"),
            token_chain_id: chain_id("ethereum-mainnet"),
            fee_asset_id: asset_id("BTC"),
            fee_asset_chain_id: chain_id("bitcoin-mainnet"),
        })
    );
}

#[test]
fn token_asset_should_reject_token_as_fee_asset() {
    let ethereum = ethereum();
    let eth = ether_asset(&ethereum);
    let usdc = usdc_asset(&ethereum, &eth);
    let result = Asset::token(
        asset_id("SECOND_TOKEN"),
        &ethereum,
        "SECOND",
        6,
        address("0xsecond"),
        &usdc,
    );

    assert_eq!(
        result,
        Err(DomainError::TokenFeeAssetNotNative {
            token_asset_id: asset_id("SECOND_TOKEN"),
            fee_asset_id: asset_id("USDC_ETHEREUM"),
        })
    );
}

#[test]
fn asset_address_should_keep_asset_identity() {
    let btc_id = asset_id("BTC");
    let asset_address = AssetAddress::new(btc_id.clone(), address("bc1qexample"));

    assert_eq!(asset_address.asset_id(), &btc_id);
}

#[test]
fn asset_address_should_keep_address_text() {
    let asset_address = AssetAddress::new(asset_id("BTC"), address("bc1qexample"));

    assert_eq!(asset_address.address().as_str(), "bc1qexample");
}

#[test]
fn amount_should_keep_asset_identity() {
    let btc_id = asset_id("BTC");
    let amount = Amount::from_u64(btc_id.clone(), 50_000);

    assert_eq!(amount.asset_id(), &btc_id);
}

#[test]
fn amount_should_keep_atomic_value() {
    let amount = Amount::from_u64(asset_id("BTC"), 50_000);

    assert_eq!(amount.atoms(), &50_000_u64.into());
}
