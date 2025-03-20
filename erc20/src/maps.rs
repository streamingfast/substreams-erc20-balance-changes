use std::collections::HashMap;

use crate::algorithms::algorithm1_call::get_owner_from_erc20_balance_change;
use crate::algorithms::algorithm2_child_calls::get_all_child_call_storage_changes;
use crate::algorithms::transfers::get_erc20_transfer;
use crate::algorithms::utils::addresses_for_storage_keys;
use common::{extend_from_address, to_global_sequence, Address, Hash};
use proto::pb::evm::tokens::balances::types::v1::{Algorithm, BalanceChange, Events, Transfer};
use substreams::errors::Error;
use substreams::log;
use substreams_abis::evm::token::erc20::events::Transfer as TransferAbi;

use substreams::pb::substreams::Clock;
use substreams::scalar::BigInt;
use substreams_ethereum::pb::eth::v2::{Block, Call, Log, StorageChange, TransactionTrace};

#[substreams::handlers::map]
pub fn map_events(clock: Clock, block: Block) -> Result<Events, Error> {
    let mut events = Events::default();
    insert_events(&clock, &block, &mut events);
    log::info!(format!(
        "transfers: {} balance_changes: {}",
        events.transfers.len(),
        events.balance_changes.len()
    ));
    Ok(events)
}

pub fn to_transfer<'a>(clock: &'a Clock, trx: &'a TransactionTrace, log: &'a Log, transfer: &'a TransferAbi, algorithm: Algorithm, index: &u64) -> Transfer {
    Transfer {
        // -- transaction --
        transaction_id: trx.hash.to_vec(),

        // -- ordering --
        ordinal: log.ordinal,
        index: *index,
        global_sequence: to_global_sequence(clock, index),

        // -- transfer --
        contract: log.address.to_vec(),
        from: transfer.from.to_vec(),
        to: transfer.to.to_vec(),
        value: transfer.value.to_string(),

        // -- debug --
        algorithm: algorithm.into(),
    }
}

pub fn to_balance_change<'a>(
    clock: &Clock,
    trx: &'a TransactionTrace,
    owner: Address,
    storage_change: &'a StorageChange,
    algorithm: Algorithm,
    index: &u64,
) -> BalanceChange {
    let old_balance = BigInt::from_unsigned_bytes_be(&storage_change.old_value);
    let new_balance = BigInt::from_unsigned_bytes_be(&storage_change.new_value);

    BalanceChange {
        // -- transaction
        transaction_id: trx.hash.to_vec(),

        // -- ordering --
        ordinal: storage_change.ordinal,
        index: *index,
        global_sequence: to_global_sequence(clock, index),

        // -- balance change --
        contract: storage_change.address.to_vec(),
        owner,
        old_balance: old_balance.to_string(),
        new_balance: new_balance.to_string(),

        // -- debug --
        algorithm: algorithm.into(),
    }
}

pub fn insert_events<'a>(clock: &'a Clock, block: &'a Block, events: &mut Events) {
    // We memoize the keccak address map by call because it is expensive to compute
    // Pre-allocate HashMaps to avoid initial reallocations
    let mut keccak_address_map = HashMap::with_capacity(block.transactions().count() * 2);
    let mut last_balance_changes: HashMap<Vec<u8>, BalanceChange> = HashMap::with_capacity(block.transactions().count() * 2);
    let mut index = 0; // relative index for ordering

    // Iterates over successful transactions
    for trx in block.transactions() {
        // Iterates over all logs in the transaction
        // excluding those from calls that were not recorded to the chain's state.
        for (log, call_view) in trx.logs_with_calls() {
            let call = call_view.as_ref();

            // -- Transfer --
            let transfer = match get_erc20_transfer(trx, call, log) {
                Some(transfer) => transfer,
                None => continue,
            };
            events.transfers.push(to_transfer(clock, trx, log, &transfer, Algorithm::Log, &index));
            index += 1;

            // -- Balance Changes --
            keccak_address_map.extend(addresses_for_storage_keys(call)); // memoize
            let balance_changes = iter_balance_changes_algorithms(trx, call, &transfer, &keccak_address_map);
            for (owner, storage_change, change_type) in balance_changes {
                let balance_change = to_balance_change(clock, trx, owner, storage_change, change_type, &index);
                let key = extend_from_address(&balance_change.contract, &balance_change.owner);

                // overwrite balance change if it already exists
                last_balance_changes.insert(key, balance_change);
                index += 1;
            }
        }
    }

    // Reserve capacity for the balance changes to avoid reallocations
    events.balance_changes.reserve(last_balance_changes.len());

    // insert only the last balance change for each contract + owner per block
    events.balance_changes.extend(last_balance_changes.into_values());
}

pub fn iter_balance_changes_algorithms<'a>(
    trx: &'a TransactionTrace,
    call: &'a Call,
    transfer: &'a TransferAbi,
    keccak_address_map: &'a HashMap<Hash, Address>,
) -> impl Iterator<Item = (Address, &'a StorageChange, Algorithm)> + 'a {
    // First iterator - algorithm #1 (normal case)
    let normal_changes = call.storage_changes.iter().filter_map(move |storage_change| {
        get_owner_from_erc20_balance_change(transfer, storage_change, keccak_address_map).map(|owner| (owner, storage_change, Algorithm::Call))
    });

    // Second iterator - algorithm #2 (child calls)
    let child_changes = get_all_child_call_storage_changes(call, trx).filter_map(move |storage_change| {
        get_owner_from_erc20_balance_change(transfer, storage_change, keccak_address_map).map(|owner| (owner, storage_change, Algorithm::ChildCalls))
    });

    normal_changes.chain(child_changes)
}
