use crate::abi::{self};
use crate::pb::erc20::types::v1::{BalanceChange, BalanceChanges};
use abi::erc20::{
    events::{Transfer},
};
use hex;
use substreams::errors::Error;
use substreams::Hex;
use substreams::scalar::BigInt;
use substreams_ethereum::pb::eth::v2::{Block, Call, TransactionTrace};
use substreams_ethereum::Event;

#[substreams::handlers::map]
pub fn map_balance_changes(block: Block) -> Result<BalanceChanges, Error> {
    let balance_changes = map_balance_change(block.clone());

    Ok(BalanceChanges {
        balance_changes
    })
}

pub fn map_balance_change(block: Block) -> Vec<BalanceChange> {
    let mut balance_changes = Vec::new();

    for tr in block.transaction_traces.iter() {
        if tr.status == 2 || tr.status == 3 { // failed or reverted
            continue;
        }

        for call in tr.calls.iter() {
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


                let balance_from = find_erc20_balance_changes(tr, call,  &transfer.from, transfer.value.clone());
                balance_changes.extend(balance_from);

                let balance_to = find_erc20_balance_changes(tr, call, &transfer.to, transfer.value.clone());
                balance_changes.extend(balance_to);
            }
        }
    }

    balance_changes
}


fn find_erc20_balance_changes(tr: &TransactionTrace, call: &Call, holder: &[u8], value: BigInt) -> Vec<BalanceChange> {
    let mut out = Vec::new();

    let storage_keys = erc20_storage_keys_for_address(call, holder);
    if storage_keys.is_empty() {
        return out;
    }

    for storage_key in storage_keys {
        let storage_key_bytes = hex::decode(&storage_key).expect("Failed to decode hex string");
        for storage_change in &call.storage_changes {
            if value.is_zero() {
                continue;
            }

            if storage_change.key != storage_key_bytes {
                continue;
            }

            if call.input.len() < 4 {
                continue;
            }

            let old_balance = BigInt::from_signed_bytes_be(&storage_change.old_value);
            let new_balance = BigInt::from_signed_bytes_be(&storage_change.new_value);

            // get absolute value of balance change
            let balance_change = new_balance.clone() - old_balance.clone();
            let balance_change_abs = if balance_change < BigInt::zero() {
                balance_change * BigInt::from(-1)
            } else {
                balance_change
            };

            if balance_change_abs != value.clone() {
                continue;
            }

            let change = BalanceChange {
                contract: Hex::encode(&call.address).to_string(),
                owner: Hex::encode(&holder).to_string(),
                old_balance: BigInt::from_signed_bytes_be(&storage_change.old_value).to_string(),
                new_balance: BigInt::from_signed_bytes_be(&storage_change.new_value).to_string(),
                transaction: Hex::encode(&tr.hash).to_string(),
                storage_key: Hex::encode(&storage_change.key).to_string(),
                call_index: call.index.to_string(),
                transfer_value: value.to_string(),
            };

            out.push(change);
        }
    }
    out
}

fn erc20_storage_keys_for_address(call: &Call, address: &[u8]) -> Vec<String> {
    let mut out = Vec::new();
    let addr_as_hex = hex::encode(address);

    for (hash, preimage) in &call.keccak_preimages {
        if preimage.len() != 128 {
            continue;
        }

        if &preimage[64..126] != "00000000000000000000000000000000000000000000000000000000000000" {
            continue;
        }

        if &preimage[24..64] == addr_as_hex {
            out.push(hash.clone());
        }
    }

    out
}
