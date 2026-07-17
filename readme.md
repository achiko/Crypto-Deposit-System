# Crypto Deposit and Payment Platform

This project provides a multi-asset cryptocurrency deposit system. Blockchain-specific operations are isolated behind the **Wallet Service (WS)** so the rest of the platform can work with every supported asset through one common interface.

## Wallet Service

The Wallet Service is a stateless blockchain adapter. It does not own deposit records, user balances, transaction history, or other application data.

For each supported asset, WS creates an asset-specific implementation of the shared `IWallet` interface. The implementation understands the rules, transaction format, fee model, signing process, and RPC API of that blockchain.

Conceptually, the interface provides operations such as:

```text
IWallet
  generateKeypair()
  getBalance(address)
  buildTransfer(...)
  signTransaction(...)
  broadcastTransaction(...)
  collect(...)
  streamBlocks()
```

The exact method signatures are an implementation detail, but all asset adapters expose the same capabilities to the Payment Service and Indexer Service.

## Asset-specific implementations

Each adapter contains only the behavior needed for its blockchain or token type.

### Wallet-based assets

Examples: ETH and SOL.

Collection is normally a single transaction signed by the deposit wallet:

```text
deposit address -> master wallet
```

### UTXO assets

Example: BTC.

The adapter discovers unspent transaction outputs and can combine multiple deposit wallets into one batch transaction:

```text
multiple deposit inputs -> master wallet output
```

Each input is signed with the key belonging to its deposit address.

### Smart-contract tokens

Example: ERC-20 tokens.

The deposit address needs the blockchain's native asset to pay transaction fees. Collection may therefore require two transactions:

1. Send native currency from the master wallet to the deposit address for gas.
2. Transfer the token from the deposit address to the master wallet.

## Responsibilities

An asset-specific wallet implementation is responsible for:

- Address and keypair generation.
- Reading native or token balances.
- Estimating transaction fees.
- Building and signing transactions.
- Broadcasting transactions to the blockchain.
- Applying the correct collection strategy for the asset.
- Providing or consuming blockchain block data where required.

It is not responsible for:

- User or deposit accounting.
- Deciding when funds should be collected.
- Persisting transaction state.
- Classifying transactions as deposits or sweeps.
- Tracking confirmations, dropped transactions, or reorganizations.

Those responsibilities belong to the Payment Service and Indexer Service described in [crypto-deposit-redesign.md](crypto-deposit-redesign.md).
