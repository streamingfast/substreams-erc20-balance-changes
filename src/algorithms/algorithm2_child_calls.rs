use substreams::log;
use substreams_ethereum::pb::eth::v2::{Call, StorageChange, TransactionTrace};
use substreams::Hex;
use crate::abi::erc20::events::Transfer;
use crate::pb::erc20::types::v1::BalanceChangeType;

use super::utils::{get_keccak_address, is_erc20_valid_address, StorageKeyToAddressMap};


pub fn get_all_child_calls(original: &Call, trx: &TransactionTrace) -> Vec<Call> {
    let mut out = Vec::new();

    for call in trx.calls.iter() {
        if call.parent_index == original.index {
            out.push(call.clone());
        }
    }
    out
}

// algorithm #2 (case where storage changes are not in the same call as the transfer event)
pub fn find_erc20_balance_changes_algorithm2(
    child_calls: Vec<Call>,
    transfer: &Transfer,
    keccak_address_map: &StorageKeyToAddressMap,
) -> Vec<(Vec<u8>, StorageChange, BalanceChangeType)> {
    let mut out = Vec::new();

    //get all storage changes for these calls:
    for call in child_calls.iter() {
        //check if any of the storage changes match the transfer.to or transfer.from
        for storage_change in call.storage_changes.iter() {
            let owner = match get_keccak_address(keccak_address_map, &storage_change) {
                Some(address) => address,
                None => continue
            };
            if !is_erc20_valid_address(&owner, transfer) {
                log::info!("owner={} does not match transfer from={} to={}", Hex(owner), Hex(&transfer.from), Hex(&transfer.to));
                continue;
            }
            out.push((owner, storage_change.clone(), BalanceChangeType::BalanceChangeType2));
        }
    }
    out
}
