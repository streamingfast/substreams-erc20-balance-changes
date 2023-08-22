use std::collections::HashMap;
use crate::abi::{self};
use crate::pb::erc20::types::v1::{BalanceChange, BalanceChanges, BalanceChangeType};
use abi::erc20::{
    events::{Transfer},
};
use hex;
use substreams::errors::Error;
use substreams::Hex;
use substreams::scalar::{BigInt};
use substreams_ethereum::pb::eth::v2::{Block, Call, TransactionTrace};
use substreams_ethereum::Event;
use substreams::log::info;

#[substreams::handlers::map]
pub fn map_balance_changes(block: Block) -> Result<BalanceChanges, Error> {
    let balance_changes = map_balance_change(block);

    Ok(BalanceChanges {
        balance_changes
    })
}

pub fn map_balance_change(block: Block) -> Vec<BalanceChange> {
    let mut balance_changes = Vec::new();

    for trx in block.transaction_traces.iter() {
        if trx.status == 2 || trx.status == 3 { // failed or reverted
            continue;
        }

        for call in trx.calls.iter() {
            if call.status_failed || call.status_reverted { // failed or reverted
                continue;
            }

            for log in call.logs.iter() {
                let transfer = match Transfer::match_and_decode(log.as_ref()) {
                    Some(transfer) => transfer,
                    None => continue,
                };

                if transfer.value == BigInt::zero() {
                    continue;
                }

                if &Hex::encode(&transfer.from) == "0000000000000000000000000000000000000000000000000000000000000000" {
                    continue;
                }

                let type0_balance_changes = find_erc20_balance_changes_type_good(trx, call, &transfer);
                if !type0_balance_changes.is_empty() {
                    balance_changes.extend(type0_balance_changes);
                    continue;
                } else {
                    let type1_balance_changes = find_erc20_balance_changes_type1(&transfer, trx);
                    if !type1_balance_changes.is_empty() {
                        balance_changes.extend(type1_balance_changes);
                        continue;
                    } else {
                        let invalid_change = BalanceChange {
                            contract: Hex::encode(&call.address).to_string(),
                            owner: Hex::encode(&transfer.to).to_string(),
                            old_balance: BigInt::from(0).to_string(),
                            new_balance: BigInt::from(0).to_string(),
                            transaction: Hex::encode(&trx.hash).to_string(),
                            storage_key: "".to_string(),
                            call_index: call.index.to_string(),
                            transfer_value: transfer.value.to_string(),
                            change_type: BalanceChangeType::TypeUnknown as i32,
                        };
                        balance_changes.push(invalid_change);
                    }
                }
            }
        }
    }

    balance_changes
}

/// normal case
fn find_erc20_balance_changes_type_good(trx: &TransactionTrace, call: &Call, transfer: &Transfer) -> Vec<BalanceChange> {
    let mut out = Vec::new();

    for storage_change in &call.storage_changes {
        let key = Hex::encode(&storage_change.key).to_string();

        let old_balance = BigInt::from_signed_bytes_be(&storage_change.old_value);
        let new_balance = BigInt::from_signed_bytes_be(&storage_change.new_value);

        let balance_change = new_balance - old_balance;
        let balance_change_abs = if balance_change < BigInt::zero() {
            balance_change * BigInt::from(-1)
        } else {
            balance_change
        };

        let value = transfer.value.clone();
        let transfer_value_abs = if value.clone() < BigInt::zero() {
            value.clone() * BigInt::from(-1)
        } else {
            value.clone()
        };

        if balance_change_abs != transfer_value_abs {
            info!("Balance change does not match transfer value. Balance change: {}, transfer value: {}", balance_change_abs, transfer_value_abs);
            continue;
        }

        let keccak_address_map = erc20_addresses_for_storage_keys(call);
        let keccak_opt = keccak_address_map.get(key.as_str());
        if keccak_opt.is_none() {
            if &Hex::encode(&storage_change.key).to_string()[0..32] == "00000000000000000000000000000000" {
                info!("Skipping balance change for zero key");
                continue;
            }

            info!("No keccak address found for key: {}, trx {}", key, Hex::encode(&trx.hash).to_string());
            continue;
        }

        let keccak_address = keccak_opt.unwrap();

        if !erc20_is_valid_address(keccak_address.clone(), transfer) {
            info!("Keccak address does not match transfer address. Keccak address: {}, sender address: {}, receiver address: {}, trx {}", Hex::encode(keccak_address), Hex::encode(&transfer.from), Hex::encode(&transfer.to), Hex::encode(&trx.hash).to_string());
            continue;
        }

        let change = BalanceChange {
            contract: Hex::encode(&call.address).to_string(),
            owner: Hex::encode(keccak_address).to_string(),
            old_balance: BigInt::from_signed_bytes_be(&storage_change.old_value).to_string(),
            new_balance: BigInt::from_signed_bytes_be(&storage_change.new_value).to_string(),
            transaction: Hex::encode(&trx.hash).to_string(),
            storage_key: Hex::encode(&storage_change.key).to_string(),
            call_index: call.index.to_string(),
            transfer_value: value.to_string(),
            change_type: BalanceChangeType::Type1 as i32,
        };

        out.push(change);
    }

    out
}


// case where storage changes are not in the same call as the transfer event
fn find_erc20_balance_changes_type1(transfer: &Transfer, trx: &TransactionTrace) -> Vec<BalanceChange> {
    let mut out = Vec::new();

    //get all keccak keys for transfer.to and transfer.from

    let mut keys = HashMap::new();
    for call in trx.calls.iter() {
        let keccak_address_map = erc20_addresses_for_storage_keys(call);
        for (hash, address) in keccak_address_map {
            keys.insert(hash, address);
        }
    }

    for call in trx.calls.iter() {
        for storage_change in &call.storage_changes {
            let key = Hex::encode(&storage_change.key).to_string();
            let keccak_opt = keys.get(key.as_str());
            if keccak_opt.is_none() {
                continue;
            }

            let keccak_address = keccak_opt.unwrap();

            if !erc20_is_valid_address(keccak_address.clone(), transfer) {
                continue;
            }

            let change = BalanceChange {
                contract: Hex::encode(&call.address).to_string(),
                owner: Hex::encode(keccak_address).to_string(),
                old_balance: BigInt::from_signed_bytes_be(&storage_change.old_value).to_string(),
                new_balance: BigInt::from_signed_bytes_be(&storage_change.new_value).to_string(),
                transaction: Hex::encode(&trx.hash).to_string(),
                storage_key: Hex::encode(&storage_change.key).to_string(),
                call_index: call.index.to_string(),
                transfer_value: transfer.value.to_string(),
                change_type: BalanceChangeType::Type2 as i32,
            };

            out.push(change);
        }
    }

    out
}

fn erc20_addresses_for_storage_keys(call: &Call) -> HashMap<String, Vec<u8>> {
    let mut out = HashMap::new();

    for (hash, preimage) in &call.keccak_preimages {
        if preimage.len() != 128 {
            continue;
        }

        if &preimage[64..126] != "00000000000000000000000000000000000000000000000000000000000000" {
            continue;
        }

        let addr = &preimage[24..64];
        out.insert(hash.clone(), hex::decode(addr).expect("Failed to decode hex string"));
    }

    out
}

fn erc20_is_valid_address(address: Vec<u8>, transfer: &Transfer) -> bool {
    let address_as_hex = hex::encode(&address);
    let transfer_from_as_hex = hex::encode(&transfer.from);
    let transfer_to_as_hex = hex::encode(&transfer.to);

    address_as_hex == transfer_from_as_hex || address_as_hex == transfer_to_as_hex
}


