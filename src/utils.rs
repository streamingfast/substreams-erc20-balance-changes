use std::collections::HashMap;
use crate::abi::{self};

use substreams::{log, pb::substreams::Clock, Hex};
use substreams_ethereum::pb::eth::v2::{Call, StorageChange, TransactionTrace};
use substreams::scalar::BigInt;
use abi::erc20::events::Transfer as TransferAbi;

use crate::abi::erc20::events::Transfer;
use hex_literal::hex;

// const NULL_ADDRESS: [u8; 20] = hex!("0000000000000000000000000000000000000000");
const ZERO_STORAGE_PREFIX: [u8; 16] = hex!("00000000000000000000000000000000");

// Timestamp to date conversion
// ex: 2015-07-30T16:02:18Z => 2015-07-30
pub fn clock_to_date(clock: &Clock) -> String {
    match clock.timestamp.expect("timestamp missing").to_string().split('T').next() {
        Some(date) => date.to_string(),
        None => "".to_string(),
    }
}

// In ClickHouse, an aggregate function like argMax can only take one expression as the “ordering” argument.
// So we typically combine (block_num, index) into a single monotonic integer.
// For example, if each of block_num and index fits in 32 bits, we can do:
// max(toUInt64(block_num) * 2^32 + index) AS version
pub fn index_to_version(clock: &Clock, index: u32) -> u64 {
    clock.number << 32 + index
}

pub type StorageKeyToAddressMap = HashMap<Vec<u8>, Vec<u8>>;

pub fn addresses_for_storage_keys(call: &Call) -> StorageKeyToAddressMap {
    let mut out = HashMap::new();

    for (storage_key, preimage) in &call.keccak_preimages {
        if preimage.len() != 128 {
            log::info!("Skipping storage key {} with invalid length", storage_key);
            continue;
        }

        if &preimage[64..126] != "00000000000000000000000000000000000000000000000000000000000000" {
            log::info!("Skipping storage key {} with non-zero padding", storage_key);
            continue;
        }

        let address = &preimage[24..64];
        out.insert(
            hex::decode(storage_key).expect("Failed to decode hash hex string"),
            hex::decode(address).expect("Failed to decode address hex string"),
        );
    }
    out
}

pub fn get_owner_from_keccak_address_map(
    keccak_address_map: &StorageKeyToAddressMap,
    storage_change: &StorageChange
) -> Option<Vec<u8>> {
    // If found in the map, clone it; otherwise enter the or_else closure
    keccak_address_map
        .get(&storage_change.key)
        .cloned()
        .or_else(|| {
            // If key starts with zero prefix, log that we're skipping it
            if storage_change.key.starts_with(&ZERO_STORAGE_PREFIX[..]) {
                log::info!(
                    "skip zero key storage change key={}",
                    Hex(&storage_change.key)
                );
            } else {
                log::info!(
                    "storage change does not match any owner address key={}",
                    Hex(&storage_change.key)
                );
            }
            None
        })
}

pub fn compute_keccak_address_map(calls: Vec<Call>) -> StorageKeyToAddressMap {
    let mut keccak_address_map = HashMap::new();

    for call in calls {
        keccak_address_map.extend(addresses_for_storage_keys(&call));
    }
    keccak_address_map
}

pub fn is_erc20_valid_address(address: &Vec<u8>, transfer: &Transfer) -> bool {
    address == &transfer.from || address == &transfer.to
}

pub fn is_erc20_valid_balance(transfer: &TransferAbi, storage_change: &StorageChange) -> bool {
    let old_balance = BigInt::from_signed_bytes_be(&storage_change.old_value);
    let new_balance = BigInt::from_signed_bytes_be(&storage_change.new_value);

    // Absolute balance change
    let balance_change_abs = BigInt::absolute(&(new_balance - old_balance));

    // Absolute transfer value
    let transfer_value_abs = BigInt::absolute(&transfer.value);

    // Compare the difference in absolute terms.
    // We allow a difference of 0 or 1 wei, i.e., if |balance_change_abs - transfer_value_abs| > 1, we continue.
    // https://github.com/streamingfast/substreams-erc20-balance-changes/issues/14
    let diff = BigInt::absolute(&(&balance_change_abs - &transfer_value_abs));
    if diff > BigInt::one() {
        log::info!("Balance change does not match transfer value. Balance change: {}, transfer value: {}", balance_change_abs, transfer_value_abs);
        return false;
    }
    return true;
}

pub fn get_all_child_calls(original: &Call, trx: &TransactionTrace) -> Vec<Call> {
    let mut out = Vec::new();

    for call in trx.calls.iter() {
        if call.parent_index == original.index {
            out.push(call.clone());
        }
    }

    out
}