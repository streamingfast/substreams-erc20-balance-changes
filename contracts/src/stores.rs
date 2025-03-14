use std::collections::HashSet;

use erc20::algorithms::transfers::get_erc20_transfer;
use substreams::pb::substreams::Clock;
use substreams::scalar::BigInt;
use substreams::store::{StoreNew, StoreAdd, StoreAddBigInt, StoreSet, StoreSetString};
use substreams::Hex;
use substreams_ethereum::pb::eth::v2::{Block, CallType};

#[substreams::handlers::store]
pub fn store_erc20_transfers(block: Block, store: StoreAddBigInt) {
    let mut transfers = HashSet::new();

    // find all ERC-20 transfers in the block
    for trx in block.transactions() {
        for (log, call_view) in trx.logs_with_calls() {
            let call = call_view.call;
            if get_erc20_transfer(trx, call, log).is_some() {
                transfers.insert(log.address.clone());
            }
        }
    }
    // increment the count for each new ERC-20 address per block
    for address in transfers {
        store.add(0, Hex::encode(address), BigInt::one());
    }
}

#[substreams::handlers::store]
pub fn store_contract_creation(clock: Clock, block: Block, store: StoreSetString) {
    for trx in block.transactions() {
        for call in &trx.calls {
            if call.call_type() != CallType::Create { continue }
            for code_change in &call.code_changes {
                let address = &Hex::encode(code_change.address.clone());
                store.set(0, format!("clock.id:{}", address), &clock.id.to_string());
                store.set(0, format!("clock.number:{}", address), &clock.number.to_string());
                store.set(0, format!("clock.timestamp.seconds:{}", address), &clock.timestamp.unwrap().seconds.to_string());
                store.set(0, format!("trx.hash:{}", address), &Hex::encode(&trx.hash));
                store.set(0, format!("trx.from:{}", address), &Hex::encode(&trx.from));
                store.set(0, format!("trx.to:{}", address), &Hex::encode(&trx.to));
            }
        }
    }
}
