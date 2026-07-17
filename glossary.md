# Domain Glossary

| Term | Definition |
|---|---|
| **Deposit** | A payment request created for a user, asset, expected amount, deposit address, and expiration time. |
| **Deposit address** | A blockchain address generated specifically to receive funds for a deposit. |
| **Master wallet** | The system-controlled destination wallet where funds from deposit addresses are collected. |
| **Payment Service (PS)** | Owns deposits, user accounting, transaction classification, the event log, and deposit counters. |
| **Wallet Service (WS)** | Stateless blockchain adapter responsible for generating wallets, reading balances, signing transactions, broadcasting transactions, and collecting funds. |
| **Indexer Service (IX)** | Observes blockchain activity, tracks watched addresses and transactions, and reports transaction-state changes. |
| **Observation** | A blockchain fact discovered by IX, such as a transaction becoming confirmed, failed, dropped, or reorged. |
| **Fact** | Raw blockchain information such as transaction ID, addresses, amounts, status, block, and fee. It contains no business classification. |
| **Business classification** | PS deciding what an observed transaction means—for example, whether it is an incoming payment, sweep, or gas prefund. |
| **Watch list** | Addresses and transaction IDs registered with IX for observation. |
| **`watch(address)`** | Registers an address with IX so transactions involving it are observed. |
| **`watch(txid)`** | Registers a specific transaction with IX so its status changes are observed. |
| **Event log** | Append-only history of relevant IX observations received by PS. Events are added but not overwritten. |
| **Accounting ledger** | Current business state derived from events, including the deposit’s `received`, `collected`, `balance`, and `accounted` counters. |
| **Projection** | The process of converting historical events into current accounting values. |
| **Rebuild / replay** | Reprocessing the event log from the beginning to reconstruct the accounting ledger. |
| **Incoming transaction** | A transaction that transfers funds from an external address to a deposit address. |
| **Collection / sweep** | Moving funds from one or more deposit addresses into the master wallet. |
| **Sweep transaction** | The blockchain transaction created to perform collection. |
| **Pending transaction** | A transaction that was broadcast but has not yet reached a final observed status. |
| **Gas prefund** | Native currency sent from the master wallet to a token deposit address so it can pay the fee for an ERC-20 sweep. |
| **Wallet-based collection** | A single transfer from one deposit address to the master wallet, used for assets such as ETH or SOL. |
| **UTXO collection** | A transaction combining spendable outputs from multiple deposit addresses into an output controlled by the master wallet. |
| **PSBT** | Partially Signed Bitcoin Transaction used to construct and sign a Bitcoin collection transaction with multiple inputs. |
| **ERC-20 collection** | Two-stage collection that may require gas prefunding followed by a token transfer to the master wallet. |
| **Transaction state** | Current IX interpretation of a transaction: pending, confirmed, failed, dropped, or reorged. |
| **Confirmed** | The transaction was included in the blockchain and reached the required confirmation threshold. |
| **Failed** | The blockchain accepted or included the transaction, but its execution was unsuccessful. |
| **Dropped** | The transaction disappeared from the pending transaction pool without being confirmed. |
| **Reorged** | A previously observed transaction changed state because its block was removed during a blockchain reorganization. |
| **Idempotency** | Processing the same event more than once without applying its accounting effect more than once. |
| **`received`** | Total confirmed external funds that have arrived at the deposit address. |
| **`collected`** | Total confirmed funds successfully moved from the deposit address to the master wallet. |
| **`balance`** | Current funds remaining at the deposit address. |
| **`accounted`** | Amount PS has credited to the user’s business balance. |
| **Expected amount** | Amount the user was requested to send. It may differ from the amount actually received. |
| **Actual transferred amount** | Amount the blockchain transaction really moved, as determined by IX. |
| **Deposit key** | Private key controlling a deposit address and used by WS to sign collection transactions. |
| **Chain adapter** | Asset-specific implementation behind WS that handles blockchain-specific operations. |