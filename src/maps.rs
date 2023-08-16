use std::collections::HashMap;
use std::ops::Div;
use crate::abi::{self};
use crate::pb::erc20::types::v1::{BalanceChange, BalanceChanges, ValidBalanceChangeStats};
use abi::erc20::{
    events::{Transfer},
};
use hex;
use substreams::errors::Error;
use substreams::Hex;
use substreams::scalar::{BigDecimal, BigInt};
use substreams::store::{StoreGet, StoreGetBigInt};
use substreams_ethereum::pb::eth::v2::{Block, Call, TransactionTrace};
use substreams_ethereum::Event;
use num_traits::cast::ToPrimitive;
use substreams::log::info;
use substreams::pb::substreams::Clock;

#[substreams::handlers::map]
pub fn map_balance_changes(block: Block) -> Result<BalanceChanges, Error> {
    let balance_changes = map_balance_change(block);

    Ok(BalanceChanges {
        balance_changes
    })
}

#[substreams::handlers::map]
pub fn map_valid_changes(clock: Clock, store: StoreGetBigInt) -> Result<ValidBalanceChangeStats, Error> {
    let valid = match store.get_last("valid") {
        Some(valid) => valid,
        None => BigInt::from(0),
    }.to_decimal(1);

    let total = match store.get_last("total") {
        Some(total) => total,
        None => BigInt::from(0),
    }.to_decimal(1);

    Ok(ValidBalanceChangeStats {
        valid_balance_change_count: valid.to_f64().unwrap_or(0.0) as f32,
        total_balance_change_count: total.to_f64().unwrap_or(0.0) as f32,
        valid_ratio: if total.is_zero() {
            0.0
        } else {
            let valid_ratio: BigDecimal = valid / total;
            valid_ratio.to_f64().unwrap_or(0.0) as f32
        },
        block_number: clock.number,
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

                let changes = find_erc20_balance_changes(trx, call, &transfer);
                if changes.is_empty() {
                    info!("No balance changes found for transfer: from {:?}, to {:?}, trx {:?}",
                        Hex::encode(&transfer.from).to_string(),
                        Hex::encode(&transfer.to).to_string(),
                        Hex::encode(&trx.hash).to_string(),
                    );

                    let dummy_change = BalanceChange {
                        contract: Hex::encode(&call.address).to_string(),
                        owner: Hex::encode(&transfer.from).to_string(),
                        old_balance: BigInt::from(0).to_string(),
                        new_balance: BigInt::from(0).to_string(),
                        transaction: Hex::encode(&trx.hash).to_string(),
                        storage_key: "".to_string(),
                        call_index: call.index.to_string(),
                        transfer_value: transfer.value.to_string(),
                        is_valid: false,
                    };
                }

                balance_changes.extend(changes);
            }
        }
    }

    balance_changes
}

fn find_erc20_balance_changes(trx: &TransactionTrace, call: &Call, transfer: &Transfer) -> Vec<BalanceChange> {
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
            is_valid: true,
        };

        out.push(change);
    }

    out
}

fn erc20_is_valid_address(address: Vec<u8>, transfer: &Transfer) -> bool {
    let address_as_hex = hex::encode(&address);
    let transfer_from_as_hex = hex::encode(&transfer.from);
    let transfer_to_as_hex = hex::encode(&transfer.to);

    address_as_hex == transfer_from_as_hex || address_as_hex == transfer_to_as_hex
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
