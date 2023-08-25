use crate::abi::{self};
use crate::pb::erc20::types::v1::{BalanceChange, BalanceChangeType, BalanceChanges};
use abi::erc20::events::Transfer;
use hex_literal::hex;
use std::collections::HashMap;
use substreams::errors::Error;
use substreams::log::info;
use substreams::scalar::BigInt;
use substreams::Hex;
use substreams_ethereum::pb::eth::v2::{Block, Call, TransactionTrace, TransactionTraceStatus};
use substreams_ethereum::Event;

const NULL_ADDRESS: [u8; 20] = hex!("0000000000000000000000000000000000000000");
const ZERO_STORAGE_PREFIX: [u8; 16] = hex!("00000000000000000000000000000000");

#[substreams::handlers::map]
pub fn map_balance_changes(block: Block) -> Result<BalanceChanges, Error> {
    Ok(BalanceChanges {
        balance_changes: map_balance_change(block),
    })
}

pub fn map_balance_change(block: Block) -> Vec<BalanceChange> {
    let mut balance_changes = Vec::new();

    for trx in block.transaction_traces.iter() {
        if trx.status != TransactionTraceStatus::Succeeded as i32 {
            continue;
        }

        for call in trx.calls.iter() {
            if call.state_reverted {
                continue;
            }

            for log in call.logs.iter() {
                let transfer = match Transfer::match_and_decode(log) {
                    Some(transfer) => transfer,
                    None => continue,
                };

                if transfer.value.is_zero() {
                    continue;
                }

                if transfer.from == NULL_ADDRESS {
                    continue;
                }

                // Trying with algorithm #1
                let mut found_balance_changes =
                    find_erc20_balance_changes_algorithm1(trx, call, &transfer);
                if !found_balance_changes.is_empty() {
                    balance_changes.extend(found_balance_changes);
                    continue;
                }

                // No balance changes found using algorithm #1, trying with algorithm #2
                found_balance_changes = find_erc20_balance_changes_algorithm2(&transfer, trx);
                if !found_balance_changes.is_empty() {
                    balance_changes.extend(found_balance_changes);
                    continue;
                }

                // No algorithm could extract the balance change, old/new balance is fixed at 0
                balance_changes.push(BalanceChange {
                    contract: Hex::encode(&call.address),
                    owner: Hex::encode(&transfer.to),
                    old_balance: "0".to_string(),
                    new_balance: "0".to_string(),
                    transaction: Hex::encode(&trx.hash),
                    storage_key: "".to_string(),
                    call_index: call.index,
                    transfer_value: transfer.value.to_string(),
                    change_type: BalanceChangeType::TypeUnknown as i32,
                });
            }
        }
    }

    balance_changes
}

/// normal case
fn find_erc20_balance_changes_algorithm1(
    trx: &TransactionTrace,
    call: &Call,
    transfer: &Transfer,
) -> Vec<BalanceChange> {
    let mut out = Vec::new();
    let mut keccak_address_map: Option<StorageKeyToAddressMap> = None;

    for storage_change in &call.storage_changes {
        let old_balance = BigInt::from_signed_bytes_be(&storage_change.old_value);
        let new_balance = BigInt::from_signed_bytes_be(&storage_change.new_value);

        let balance_change = new_balance - old_balance;
        let balance_change_abs = if balance_change < BigInt::zero() {
            balance_change.neg()
        } else {
            balance_change
        };

        let value = transfer.value.clone();
        let transfer_value_abs = if value.clone() < BigInt::zero() {
            value.neg()
        } else {
            value.clone()
        };

        if balance_change_abs != transfer_value_abs {
            info!("Balance change does not match transfer value. Balance change: {}, transfer value: {}", balance_change_abs, transfer_value_abs);
            continue;
        }

        // We memoize the keccak address map by call because it is expensive to compute
        if keccak_address_map.is_none() {
            keccak_address_map = Some(erc20_addresses_for_storage_keys(call));
        }

        let keccak_address = match keccak_address_map
            .as_ref()
            .unwrap()
            .get(&storage_change.key)
        {
            Some(address) => address,
            None => {
                if storage_change.key[0..16] == ZERO_STORAGE_PREFIX {
                    info!("Skipping balance change for zero key");
                    continue;
                }

                info!(
                    "No keccak address found for key: {}, trx {}",
                    Hex(&storage_change.key),
                    Hex(&trx.hash)
                );
                continue;
            }
        };

        if !erc20_is_valid_address(keccak_address, transfer) {
            info!("Keccak address does not match transfer address. Keccak address: {}, sender address: {}, receiver address: {}, trx {}", Hex(keccak_address), Hex(&transfer.from), Hex(&transfer.to), Hex(&trx.hash));
            continue;
        }

        let change = BalanceChange {
            contract: Hex::encode(&call.address),
            owner: Hex::encode(keccak_address),
            old_balance: BigInt::from_signed_bytes_be(&storage_change.old_value).to_string(),
            new_balance: BigInt::from_signed_bytes_be(&storage_change.new_value).to_string(),
            transaction: Hex::encode(&trx.hash),
            storage_key: Hex::encode(&storage_change.key),
            call_index: call.index,
            transfer_value: value.to_string(),
            change_type: BalanceChangeType::Type1 as i32,
        };

        out.push(change);
    }

    out
}

// case where storage changes are not in the same call as the transfer event
fn find_erc20_balance_changes_algorithm2(
    transfer: &Transfer,
    trx: &TransactionTrace,
) -> Vec<BalanceChange> {
    let mut out = Vec::new();

    //get all keccak keys for transfer.to and transfer.from

    let mut keys = HashMap::new();
    for call in trx.calls.iter() {
        let keccak_address_map = erc20_addresses_for_storage_keys(call);

        keys.extend(keccak_address_map);
    }

    for call in trx.calls.iter() {
        for storage_change in &call.storage_changes {
            let keccak_address = match keys.get(&storage_change.key) {
                Some(address) => address,
                None => continue,
            };

            if !erc20_is_valid_address(keccak_address, transfer) {
                continue;
            }

            let change = BalanceChange {
                contract: Hex::encode(&call.address),
                owner: Hex::encode(keccak_address),
                old_balance: BigInt::from_signed_bytes_be(&storage_change.old_value).to_string(),
                new_balance: BigInt::from_signed_bytes_be(&storage_change.new_value).to_string(),
                transaction: Hex::encode(&trx.hash),
                storage_key: Hex::encode(&storage_change.key),
                call_index: call.index,
                transfer_value: transfer.value.to_string(),
                change_type: BalanceChangeType::Type2 as i32,
            };

            out.push(change);
        }
    }

    out
}

type StorageKeyToAddressMap = HashMap<Vec<u8>, Vec<u8>>;

fn erc20_addresses_for_storage_keys(call: &Call) -> StorageKeyToAddressMap {
    let mut out = HashMap::new();

    for (hash, preimage) in &call.keccak_preimages {
        if preimage.len() != 128 {
            continue;
        }

        if &preimage[64..] != "00000000000000000000000000000000000000000000000000000000000000" {
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

fn erc20_is_valid_address(address: &Vec<u8>, transfer: &Transfer) -> bool {
    address == &transfer.from || address == &transfer.to
}
