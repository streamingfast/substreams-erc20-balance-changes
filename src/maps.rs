use crate::abi::{self};
use crate::pb::erc20::types::v1::{BalanceChange, BalanceChangeType, Events, Transfer};
use crate::utils::{clock_to_date, compute_keccak_address_map, erc20_is_valid_address, get_all_child_calls, index_to_version, StorageKeyToAddressMap};
use abi::erc20::events::Transfer as TransferAbi;
use hex_literal::hex;
use substreams::errors::Error;
use substreams::log::info;
use substreams::scalar::BigInt;
use substreams::Hex;
use substreams::pb::substreams::Clock;
use substreams_ethereum::pb::eth::v2::{Block, Call, Log, StorageChange, TransactionTrace, TransactionTraceStatus};
use substreams_ethereum::Event;

// const NULL_ADDRESS: [u8; 20] = hex!("0000000000000000000000000000000000000000");
const ZERO_STORAGE_PREFIX: [u8; 16] = hex!("00000000000000000000000000000000");

#[substreams::handlers::map]
pub fn map_events(clock: Clock, block: Block) -> Result<Events, Error> {
    let transfers = iter_transfers(block);
    let balance_changes = iter_balance_changes(transfers.clone());

    Ok(Events {
        transfers: map_transfers(&clock, transfers),
        balance_changes: map_balance_changes(&clock, balance_changes),
    })
}

pub fn map_transfers(clock: &Clock, transfers: Vec<(TransactionTrace, Call, Log, TransferAbi)>) -> Vec<Transfer> {
    let mut events = Vec::new();

    for (trx, call, log, transfer) in transfers {
        events.push(Transfer {
            // -- block --
            block_num: clock.number,
            block_hash: clock.id.clone(),
            date: clock_to_date(&clock),
            timestamp: clock.timestamp,

            // -- transaction --
            transaction_id: Hex::encode(&trx.hash),

            // -- call --
            call_index: call.index,

            // -- log --
            log_index: log.index,
            log_block_index: log.block_index,
            log_ordinal: log.ordinal,

            // -- transfer --
            contract: Hex::encode(&call.address),
            from: Hex::encode(&transfer.from),
            to: Hex::encode(&transfer.to),
            value: transfer.value.to_string(),
        });
    }
    events
}

pub fn map_balance_changes(clock: &Clock, balance_changes: Vec<(TransactionTrace, Call, Log, TransferAbi, StorageChange, BalanceChangeType)>) -> Vec<BalanceChange> {
    let mut events = Vec::new();
    let mut index = 0; // incrementing index for each balance change

    for (trx, call, log, transfer, storage_change, change_type) in balance_changes {
        events.push(BalanceChange {
            // -- block --
            block_num: clock.number,
            block_hash: clock.id.clone(),
            date: clock_to_date(&clock),
            timestamp: clock.timestamp,

            // -- transaction
            transaction_id: Hex::encode(&trx.hash),

            // -- call --
            call_index: call.index,

            // -- log --
            log_index: log.index,
            log_block_index: log.block_index,
            log_ordinal: log.ordinal,

            // -- storage change --
            storage_key: Hex::encode(&storage_change.key),
            storage_ordinal: storage_change.ordinal,

            // -- indexing --
            index,
            version: index_to_version(&clock, index),

            // -- balance change --
            contract: Hex::encode(&call.address),
            owner: Hex::encode(storage_change.key),
            old_balance: BigInt::from_unsigned_bytes_be(&storage_change.old_value).to_string(),
            new_balance: BigInt::from_unsigned_bytes_be(&storage_change.new_value).to_string(),
            amount: transfer.value.to_string(),
            change_type: change_type as i32,
        });
        index += 1;
    }
    events
}

pub fn iter_transfers(block: Block) -> Vec<(TransactionTrace, Call, Log, TransferAbi)> {
    let mut out = Vec::new();

    for trx in block.transaction_traces.iter() {
        if trx.status != TransactionTraceStatus::Succeeded as i32 {
            continue;
        }
        for call in trx.calls.iter() {
            if call.state_reverted {
                continue;
            }

            for log in call.logs.iter() {
                let transfer = match TransferAbi::match_and_decode(log) {
                    Some(transfer) => transfer,
                    None => continue,
                };
                if transfer.value.is_zero() {
                    continue;
                }
                out.push((trx.clone(), call.clone(), log.clone(), transfer));
            }
        }
    }
    out
}

pub fn iter_balance_changes(transfers: Vec<(TransactionTrace, Call, Log, TransferAbi)>) -> Vec<(TransactionTrace, Call, Log, TransferAbi, StorageChange, BalanceChangeType)> {
    let mut out = Vec::new();

    // We memoize the keccak address map by call because it is expensive to compute
    let calls = transfers.iter().map(|(trx, _, _, _)| trx.calls.clone()).flatten().collect::<Vec<Call>>();
    let keccak_address_map = compute_keccak_address_map(calls);

    for (trx, call, log, transfer) in transfers {
        // algorithm #1 (normal case)
        for storage_changes in find_erc20_balance_changes_algorithm1(&call, &transfer, &keccak_address_map) {
            out.push((trx.clone(), call.clone(), log.clone(), transfer.clone(), storage_changes, BalanceChangeType::BalanceChangeType1));
        }

        // algorithm #2 (case where storage changes are not in the same call as the transfer event)
        for storage_changes in find_erc20_balance_changes_algorithm2(&call, &transfer, &trx, &keccak_address_map) {
            out.push((trx.clone(), call.clone(), log.clone(), transfer.clone(), storage_changes, BalanceChangeType::BalanceChangeType2));
        }
    }
    out
}

// algorithm #1 (normal case)
fn find_erc20_balance_changes_algorithm1(
    call: &Call,
    transfer: &TransferAbi,
    keccak_address_map: &StorageKeyToAddressMap,
) -> Vec<StorageChange> {
    let mut out = Vec::new();

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

        let keccak_address = match keccak_address_map
            .get(&storage_change.key)
        {
            Some(address) => address,
            None => {
                // if storage_change.key[0..16] == ZERO_STORAGE_PREFIX {
                //     info!("Skipping balance change for zero key");
                //     continue;
                // }

                info!(
                    "No keccak address found for key: {}, address {}",
                    Hex(&storage_change.key),
                    Hex(&call.address)
                );
                continue;
            }
        };

        if !erc20_is_valid_address(keccak_address, transfer) {
            info!("Keccak address does not match transfer address. Keccak address: {}, sender address: {}, receiver address: {}", Hex(keccak_address), Hex(&transfer.from), Hex(&transfer.to));
            continue;
        }
        out.push(storage_change.clone());
    }
    out
}


// algorithm #2 (case where storage changes are not in the same call as the transfer event)
fn find_erc20_balance_changes_algorithm2(
    original_call: &Call,
    transfer: &TransferAbi,
    trx: &TransactionTrace,
    keccak_address_map: &StorageKeyToAddressMap,
) -> Vec<StorageChange> {
    let mut out = Vec::new();

    let child_calls = get_all_child_calls(original_call, trx);

    //get all storage changes for these calls:
    let mut storage_changes = Vec::new();
    for call in child_calls.iter() {
        storage_changes.extend(call.storage_changes.clone());
    }

    let mut total_sent = BigInt::zero();
    let mut total_received = BigInt::zero();

    //check if any of the storage changes match the transfer.to or transfer.from
    for storage_change in storage_changes.clone().iter() {
        let keccak_address = match keccak_address_map.get(&storage_change.key) {
            Some(address) => address,
            None => continue,
        };

        if !erc20_is_valid_address(keccak_address, transfer) {
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

        out.push(storage_change.clone());
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
        if keccak_address_map.get(&storage_change.key).is_none() {
            continue;
        };

        let old_balance = BigInt::from_unsigned_bytes_be(&storage_change.old_value);
        let new_balance = BigInt::from_unsigned_bytes_be(&storage_change.new_value);

        let mut balance_change = new_balance - old_balance;
        if balance_change < BigInt::zero() {
            balance_change = balance_change.neg();
        }

        if balance_change != diff {
            continue;
        }

        out.push(storage_change.clone());
    }

    out
}