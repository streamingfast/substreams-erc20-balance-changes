use proto::pb::evm::tokens::types::v1::balance_change::Algorithm;
use proto::pb::evm::tokens::types::v1::{BalanceChange, Events, Transfer};
use erc20::utils::{clock_to_date, to_global_sequence};
use substreams::Hex;
use substreams::{errors::Error, scalar::BigInt};

use substreams::pb::substreams::Clock;
use substreams_ethereum::pb::eth::v2::balance_change::Reason;
use substreams_ethereum::pb::eth::v2::{BalanceChange as BalanceChangeAbi, Block, TransactionTrace, TransactionTraceStatus};

#[substreams::handlers::map]
pub fn map_events(clock: Clock, block: Block) -> Result<Events, Error> {
    let mut events = Events::default();
    insert_events(&clock, &block, &mut events);
    Ok(events)
}

pub fn to_balance_change<'a>(
    clock: &Clock,
    trx: &'a TransactionTrace,
    balance_change: &'a BalanceChangeAbi,
    algorithm: Algorithm,
    index: &u64,
) -> BalanceChange {
    let old_balance = match balance_change.old_value.as_ref() {
        Some(v) => BigInt::from_unsigned_bytes_be(&v.bytes).to_string(),
        None => String::from("0"),
    };

    let new_balance = match balance_change.new_value.as_ref() {
        Some(v) => BigInt::from_unsigned_bytes_be(&v.bytes).to_string(),
        None => String::from("0"),
    };

    BalanceChange {
        // -- block --
        block_num: clock.number,
        block_hash: clock.id.clone(),
        date: clock_to_date(clock),
        timestamp: clock.timestamp,

        // -- transaction
        transaction_id: Hex::encode(&trx.hash),

        // -- balance change --
        contract: "native".to_string(),
        owner: Hex::encode(&balance_change.address),
        old_balance,
        new_balance,

        // -- ordering --
        ordinal: balance_change.ordinal,
        global_sequence: to_global_sequence(clock, index),

        // -- metadata --
        algorithm: algorithm.into(),
    }
}

pub fn to_transfer<'a>(clock: &'a Clock, trx: &'a TransactionTrace, balance_change: &'a BalanceChangeAbi, index: &u64) -> Transfer {
    let old_balance = match balance_change.old_value.as_ref() {
        Some(v) => BigInt::from_unsigned_bytes_be(&v.bytes),
        None => BigInt::zero(),
    };

    let new_balance = match balance_change.new_value.as_ref() {
        Some(v) => BigInt::from_unsigned_bytes_be(&v.bytes),
        None => BigInt::zero(),
    };

    let value = new_balance - old_balance;

    Transfer {
        // -- block --
        block_num: clock.number,
        block_hash: clock.id.clone(),
        date: clock_to_date(clock),
        timestamp: clock.timestamp,

        // -- transaction --
        transaction_id: Hex::encode(&trx.hash),

        // -- ordering --
        ordinal: trx.index.into(),
        global_sequence: to_global_sequence(clock, index),

        // -- transfer --
        contract: "native".to_string(),
        from: Hex::encode(&trx.from),
        to: Hex::encode(&trx.to),
        value: value.to_string(),
    }
}

pub fn insert_events<'a>(clock: &'a Clock, block: &'a Block, events: &mut Events) {
    let mut transfer_index = 0;
    let mut balance_changes_index = 0;

    // balance changes at block level
    for balance_change in &block.balance_changes {
        events.balance_changes.push(
            to_balance_change(clock, &TransactionTrace::default(), balance_change, Algorithm::NativeBlock, &transfer_index)
        );
        transfer_index += 1;
    }

    // balance changes from transactions
    for trx in &block.transaction_traces {
        for call_view in trx.calls() {
            for balance_change in &call_view.call.balance_changes {
                let algorithm = if is_failed_transaction(trx) {
                    Algorithm::NativeFailed
                } else if is_gas_balance_change(balance_change) {
                    Algorithm::NativeGas
                } else if is_transfer_balance_change(balance_change) {
                    Algorithm::NativeTransfer
                } else {
                    Algorithm::NativeBlock
                };

                // balance change
                events.balance_changes.push(
                    to_balance_change(clock, trx, balance_change, algorithm, &balance_changes_index)
                );
                balance_changes_index += 1;

                // transfer
                if balance_change.reason() == Reason::Transfer {

                    // only include transfer as sender (from), prevents duplicate Native transfers
                    if balance_change.address == trx.from {
                        continue;
                    }
                    events.transfers.push(
                        to_transfer(clock, &trx, balance_change, &transfer_index)
                    );
                    transfer_index += 1;
                }
            }
        }

    }
}

// failed transactions incur balance changes due to gas buy and refund
pub fn is_failed_transaction(trx: &TransactionTrace) -> bool {
    let status = trx.status();
    if status == TransactionTraceStatus::Reverted || status == TransactionTraceStatus::Failed {
        return true;
    }
    false
}

pub fn is_gas_balance_change(balance_change: &BalanceChangeAbi) -> bool {
    let reason = balance_change.reason();
    if reason == Reason::GasBuy || reason == Reason::GasRefund || reason == Reason::RewardTransactionFee {
        return true;
    }
    false
}

pub fn is_transfer_balance_change(balance_change: &BalanceChangeAbi) -> bool {
    let reason = balance_change.reason();
    if reason == Reason::Transfer {
        return true;
    }
    false
}