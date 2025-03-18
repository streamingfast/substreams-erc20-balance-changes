use proto::pb::evm::tokens::balances::types::v1::Algorithm;
use substreams::scalar::BigInt;
use substreams_ethereum::pb::eth::v2::{Call, TransactionTrace};

use crate::{
    maps::TransferStruct,
    utils::{is_failed_call, is_failed_transaction},
};

pub fn get_transfer_from_transaction<'a>(trx: &'a TransactionTrace) -> Option<TransferStruct> {
    if is_failed_transaction(trx) {
        return None;
    }

    let value = BigInt::from_unsigned_bytes_be(trx.value.as_ref()?.bytes.as_ref());
    if value.le(&BigInt::zero()) {
        return None;
    }

    Some(TransferStruct {
        from: trx.from.to_vec(),
        to: trx.to.to_vec(),
        value,
        ordinal: trx.begin_ordinal,
        algorithm: Algorithm::Transaction,
    })
}

pub fn get_transfer_from_call<'a>(call: &'a Call) -> Option<TransferStruct> {
    if is_failed_call(call) {
        return None;
    }

    let value = BigInt::from_unsigned_bytes_be(call.value.as_ref()?.bytes.as_ref());
    if value.le(&BigInt::zero()) {
        return None;
    }

    Some(TransferStruct {
        from: call.caller.to_vec(),
        to: call.address.to_vec(),
        value,
        ordinal: call.begin_ordinal,
        algorithm: Algorithm::Call,
    })
}
