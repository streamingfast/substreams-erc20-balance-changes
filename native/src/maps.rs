use proto::pb::evm::tokens::types::v1::{BalanceChange, BalanceChangeType, Events};
use erc20::utils::{clock_to_date, to_global_sequence};
use substreams::Hex;
use substreams::{errors::Error, scalar::BigInt};

use substreams::pb::substreams::Clock;
use substreams_ethereum::pb::eth::v2::balance_change::Reason;
use substreams_ethereum::pb::eth::v2::{BalanceChange as BalanceChangeAbi, Block, Call, TransactionTrace, TransactionTraceStatus};

#[substreams::handlers::map]
pub fn map_events(clock: Clock, block: Block) -> Result<Events, Error> {
    let mut events = Events::default();
    insert_events(&clock, &block, &mut events);
    Ok(events)
}

pub fn to_balance_change<'a>(
    clock: &Clock,
    trx: &'a TransactionTrace,
    call: &'a Call,
    balance_change: &'a BalanceChangeAbi,
    change_type: BalanceChangeType,
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

        // -- call --
        call_index: call.index,
        call_address: Hex::encode(&call.address),

        // -- log --
        log_index: 0,
        log_block_index: 0,
        log_ordinal: 0,

        // -- balance change --
        contract: "native".to_string(),
        owner: Hex::encode(&balance_change.address),
        old_balance: old_balance.to_string(),
        new_balance: new_balance.to_string(),

        // -- indexing --
        ordinal: balance_change.ordinal,
        global_sequence: to_global_sequence(clock, &balance_change.ordinal),

        // -- debug --
        r#type: change_type as i32,
    }
}

pub fn insert_events<'a>(clock: &'a Clock, block: &'a Block, events: &mut Events) {
    // balance changes from block
    for balance_change in &block.balance_changes {
        if skip_balance_change(&balance_change) { continue; }
        events.balance_changes.push(
            to_balance_change(clock, &TransactionTrace::default(), &Call::default(), balance_change, BalanceChangeType::Native)
        );
    }

    // balance changes from transactions
    for trx in &block.transaction_traces {

        // failed transactions incur balance changes due to gas buy and refund
        let status = trx.status();
        if !(status == TransactionTraceStatus::Reverted || status == TransactionTraceStatus::Failed) { continue; }

        for call_view in trx.calls() {
            let call = call_view.call;
            for balance_change in &call.balance_changes {
                if skip_balance_change(&balance_change) { continue; }
                events.balance_changes.push(
                    to_balance_change(clock, &trx, call, &balance_change, BalanceChangeType::Native)
                );
            }
        }
    }
}

pub fn skip_balance_change(balance_change: &BalanceChangeAbi) -> bool {
    let reason = balance_change.reason();
    if !(reason == Reason::GasBuy || reason == Reason::GasRefund || reason == Reason::RewardTransactionFee) {
        return true;
    }
    return false;
}
