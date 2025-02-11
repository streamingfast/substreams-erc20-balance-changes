use crate::abi::{self};
use crate::pb::erc20::types::v1::{BalanceChange, BalanceChangeType, Events, Transfer};
use crate::utils::{clock_to_date, compute_keccak_address_map, get_all_child_calls, get_owner_from_keccak_address_map, index_to_version, is_erc20_valid_address, is_erc20_valid_balance, StorageKeyToAddressMap};
use abi::erc20::events::Transfer as TransferAbi;
use substreams::errors::Error;
use substreams::log::info;
use substreams::scalar::BigInt;
use substreams::Hex;
use substreams::pb::substreams::Clock;
use substreams_ethereum::pb::eth::v2::{Block, Call, Log, StorageChange, TransactionTrace, TransactionTraceStatus};
use substreams_ethereum::Event;

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
        let child_calls = get_all_child_calls(&call, &trx);
        for storage_changes in find_erc20_balance_changes_algorithm2(child_calls, &transfer, &keccak_address_map) {
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
        // Check if the transfer matches the storage change balance changes
        if !is_erc20_valid_balance(transfer, storage_change) {
            continue;
        }

        // extract the owner address
        let owner = match get_owner_from_keccak_address_map(keccak_address_map, &storage_change) {
            Some(address) => address,
            None => continue
        };

        // make sure owner is either the sender or receiver
        if !is_erc20_valid_address(&owner, transfer) {
            info!("owner={} does not match transfer from={} to={}", Hex(owner), Hex(&transfer.from), Hex(&transfer.to));
            continue;
        }
        out.push(storage_change.clone());
    }
    out
}

// algorithm #2 (case where storage changes are not in the same call as the transfer event)
fn find_erc20_balance_changes_algorithm2(
    child_calls: Vec<Call>,
    transfer: &TransferAbi,
    keccak_address_map: &StorageKeyToAddressMap,
) -> Vec<StorageChange> {
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
            info!("owner={} does not match transfer from={} to={}", Hex(owner), Hex(&transfer.from), Hex(&transfer.to));
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

        // Check if the transfer matches the storage change balance changes
        let owner = match get_owner_from_keccak_address_map(keccak_address_map, &storage_change) {
            Some(address) => address,
            None => continue
        };

        // make sure owner is either the sender or receiver
        if !is_erc20_valid_address(&owner, transfer) {
            info!("owner={} does not match transfer from={} to={}", Hex(owner), Hex(&transfer.from), Hex(&transfer.to));
            continue;
        }

        let old_balance = BigInt::from_unsigned_bytes_be(&storage_change.old_value);
        let new_balance = BigInt::from_unsigned_bytes_be(&storage_change.new_value);

        let mut balance_change = new_balance - old_balance;
        if balance_change < BigInt::zero() {
            balance_change = balance_change.neg();
        }

        if balance_change != diff {
            info!("Algo2: Balance change does not match transfer value. Balance change: {}, transfer value: {}", balance_change, transfer.value);
            continue;
        }

        out.push(storage_change.clone());
    }

    out
}