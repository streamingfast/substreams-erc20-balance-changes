use substreams::scalar::BigInt;
use substreams_ethereum::pb::eth::v2::{Call, balance_change::Reason, BalanceChange, TransactionTrace, TransactionTraceStatus};

pub fn is_failed_transaction(trx: &TransactionTrace) -> bool {
    let status = trx.status();
    if status == TransactionTraceStatus::Reverted || status == TransactionTraceStatus::Failed {
        return true;
    }
    false
}

pub fn is_failed_call(call: &Call) -> bool {
    if call.state_reverted || call.status_failed || call.state_reverted {
        return true;
    }
    false
}

pub fn is_gas_balance_change(balance_change: &BalanceChange) -> bool {
    let reason = balance_change.reason();
    if reason == Reason::GasBuy || reason == Reason::GasRefund || reason == Reason::RewardTransactionFee {
        return true;
    }
    false
}

pub fn is_transfer_balance_change(balance_change: &BalanceChange) -> bool {
    let reason = balance_change.reason();
    if reason == Reason::Transfer {
        return true;
    }
    false
}

pub fn get_balances(balance_change: &BalanceChange) -> (BigInt, BigInt) {
    let old_balance = match balance_change.old_value.as_ref() {
        Some(v) => BigInt::from_unsigned_bytes_be(&v.bytes),
        None => BigInt::zero(),
    };

    let new_balance = match balance_change.new_value.as_ref() {
        Some(v) => BigInt::from_unsigned_bytes_be(&v.bytes),
        None => BigInt::zero(),
    };

    (old_balance, new_balance)
}