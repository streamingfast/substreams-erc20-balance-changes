use proto::pb::evm::tokens::types::v1::Algorithm;
use substreams::{scalar::BigInt, Hex};
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
        from: Hex::encode(&trx.from),
        to: Hex::encode(&trx.to),
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
        from: Hex::encode(&call.caller),
        to: Hex::encode(&call.address),
        value,
        ordinal: call.begin_ordinal,
        algorithm: Algorithm::Call,
    })
}
