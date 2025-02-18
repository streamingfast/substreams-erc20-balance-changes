use substreams::log;
use substreams_ethereum::pb::eth::v2::{Call, StorageChange, TransactionTrace};
use substreams::scalar::BigInt;
use substreams::Hex;
use crate::abi::erc20::events::Transfer;
use crate::pb::erc20::types::v1::BalanceChangeType;

use super::utils::{get_owner_from_keccak_address_map, is_erc20_valid_address, StorageKeyToAddressMap};


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
    let mut storage_changes = Vec::new();
    for call in child_calls.iter() {
        storage_changes.extend(call.storage_changes.clone());
    }

    let mut total_sent = BigInt::zero();
    let mut total_received = BigInt::zero();

    //check if any of the storage changes match the transfer.to or transfer.from
    for storage_change in storage_changes.clone().iter() {
        let owner = match get_owner_from_keccak_address_map(keccak_address_map, &storage_change) {
            Some(address) => address,
            None => continue
        };

        if !is_erc20_valid_address(&owner, transfer) {
            log::info!("owner={} does not match transfer from={} to={}", Hex(owner), Hex(&transfer.from), Hex(&transfer.to));
            continue;
        }

        let old_balance = BigInt::from_unsigned_bytes_be(&storage_change.old_value);
        let new_balance = BigInt::from_unsigned_bytes_be(&storage_change.new_value);

        let balance_change = new_balance - old_balance;
        if balance_change < BigInt::zero() {
            total_sent = total_sent + balance_change.neg();
        } else {
            total_received = total_received + balance_change;
        };

        out.push((owner, storage_change.clone(), BalanceChangeType::BalanceChangeType2));
    }

    if total_sent == transfer.value {
        return out;
    }

    let mut diff = total_sent - total_received;
    if diff < BigInt::zero() {
        diff = diff.neg();
    }

    // look for a storage change that matches the diff
    for storage_change in storage_changes.iter() {

        // Check if the transfer matches the storage change balance changes
        let owner = match get_owner_from_keccak_address_map(keccak_address_map, &storage_change) {
            Some(address) => address,
            None => continue
        };

        // make sure owner is either the sender or receiver
        if !is_erc20_valid_address(&owner, transfer) {
            log::info!("owner={} does not match transfer from={} to={}", Hex(owner), Hex(&transfer.from), Hex(&transfer.to));
            continue;
        }

        let old_balance = BigInt::from_unsigned_bytes_be(&storage_change.old_value);
        let new_balance = BigInt::from_unsigned_bytes_be(&storage_change.new_value);

        let mut balance_change = new_balance - old_balance;
        if balance_change < BigInt::zero() {
            balance_change = balance_change.neg();
        }

        if balance_change != diff {
            log::info!("Algo2: Balance change does not match transfer value. Balance change: {}, transfer value: {}", balance_change, transfer.value);
            continue;
        }

        out.push((owner, storage_change.clone(), BalanceChangeType::BalanceChangeType2));
    }

    out
}