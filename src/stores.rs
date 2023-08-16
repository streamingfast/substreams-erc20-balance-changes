use substreams::pb::substreams::Clock;
use substreams::scalar::BigInt;
use substreams::store::{StoreAdd, StoreAddBigInt};
use crate::pb::erc20::types::v1::BalanceChanges;
use substreams::store::StoreNew;

#[substreams::handlers::store]
pub fn store_valid_balance_changes(balance_changes: BalanceChanges, store: StoreAddBigInt) {
    for change in balance_changes.balance_changes {
        if change.is_valid {
            store.add(0, "valid", BigInt::from(1));
        }

        store.add(0, "total", BigInt::from(1));
    }
}