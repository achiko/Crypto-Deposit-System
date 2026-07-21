# Wallet Service

`wallet-service` is the Rust foundation for chain-neutral crypto deposit wallet
adapters. The crate currently provides a validated domain model and a minimal,
object-safe `Wallet` trait. Concrete Bitcoin, Ethereum, Solana, and token
adapters are not implemented yet.

The intended service boundary is stateless: Payment Service owns deposits,
accounting, and the decision to collect, while Indexer Service owns blockchain
observations, confirmations, and reorg handling. See the
[crypto deposit redesign](../crypto-deposit-redesign.md) for the wider system
architecture.

## Current API

- `ChainId`, `AssetId`, and `Address` prevent accidental interchange with
  arbitrary strings and reject blank values. Chain-specific adapters must still
  validate and normalize address syntax.
- `Chain` records whether a network uses a UTXO or account ledger.
- `Asset` separates native assets from tokens and selects a validated
  `CollectionModel` during construction.
- Token construction rejects UTXO chains, cross-chain fee assets, and token fee
  assets. A token must use a native fee asset on the same account-based chain.
- `AssetAddress` associates an address with its asset without copying all asset
  metadata.
- `Amount` stores an arbitrary-size, non-negative integer in atomic units. The
  asset's `decimals` value is display metadata; monetary values are not stored
  as floating point numbers.
- `Wallet` is currently the object-safe adapter boundary for one configured
  asset and exposes `asset()`. Key management, balances, transaction building,
  signing, collection, broadcasting, and chain observation remain future work.

## Example

```rust
use wallet_service::{
    Asset, AssetId, Chain, ChainId, CollectionModel, DomainError, LedgerModel,
};

fn main() -> Result<(), DomainError> {
    let chain_id = ChainId::try_from("bitcoin-mainnet")?;
    let bitcoin = Chain::new(chain_id, LedgerModel::Utxo);
    let btc_id = AssetId::try_from("BTC")?;
    let btc = Asset::native(btc_id, &bitcoin, "BTC", 8);

    assert_eq!(btc.collection_model(), CollectionModel::Utxo);
    Ok(())
}
```

Run the example binary:

```shell
cargo run --locked
```

## Development

The crate targets Rust 1.75 or newer and denies missing public documentation,
unsafe code, and Clippy's `all` lint group.

Run the full local verification suite from this directory:

```shell
cargo fmt --all -- --check
cargo check --all-targets --all-features --locked
cargo clippy --all-targets --all-features --locked -- -D warnings
cargo test --all-features --locked
cargo test --doc --locked
RUSTDOCFLAGS="-D warnings" cargo doc --no-deps --locked
```
