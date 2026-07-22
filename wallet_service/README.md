# Wallet Service

`wallet-service` is the Rust foundation for chain-neutral crypto deposit wallet
adapters. The crate currently provides asset metadata, generated-key output,
ledger classification, and a minimal object-safe `Wallet` trait. Concrete
Bitcoin, Ethereum, Solana, and token adapters are not implemented yet.

The intended service boundary is stateless: Payment Service owns deposits,
accounting, and the decision to collect, while Indexer Service owns blockchain
observations, confirmations, and reorg handling. See the
[crypto deposit redesign](../crypto-deposit-redesign.md) for the wider system
architecture.

## Current API

- `Asset` stores an asset identifier, chain identifier, symbol, display
  decimals, and an optional token contract or mint address.
- `Keypair` contains a generated address, encoded public key, and optional
  private signing key.
- `ChainId`, `AssetId`, and `Address` are distinct domain types used by asset
  metadata and generated keys.
- `LedgerModel` distinguishes UTXO chains from account-based chains.
- `Wallet` is the object-safe adapter boundary for one configured asset. It
  exposes `asset()` and asynchronous `generate_keypair()` operations.
- `WalletError` currently reports unsupported assets.

Balance lookup, transaction construction, signing, collection, broadcasting,
and chain observation are not part of the current trait.

## Example

```rust
use wallet_service::LedgerModel;

let ledger_model = LedgerModel::Utxo;
assert!(matches!(ledger_model, LedgerModel::Utxo));
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
