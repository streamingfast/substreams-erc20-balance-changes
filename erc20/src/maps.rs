use std::collections::HashMap;

use crate::algorithms::algorithm1_call::find_erc20_balance_changes_algorithm1;
use crate::algorithms::algorithm2_child_calls::find_erc20_balance_changes_algorithm2;
use crate::algorithms::fishing::is_fishing_transfers;
use crate::algorithms::utils::{addresses_for_storage_keys, Address, Hash};
use proto::pb::evm::tokens::types::v1::balance_change::{Reason as BalanceChangeReason, Algorithm as BalanceChangeAlgorithm};
use proto::pb::evm::tokens::types::v1::{BalanceChange, Events, TokenType, Transfer};
use crate::utils::{clock_to_date, to_global_sequence};
use substreams::errors::Error;
use substreams_abis::evm::token::erc20::events::Transfer as TransferAbi;

use substreams::pb::substreams::Clock;
use substreams::scalar::BigInt;
use substreams::Hex;
use substreams_ethereum::pb::eth::v2::{Block, Call, Log, StorageChange, TransactionTrace};
use substreams_ethereum::Event;

#[substreams::handlers::map]
pub fn map_events(clock: Clock, block: Block) -> Result<Events, Error> {
    let mut events = Events::default();
    insert_events(&clock, &block, &mut events);
    Ok(events)
}

pub fn to_transfer<'a>(clock: &'a Clock, trx: &'a TransactionTrace, log: &'a Log, transfer: &'a TransferAbi, index: &u64) -> Transfer {
    Transfer {
        // -- block --
        block_num: clock.number,
        block_hash: clock.id.clone(),
        date: clock_to_date(clock),
        timestamp: clock.timestamp,

        // -- transaction --
        transaction_id: Hex::encode(&trx.hash),

        // -- ordering --
        ordinal: log.ordinal,
        global_sequence: to_global_sequence(clock, index),

        // -- transfer --
        contract: Hex::encode(&log.address),
        from: Hex::encode(&transfer.from),
        to: Hex::encode(&transfer.to),
        value: transfer.value.to_string(),

        // -- metadata --
        r#type: TokenType::Erc20.into(),
    }
}

pub fn to_balance_change<'a>(
    clock: &Clock,
    trx: &'a TransactionTrace,
    owner: Address,
    storage_change: &'a StorageChange,
    algorithm: BalanceChangeAlgorithm,
    index: &u64,
) -> BalanceChange {
    let old_balance = BigInt::from_unsigned_bytes_be(&storage_change.old_value);
    let new_balance = BigInt::from_unsigned_bytes_be(&storage_change.new_value);

    BalanceChange {
        // -- block --
        block_num: clock.number,
        block_hash: clock.id.clone(),
        date: clock_to_date(clock),
        timestamp: clock.timestamp,

        // -- transaction
        transaction_id: Hex::encode(&trx.hash),

        // -- balance change --
        contract: Hex::encode(&storage_change.address),
        owner: Hex::encode(owner),
        old_balance: old_balance.to_string(),
        new_balance: new_balance.to_string(),

        // -- ordering --
        ordinal: storage_change.ordinal,
        global_sequence: to_global_sequence(clock, index),

        // -- metadata --
        r#type: TokenType::Erc20.into(),
        reason: BalanceChangeReason::Transfer.into(),
        algorithm: algorithm.into(),
    }
}

pub fn insert_events<'a>(clock: &'a Clock, block: &'a Block, events: &mut Events) {
    // We memoize the keccak address map by call because it is expensive to compute
    let mut keccak_address_map = HashMap::new();
    let mut transfer_index = 0;
    let mut balance_changes_index = 0;

    // Iterates over successful transactions
    for trx in block.transactions() {
        // Iterates over all logs in the transaction
        // excluding those from calls that were not recorded to the chain's state.
        for (log, call_view) in trx.logs_with_calls() {
            let call = call_view.as_ref();

            // -- Transfer --
            if is_fishing_transfers(trx, call) {
                continue;
            }
            let transfer = match TransferAbi::match_and_decode(log) {
                Some(transfer) => transfer,
                None => continue,
            };
            if transfer.value.is_zero() {
                continue;
            }
            events.transfers.push(to_transfer(clock, trx, log, &transfer, &transfer_index));
            transfer_index += 1;

            // -- Balance Changes --
            keccak_address_map.extend(addresses_for_storage_keys(call)); // memoize
            let balance_changes = iter_balance_changes_algorithms(trx, call, &transfer, &keccak_address_map);
            for (owner, storage_change, change_type) in balance_changes {
                let balance_change = to_balance_change(clock, trx, owner, storage_change, change_type, &balance_changes_index);
                balance_changes_index += 1;

                // insert balance change event
                events.balance_changes.push(balance_change);
            }
        }
    }
}

pub fn iter_balance_changes_algorithms<'a>(
    trx: &'a TransactionTrace,
    call: &'a Call,
    transfer: &'a TransferAbi,
    keccak_address_map: &'a HashMap<Hash, Address>,
) -> Vec<(Address, &'a StorageChange, BalanceChangeAlgorithm)> {
    let mut out = Vec::new();

    // algorithm #1 (normal case)
    for (owner, storage_changes, change_type) in find_erc20_balance_changes_algorithm1(call, transfer, keccak_address_map) {
        out.push((owner, storage_changes, change_type));
    }

    // algorithm #2 (case where storage changes are not in the same call as the transfer event)
    for (owner, storage_changes, change_type) in find_erc20_balance_changes_algorithm2(trx, call, transfer, keccak_address_map) {
        out.push((owner, storage_changes, change_type));
    }
    out
}
