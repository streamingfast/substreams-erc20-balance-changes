use std::collections::HashMap;

use substreams::pb::substreams::Clock;
use substreams_ethereum::pb::eth::v2::{Call, TransactionTrace};

use crate::abi::erc20::events::Transfer;

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

    for (hash, preimage) in &call.keccak_preimages {
        if preimage.len() != 128 {
            continue;
        }

        if &preimage[64..126] != "00000000000000000000000000000000000000000000000000000000000000" {
            continue;
        }

        let addr = &preimage[24..64];
        out.insert(
            hex::decode(hash).expect("Failed to decode hash hex string"),
            hex::decode(addr).expect("Failed to decode address hex string"),
        );
    }
    out
}

pub fn compute_keccak_address_map(calls: Vec<Call>) -> StorageKeyToAddressMap {
    let mut keccak_address_map = HashMap::new();

    for call in calls {
        keccak_address_map.extend(addresses_for_storage_keys(&call));
    }
    keccak_address_map
}

pub fn erc20_is_valid_address(address: &Vec<u8>, transfer: &Transfer) -> bool {
    address == &transfer.from || address == &transfer.to
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