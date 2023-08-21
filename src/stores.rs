use substreams::scalar::BigInt;
use substreams::store::{StoreAdd, StoreAddBigInt};
use crate::pb::erc20::types::v1::BalanceChanges;
use substreams::store::StoreNew;

#[substreams::handlers::store]
pub fn store_valid_balance_changes(balance_changes: BalanceChanges, store: StoreAddBigInt) {
    for change in balance_changes.balance_changes {
        match change.r#type {
            0 => {
                store.add(0, "type0", BigInt::from(1));
                store.add(0, "total", BigInt::from(1));
            },
            1 => {
                store.add(0, "type1", BigInt::from(1));
                store.add(0, "total", BigInt::from(1));
            },
            66 => {
                store.add(0, "type66", BigInt::from(1));
                store.add(0, "total", BigInt::from(1));
            },
            _ => {}
        }
    }
}