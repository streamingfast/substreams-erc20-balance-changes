use proto::pb::evm::tokens::balances::types::v1::Algorithm;
use substreams::scalar::BigInt;
use substreams_ethereum::pb::eth::v2::{Call, TransactionTrace};

use crate::{maps::TransferStruct, utils::{is_failed_call, is_failed_transaction}};

pub fn get_transfer_from_transaction<'a>(trx: &'a TransactionTrace) -> Option<TransferStruct> {
    if is_failed_transaction(trx) {
        return None;
    }
    // transfer must be > 0
    let value = match trx.value.as_ref() {
        Some(v) => BigInt::from_unsigned_bytes_be(&v.bytes),
        None => return None,
    };
    if value.le(&BigInt::zero()) {
        return None;
    }

    Some(TransferStruct{
        from: trx.from.clone(),
        to: trx.to.clone(),
        value,
        ordinal: trx.begin_ordinal,
        algorithm: Algorithm::Transaction,
    })
}

pub fn get_transfer_from_call<'a>(call: &'a Call) -> Option<TransferStruct> {
    if is_failed_call(call) {
        return None;
    }
    // transfer must be > 0
    let value = match call.value.as_ref() {
        Some(v) => BigInt::from_unsigned_bytes_be(&v.bytes),
        None => return None,
    };
    if value.le(&BigInt::zero()) {
        return None;
    }

    Some(TransferStruct{
        from: call.caller.clone(),
        to: call.address.clone(),
        value,
        ordinal: call.begin_ordinal,
        algorithm: Algorithm::Call,
    })
}
