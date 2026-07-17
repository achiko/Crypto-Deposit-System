# Bitcoin Reorganization Detection and Payment Recovery

**Research date:** July 17, 2026

**Scope:** Bitcoin Core RPC and ZMQ, indexer synchronization, and payment-state
recovery.

## 1. Purpose

This document describes how a Bitcoin payment or indexing service should detect
chain reorganizations and recover without losing payment history or crediting a
transaction that is no longer confirmed.

The design assumes that Bitcoin Core is the authoritative chain source. The same
chain-comparison model can be used with another Bitcoin-compatible backend, but
its notification and RPC semantics must be verified separately.

## 2. Conclusion

A service should never identify a processed block by height alone. It must store
at least:

```text
height
block_hash
previous_block_hash
```

The service detects a reorganization when its stored block hash at a height
differs from Bitcoin Core's active-chain block hash at that same height.

The safe synchronization procedure is:

1. Treat a new-block notification as a signal to synchronize, not as complete
   chain history.
2. Read Bitcoin Core's current best height and block hash.
3. Compare the locally stored chain with the active chain.
4. Walk backward until both sides contain the same block hash at the same
   height. This is the common ancestor.
5. Roll back every locally processed block above that ancestor.
6. Process the new active branch in ascending height order.
7. Recalculate the state of every payment affected by the disconnected blocks.

Waiting for additional confirmations reduces reorganization risk, but it does
not remove the requirement to detect and recover from reorganizations.

## 3. Why Parent-Hash Comparison Is Not Enough

For an uninterrupted block stream, a normal extension satisfies:

```text
newBlock.previousBlockHash == localTip.blockHash
```

A mismatch is an important warning, but it does not by itself prove that a
reorganization occurred. It can also mean:

- The subscriber missed one or more notifications.
- The service was offline while blocks were produced.
- A ZMQ message was dropped.
- A load-balanced provider returned results from different nodes.

The definitive check compares canonical block hashes at equal heights. If the
stored hash at height `H` differs from `getblockhash(H)`, the stored block at
that height is no longer part of the active chain.

## 4. Persistent Data Model

An indexer should retain enough block history to find a common ancestor and
reverse block effects.

### 4.1 Active block records

```text
IndexedBlock
  height
  hash
  previous_hash
  processed_at
  active
```

The pair `(height, hash)` should be unique. Replaced blocks should normally be
marked inactive rather than immediately deleted, because they are useful for
auditing and incident investigation.

### 4.2 Transaction confirmation evidence

```text
PaymentObservation
  txid
  block_hash
  block_height
  transaction_index
  confirmations
  status
```

Storing `block_hash` is essential. A transaction's height alone cannot prove
which branch contained it.

### 4.3 Reversible block effects

Every state change derived from a block must be reversible. This can be
implemented with:

- Per-block undo records.
- An append-only event ledger followed by state reconstruction.
- Rows tagged with the block hash that created them.
- A database transaction that reverses all effects associated with a block.

Financial history should not be silently deleted during rollback.

## 5. RPC-Based Detection Algorithm

Bitcoin Core's `getblockhash height` returns the block hash in the current best
chain at that height. This makes it the authoritative comparison operation.

```ts
async function synchronizeChain(): Promise<void> {
  const localTip = await database.getIndexedTip();
  const remote = await bitcoinRpc.getBlockchainInfo();

  if (!localTip) {
    await indexRange(config.startHeight, remote.blocks);
    return;
  }

  let commonHeight = Math.min(localTip.height, remote.blocks);

  while (commonHeight >= config.startHeight) {
    const localHash = await database.getActiveBlockHash(commonHeight);
    const remoteHash = await bitcoinRpc.getBlockHash(commonHeight);

    if (localHash === remoteHash) {
      break;
    }

    commonHeight--;
  }

  if (commonHeight < config.startHeight) {
    throw new Error("No common ancestor inside the retained history");
  }

  if (commonHeight < localTip.height) {
    await rollbackBlocks(localTip.height, commonHeight + 1);
  }

  const refreshedTip = await bitcoinRpc.getBlockchainInfo();
  await indexRange(commonHeight + 1, refreshedTip.blocks);
}
```

For every block added during `indexRange`, validate continuity before applying
its effects:

```ts
if (block.previousblockhash !== expectedPreviousHash) {
  // The chain changed again while synchronization was in progress.
  // Abort this pass and restart reconciliation from persisted state.
  throw new ChainChangedDuringSyncError();
}
```

Rollback and forward indexing should be idempotent. A crash at any point must be
recoverable by running the same reconciliation procedure again.

## 6. Quick Check for a Stored Block

Bitcoin Core also provides a useful diagnostic:

```bash
bitcoin-cli getblockheader <stored-block-hash>
```

The verbose response contains `confirmations`. A value of `-1` means the block
is not on the active chain. This is convenient for checking a known block, but
the common-ancestor algorithm is still required to determine rollback depth.

## 7. ZMQ Notifications

### 7.1 Recommended topic

Bitcoin Core's ZMQ `sequence` topic explicitly publishes:

- `C`: a block was connected.
- `D`: a block was disconnected.

Example configuration:

```ini
zmqpubsequence=tcp://127.0.0.1:28332
```

A `D` notification is an immediate reorganization signal. The service should
record it for observability and start RPC reconciliation.

### 7.2 Why RPC reconciliation remains mandatory

ZMQ is a publish/subscribe transport without replay or acknowledgement. Messages
may be missed because of subscriber downtime, reconnection, queue limits, or
network interruption. Bitcoin Core includes a per-topic message counter so the
subscriber can notice gaps, but it cannot retrieve missing messages from ZMQ.

Therefore:

```text
ZMQ notification -> wake synchronization worker -> reconcile through RPC
```

Do not make ZMQ delivery the sole source of persisted chain truth.

### 7.3 `hashblock` and `rawblock`

The `hashblock` and `rawblock` topics notify when the chain tip changes, but
they do not provide the full disconnected and connected sequence for a reorg.
The subscriber must retrieve the path from its last known block to the new tip.
Use `sequence` when explicit connect/disconnect visibility is wanted.

## 8. Payment-State Recovery

When a block is disconnected, all payments observed in that block lose their
confirmation evidence from that branch. They must not remain unconditionally
settled merely because they were previously confirmed.

A useful state model is:

```text
observed_in_mempool
        |
        v
confirmed_in_block -> settled_by_policy
        |
        v
     reorged
      /   \
     v     v
pending   conflicted_or_dropped
     |
     v
confirmed_in_replacement_branch
```

After rollback, check each affected transaction:

1. **Included in the replacement branch:** update its new block hash, height,
   and confirmation count.
2. **Present in the mempool:** return it to a pending or unconfirmed state.
3. **Absent from both chain and mempool:** keep it as reorged/dropped and
   continue reconciliation according to retention policy.
4. **Conflicting spend confirmed:** mark the original transaction conflicted or
   double-spent and prevent credit based on the original observation.

Notifications and balance updates should be idempotent. If the application has
already credited a customer, the business layer needs an explicit compensating
entry or risk workflow rather than destructive history edits.

## 9. Confirmation Policy

Confirmation count should be derived from active-chain state:

```text
confirmations = active_tip_height - payment_block_height + 1
```

This calculation is valid only after verifying that `payment_block_hash` is
still canonical. A high stored confirmation number must not survive when its
block becomes inactive.

The number of confirmations required before settlement is a business-risk
policy based on payment value and the cost of reversal. It is separate from the
technical reorg-detection mechanism.

## 10. Important Edge Cases

### Missed blocks without a reorg

If the local tip is still an ancestor of the remote tip, there is no rollback.
Index the missing range normally.

### Replacement at the same height

Two different hashes at the same height prove that the locally stored block is
not the current canonical block at that height.

### Remote tip below the local tip

Do not assume height only increases. Start comparison at the lower of the two
tip heights, find the common ancestor, and roll back the excess local blocks.

### Reorg during recovery

The node tip may change while the service is rolling back and replaying. Check
parent continuity for every new block and restart synchronization if the branch
changes again.

### Notification loss or process restart

Run reconciliation at startup, after every reconnect, whenever a notification
sequence gap is detected, and periodically even when the stream appears healthy.

### Load-balanced RPC endpoints

Different backend nodes may temporarily report different tips. A payment
indexer should use a stable authoritative node or ensure that all related calls
are pinned to a consistent backend. Provider diversity can be used for
monitoring, but inconsistent responses must not be combined into one chain
view.

### Retention limit exceeded

If no common ancestor exists inside retained local history, stop incremental
processing and rescan from a known-good checkpoint. Do not guess the fork point.

## 11. Operational Signals

Record at least:

- Old tip height and hash.
- New tip height and hash.
- Common ancestor height and hash.
- Number and hashes of disconnected blocks.
- Number and hashes of connected blocks.
- Affected payment transaction IDs.
- ZMQ sequence gaps.
- Reconciliation duration and failures.

Alert on deep reorganizations, repeated branch changes, missing common
ancestors, failed rollbacks, and RPC backends returning inconsistent canonical
hashes.

## 12. Regtest Verification Plan

The implementation should be tested against a controlled Bitcoin Core regtest
network:

1. Mine a chain containing a test payment.
2. Process the payment and record its block hash and height.
3. Create a longer competing branch that excludes the payment or contains a
   conflicting spend.
4. Let Bitcoin Core activate the longer branch.
5. Verify that the service finds the exact common ancestor.
6. Verify that old block effects are reversed in descending height order.
7. Verify that replacement blocks are applied in ascending height order.
8. Verify the final payment state for re-included, mempool, dropped, and
   conflicting transactions.
9. Repeat while stopping the subscriber to prove that RPC reconciliation works
   even when ZMQ notifications are missed.
10. Repeat with a service crash during rollback and during forward indexing to
    prove idempotent recovery.

`invalidateblock` and `reconsiderblock` are useful for regtest scenarios, but
they modify node chainstate and should never be used against an unscoped
production node.

## 13. Implementation Checklist

- [ ] Persist block height, hash, and previous hash.
- [ ] Bind each confirmed payment to its confirming block hash.
- [ ] Compare local and canonical hashes at equal heights.
- [ ] Walk backward to the common ancestor on mismatch.
- [ ] Make every block-derived database change reversible.
- [ ] Roll back old blocks before applying the replacement branch.
- [ ] Make rollback and replay idempotent.
- [ ] Use ZMQ only as a synchronization trigger.
- [ ] Detect ZMQ message-counter gaps.
- [ ] Reconcile at startup, reconnect, sequence gap, and periodically.
- [ ] Re-evaluate affected transactions after rollback.
- [ ] Keep settlement policy separate from confirmation detection.
- [ ] Test normal gaps, shallow and deep reorgs, conflicts, restarts, and crashes.

## 14. Primary References

- [Bitcoin Core 31.0 `getblockheader`](https://bitcoincore.org/en/doc/31.0.0/rpc/blockchain/getblockheader/)
- [Bitcoin Core 31.0 `getblockhash`](https://bitcoincore.org/en/doc/31.0.0/rpc/blockchain/getblockhash/)
- [Bitcoin Core ZMQ documentation](https://github.com/bitcoin/bitcoin/blob/master/doc/zmq.md)
