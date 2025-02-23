use std::collections::HashSet;

use crate::pb::erc20::types::v1::Events;
use substreams::scalar::BigInt;
use substreams::store::StoreNew;
use substreams::store::{StoreAdd, StoreAddBigInt};

#[substreams::handlers::store]
pub fn store_valid_balance_changes(events: Events, store: StoreAddBigInt) {
    let mut balance_changes_type_1: u64 = 0;
    let mut balance_changes_type_2: u64 = 0;
    let mut balance_changes: u64 = 0;
    let mut transfers: u64 = 0;
    let mut transfers_not_matched: u64 = 0;

    let mut logs = HashSet::new();
    for change in events.balance_changes {
        let key = format!("{}:{}", change.transaction_id, change.log_index);
        logs.insert(key);
        balance_changes += 1;
        match change.balance_change_type {
            1 => {
                balance_changes_type_1 += 1;
            }
            2 => {
                balance_changes_type_2 += 1;
            }
            _ => {}
        }
    }
    // transfers that do not match any balance changes (type0)
    for transfer in events.transfers {
        let key = format!("{}:{}", transfer.transaction_id, transfer.log_index);
        if !logs.contains(&key) {
            transfers_not_matched += 1;
        }
        transfers += 1;
    }

    store.add(
        0,
        "balance_changes_type_1",
        BigInt::from(balance_changes_type_1),
    );
    store.add(
        0,
        "balance_changes_type_2",
        BigInt::from(balance_changes_type_2),
    );
    store.add(0, "balance_changes", BigInt::from(balance_changes));
    store.add(0, "transfers", BigInt::from(transfers));
    store.add(
        0,
        "transfers_not_matched",
        BigInt::from(transfers_not_matched),
    );
}
