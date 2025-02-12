use crate::abi::{self};
use crate::algorithms::{compute_keccak_address_map, find_erc20_balance_changes_algorithm1, find_erc20_balance_changes_algorithm2, get_all_child_calls};
use crate::pb::erc20::types::v1::{BalanceChange, BalanceChangeType, Events, Transfer};
use crate::utils::{clock_to_date, index_to_version};
use abi::erc20::events::Transfer as TransferAbi;
use substreams::errors::Error;

use substreams::pb::substreams::Clock;
use substreams::scalar::BigInt;
use substreams::Hex;
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
            data: Hex::encode(&log.data),
            topic0: Hex::encode(&log.topics[0]),

            // -- transfer --
            contract: Hex::encode(&call.address),
            from: Hex::encode(&transfer.from),
            to: Hex::encode(&transfer.to),
            value: transfer.value.to_string(),
        });
    }
    events
}

pub fn map_balance_changes(clock: &Clock, balance_changes: Vec<(TransactionTrace, Call, Log, TransferAbi, Vec<u8>, StorageChange, BalanceChangeType)>) -> Vec<BalanceChange> {
    let mut events = Vec::new();
    let mut index = 0; // incrementing index for each balance change

    for (trx, call, log, transfer, owner, storage_change, change_type) in balance_changes {
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
            storage_address: Hex::encode(&storage_change.address),

            // -- indexing --
            index,
            version: index_to_version(&clock, index),

            // -- balance change --
            contract: Hex::encode(&call.address),
            owner: Hex::encode(owner),
            old_balance: BigInt::from_unsigned_bytes_be(&storage_change.old_value).to_string(),
            new_balance: BigInt::from_unsigned_bytes_be(&storage_change.new_value).to_string(),

            // -- transfer --
            from: Hex::encode(&transfer.from),
            to: Hex::encode(&transfer.to),
            value: transfer.value.to_string(),

            // -- debug --
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

pub fn iter_balance_changes(transfers: Vec<(TransactionTrace, Call, Log, TransferAbi)>) -> Vec<(TransactionTrace, Call, Log, TransferAbi, Vec<u8>, StorageChange, BalanceChangeType)> {
    let mut out = Vec::new();

    // We memoize the keccak address map by call because it is expensive to compute
    let calls = transfers.iter().map(|(trx, _, _, _)| trx.calls.clone()).flatten().collect::<Vec<Call>>();
    let keccak_address_map = compute_keccak_address_map(calls);

    for (trx, call, log, transfer) in transfers {
        // algorithm #1 (normal case)
        for (owner, storage_changes) in find_erc20_balance_changes_algorithm1(&call, &transfer, &keccak_address_map) {
            out.push((trx.clone(), call.clone(), log.clone(), transfer.clone(), owner, storage_changes, BalanceChangeType::BalanceChangeType1));
        }

        // algorithm #2 (case where storage changes are not in the same call as the transfer event)
        let child_calls = get_all_child_calls(&call, &trx);
        for (owner, storage_changes) in find_erc20_balance_changes_algorithm2(child_calls, &transfer, &keccak_address_map) {
            out.push((trx.clone(), call.clone(), log.clone(), transfer.clone(), owner, storage_changes, BalanceChangeType::BalanceChangeType2));
        }
    }
    out
}
