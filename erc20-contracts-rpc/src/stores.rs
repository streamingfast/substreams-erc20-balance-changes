use std::collections::HashSet;

use erc20::algorithms::transfers::get_erc20_transfer;
use substreams::scalar::BigInt;
use substreams::store::{StoreAdd, StoreAddBigInt, StoreNew};
use substreams::Hex;
use substreams_ethereum::pb::eth::v2::Block;

#[substreams::handlers::store]
pub fn store_erc20_transfers(block: Block, store: StoreAddBigInt) {
    // Find all contracts with transfers
    let transfers: HashSet<&[u8]> = block
        .transactions()
        .flat_map(|trx| {
            trx.logs_with_calls()
                .filter_map(move |(log, call_view)| get_erc20_transfer(trx, call_view.call, log).map(|_| log.address.as_ref()))
        })
        .collect();

    // flag token contracts with transfers in store
    for address in transfers {
        store.add(0, Hex::encode(address), BigInt::one());
    }
}
