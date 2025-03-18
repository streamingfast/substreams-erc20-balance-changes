use std::collections::HashSet;

use erc20::algorithms::transfers::get_erc20_transfer;
use substreams::scalar::BigInt;
use substreams::store::{StoreAdd, StoreAddBigInt, StoreNew};
use substreams::Hex;
use substreams_ethereum::pb::eth::v2::Block;

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
