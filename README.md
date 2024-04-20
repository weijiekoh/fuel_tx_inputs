```bash
cargo run
```

Output:

```
tx inputs: [Contract(Contract { utxo_id: UtxoId { tx_id: 0000000000000000000000000000000000000000000000000000000000000000, output_index: 0 }, balance_root: 0000000000000000000000000000000000000000000000000000000000000000, state_root: 0000000000000000000000000000000000000000000000000000000000000000, tx_pointer: TxPointer { block_height: 00000000, tx_index: 0 }, contract_id: 7f4aa1caeb68a5c768b0a56085e38f3b6c0db67346f0504af5141c35978e5ce1 }), CoinSigned(Coin { utxo_id: UtxoId { tx_id: 876623495f0f2d32a0d6303adc9aac61f1b1a026609635d88c0da4e2ea88aeb2, output_index: 1 }, owner: 09c0b2d1a486c439a87bcba6b46a7a1a23f3897cc83a94521a96da5c23bc58db, amount: 100, asset_id: 0000000000000000000000000000000000000000000000000000000000000000, tx_pointer: TxPointer { block_height: 00000000, tx_index: 0 }, witness_index: 0, predicate_gas_used: Empty, predicate: Empty, predicate_data: Empty })]

fetched tx inputs: [Contract(Contract { utxo_id: UtxoId { tx_id: 876623495f0f2d32a0d6303adc9aac61f1b1a026609635d88c0da4e2ea88aeb2, output_index: 0 }, balance_root: 0000000000000000000000000000000000000000000000000000000000000000, state_root: dab5d0b9b43773eab24ef7b9413d36f849c9627fd37882a52927685e8e1c3163, tx_pointer: TxPointer { block_height: 00000001, tx_index: 0 }, contract_id: 7f4aa1caeb68a5c768b0a56085e38f3b6c0db67346f0504af5141c35978e5ce1 }), CoinSigned(Coin { utxo_id: UtxoId { tx_id: 876623495f0f2d32a0d6303adc9aac61f1b1a026609635d88c0da4e2ea88aeb2, output_index: 1 }, owner: 09c0b2d1a486c439a87bcba6b46a7a1a23f3897cc83a94521a96da5c23bc58db, amount: 100, asset_id: 0000000000000000000000000000000000000000000000000000000000000000, tx_pointer: TxPointer { block_height: 00000001, tx_index: 0 }, witness_index: 0, predicate_gas_used: Empty, predicate: Empty, predicate_data: Empty })]

```
