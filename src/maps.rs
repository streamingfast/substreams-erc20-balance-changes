use crate::abi::{self};
use crate::algorithms::{addresses_for_storage_keys, find_erc20_balance_changes_algorithm1, find_erc20_balance_changes_algorithm2, get_all_child_calls, StorageKeyToAddressMap};
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
    let transfers = iter_transfers(&block);
    let balance_changes = iter_balance_changes(&block, transfers.clone());

    Ok(Events {
        transfers: map_transfers(&clock, &block, transfers),
        balance_changes: map_balance_changes(&clock, &block, balance_changes),
    })
}

pub fn map_transfers(clock: &Clock, block: &Block, transfers: Vec<(u32, Call, Log, TransferAbi)>) -> Vec<Transfer> {
    let mut events = Vec::new();

    for (trx_index, call, log, transfer) in transfers {
        let trx = block.transaction_traces.get(trx_index as usize).unwrap();

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

pub fn map_balance_changes(clock: &Clock, block: &Block, balance_changes: Vec<(u32, Call, Log, TransferAbi, Vec<u8>, StorageChange, BalanceChangeType)>) -> Vec<BalanceChange> {
    let mut events = Vec::new();
    let mut index = 0; // incrementing index for each balance change

    for (trx_index, call, log, transfer, owner, storage_change, change_type) in balance_changes {
        let trx = block.transaction_traces.get(trx_index as usize).unwrap();
        let old_balance = BigInt::from_unsigned_bytes_be(&storage_change.old_value);
        let new_balance = BigInt::from_unsigned_bytes_be(&storage_change.new_value);
        let amount = new_balance.clone() - old_balance.clone();

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
            old_balance: old_balance.to_string(),
            new_balance: new_balance.to_string(),
            amount: amount.to_string(),

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

pub fn iter_transfers(block: &Block) -> Vec<(u32, Call, Log, TransferAbi)> {
    let mut out = Vec::new();

    // Iterates over successful transactions
    let mut trx_index = 0;
    for trx in block.transaction_traces.iter() {
        if trx.status != TransactionTraceStatus::Succeeded as i32 {
            trx_index += 1;
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
                out.push((trx_index, call.clone(), log.clone(), transfer));
            }
        }
        trx_index += 1;
    }
    out
}

pub fn iter_balance_changes(block: &Block, transfers: Vec<(u32, Call, Log, TransferAbi)>) -> Vec<(u32, Call, Log, TransferAbi, Vec<u8>, StorageChange, BalanceChangeType)> {
    let mut out = Vec::new();

    let mut keccak_address_map = StorageKeyToAddressMap::new();
    for (trx_index, call, log, transfer) in transfers {
        let trx = block.transaction_traces.get(trx_index as usize).unwrap();

        // We memoize the keccak address map by call because it is expensive to compute
        keccak_address_map.extend(addresses_for_storage_keys(&call));

        // algorithm #1 (normal case)
        for (owner, storage_changes) in find_erc20_balance_changes_algorithm1(&call, &transfer, &keccak_address_map) {
            out.push((trx_index, call.clone(), log.clone(), transfer.clone(), owner, storage_changes, BalanceChangeType::BalanceChangeType1));
        }

        // algorithm #2 (case where storage changes are not in the same call as the transfer event)
        let child_calls = get_all_child_calls(&call, &trx);
        for (owner, storage_changes) in find_erc20_balance_changes_algorithm2(child_calls, &transfer, &keccak_address_map) {
            out.push((trx_index, call.clone(), log.clone(), transfer.clone(), owner, storage_changes, BalanceChangeType::BalanceChangeType2));
        }
    }
    out
}
