# Iwallet Service Interface

## Description

The `Iwallet Service` is a service that allows to manage the wallets of the users. It is used by the `Wallet Service` to manage the wallets.

## Specification


Trait `IwalletService` is used to manage the wallets of the users.

### Methods
```rust
- generate_keypair(asset) - Generate a deposit private key and address
- get_balance(asset, address) - Read an address’s native or token balance
- build_transaction (request) - Construct an unsigned blockchain transaction.**
- sign_transaction (transaction, key) - Sign a transaction.
- broadcast_transaction (transaction) - Submit a signed transaction and return its txid.
- collect(request) - Sweep an account-based asset such as ETH or SOL into the master wallet.
- collect_batch(request) - Combine multiple BTC deposit UTXOs into one master-wallet transaction.
- get_native_balance_and_required_gas(request) - Check whether an ERC-20 deposit address has enough native currency to pay gas.
- prefund_gas(request) - Send native currency from the master wallet to an ERC-20 deposit address.
- sweep_token(request) - Transfer ERC-20 tokens from a deposit address to the master address.
- stream_blocks(chain, cursor) - Stream blocks from a chain.

```
