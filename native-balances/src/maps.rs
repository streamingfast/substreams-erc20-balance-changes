use common::{to_global_sequence, to_optional_vector, Address, NATIVE_ADDRESS};
use proto::pb::evm::tokens::algorithm::v1::Algorithm;
use proto::pb::evm::tokens::erc20::balances::v1::{BalanceChange, Events, Transfer};
use substreams::{errors::Error, scalar::BigInt};

use substreams::pb::substreams::Clock;
use substreams_ethereum::pb::eth::v2::{BalanceChange as BalanceChangeAbi, Block, Call, TransactionTrace};

use crate::algorithms::transfers::{get_transfer_from_call, get_transfer_from_transaction};
use crate::utils::{get_balances, is_failed_transaction, is_gas_balance_change};

#[substreams::handlers::map]
pub fn map_events(clock: Clock, block: Block) -> Result<Events, Error> {
    // Pre-allocate vectors to avoid reallocations
    let transaction_count = block.transactions().count();
    let mut events = Events {
        transfers: Vec::with_capacity(transaction_count * 2),
        balance_changes: Vec::with_capacity(transaction_count * 4),
    };
    insert_events(&clock, &block, &mut events);
    Ok(events)
}

pub fn to_balance_change<'a>(
    clock: &Clock,
    trx: &'a TransactionTrace,
    call: &'a Call,
    balance_change: &'a BalanceChangeAbi,
    algorithm: Algorithm,
    index: u64,
) -> BalanceChange {
    let (old_balance, new_balance) = get_balances(balance_change);

    BalanceChange {
        // -- transaction --
        transaction_id: to_optional_vector(&trx.hash),

        // -- call --
        caller: to_optional_vector(&call.caller),

        // -- ordering --
        ordinal: balance_change.ordinal,
        index,
        global_sequence: to_global_sequence(clock, index),

        // -- balance change --
        contract: NATIVE_ADDRESS.to_vec(),
        address: balance_change.address.to_vec(),
        old_balance: old_balance.to_string(),
        new_balance: new_balance.to_string(),

        // -- debug --
        algorithm: algorithm.into(),
        reason: balance_change.reason().as_str_name().to_string()
    }
}

// add default
#[derive(Default)]
pub struct TransferStruct {
    pub from: Address,
    pub to: Address,
    pub value: BigInt,
    pub ordinal: u64,
    pub algorithm: Algorithm,
}

pub fn to_transfer<'a>(clock: &'a Clock, trx: &'a TransactionTrace, call: &'a Call, transfer: TransferStruct, index: u64) -> Transfer {
    Transfer {
        // -- transaction --
        transaction_id: to_optional_vector(&trx.hash),

        // -- call --
        caller: to_optional_vector(&call.caller),

        // -- ordering --
        ordinal: transfer.ordinal,
        index,
        global_sequence: to_global_sequence(clock, index),

        // -- transfer --
        contract: NATIVE_ADDRESS.to_vec(),
        from: transfer.from,
        to: transfer.to,
        value: transfer.value.to_string(),

        // -- debug --
        algorithm: transfer.algorithm.into(),
        r#type: trx.r#type().as_str_name().to_string(),
    }
}

pub fn insert_events<'a>(clock: &'a Clock, block: &'a Block, events: &mut Events) {
    let mut index = 0; // relative index for ordering

    // Pre-allocate a default TransactionTrace to avoid creating it multiple times
    let default_trace = TransactionTrace::default();
    let default_call = Call::default();

    // balance changes at block level
    for balance_change in &block.balance_changes {
        events
            .balance_changes
            .push(to_balance_change(clock, &default_trace, &default_call, balance_change, Algorithm::Block, index));
        index += 1;
    }

    // balance changes at system call level
    for call in &block.system_calls {
        for balance_change in &call.balance_changes {
            events
                .balance_changes
                .push(to_balance_change(clock, &default_trace, call, balance_change, Algorithm::System, index));
            index += 1;
        }
    }

    // iterate over successful transactions
    for trx in block.transactions() {
        // find all transfers from transactions
        if let Some(transfer) = get_transfer_from_transaction(trx) {
            events.transfers.push(to_transfer(clock, trx, &default_call, transfer, index));
            index += 1;
        }
        // find all transfers from calls
        for call_view in trx.calls() {
            if let Some(transfer) = get_transfer_from_call(call_view.call) {
                events.transfers.push(to_transfer(clock, trx, call_view.call, transfer, index));
                index += 1;
            }
        }
    }

    // iterate over all transactions including failed ones
    for trx in &block.transaction_traces {
        for call_view in trx.calls() {
            // balance changes
            for balance_change in &call_view.call.balance_changes {
                let is_failed = is_failed_transaction(trx);

                // failed transactions with gas balance changes are still considered as valid balance changes
                let algorithm = if is_failed && is_gas_balance_change(balance_change) {
                    Algorithm::Gas
                // skip failed transactions
                } else if is_failed {
                    continue;
                // valid balance change
                } else {
                    Algorithm::Call
                };

                // balance change
                events.balance_changes.push(to_balance_change(clock, trx, call_view.call, balance_change, algorithm, index));
                index += 1;
            }
        }
    }
}
