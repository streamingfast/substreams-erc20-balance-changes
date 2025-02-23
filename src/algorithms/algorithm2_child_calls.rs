use std::collections::HashMap;

use crate::abi::erc20::events::Transfer;
use crate::pb::erc20::types::v1::BalanceChangeType;
use substreams::log;
use substreams::Hex;
use substreams_ethereum::pb::eth::v2::{Call, StorageChange, TransactionTrace};

use super::utils::{get_keccak_address, is_erc20_valid_address};
use super::utils::{Address, Hash};

pub fn get_all_child_call_storage_changes<'a>(original: &'a Call, trx: &'a TransactionTrace) -> impl Iterator<Item = &'a StorageChange> + 'a {
    trx.calls.iter().filter(move |call| call.parent_index == original.index).flat_map(|call| call.storage_changes.iter())
}

// algorithm #2 (case where storage changes are not in the same call as the transfer event)
pub fn find_erc20_balance_changes_algorithm2<'a>(
    trx: &'a TransactionTrace,
    original: &'a Call,
    transfer: &'a Transfer,
    keccak_address_map: &'a HashMap<Hash, Address>,
) -> Vec<(Address, &'a StorageChange, BalanceChangeType)> {
    let mut out = Vec::new();

    // check if any of the storage changes match the transfer.to or transfer.from
    for storage_change in get_all_child_call_storage_changes(original, trx) {
        let owner = match get_keccak_address(keccak_address_map, storage_change) {
            Some(address) => address,
            None => continue,
        };
        if !is_erc20_valid_address(&owner, transfer) {
            log::info!("owner={} does not match transfer from={} to={}", Hex(owner), Hex(&transfer.from), Hex(&transfer.to));
            continue;
        }
        out.push((owner, storage_change, BalanceChangeType::BalanceChangeType2));
    }
    out
}
