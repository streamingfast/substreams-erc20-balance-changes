use common::{Address, NULL_ADDRESS};
use proto::pb::evm::tokens::algorithm::v1::Algorithm;
use substreams::{log, scalar::BigInt};
use substreams_ethereum::pb::eth::v2::{Call, TransactionTrace};

use crate::{
    maps::TransferStruct,
    utils::{is_failed_call, is_failed_transaction},
};

pub fn get_transfer_from_transaction(trx: &TransactionTrace) -> Option<TransferStruct> {
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

pub fn get_transfer_from_call(call: &Call) -> Option<TransferStruct> {
    if is_failed_call(call) {
        return None;
    }

    let value = BigInt::from_unsigned_bytes_be(call.value.as_ref()?.bytes.as_ref());
    if value.le(&BigInt::zero()) {
        return None;
    }
    log::info!(
        "get_transfer_from_call: {} {} {}",
        call.parent_index,
        call.index,
        value
    );

    Some(TransferStruct {
        from: call.caller.to_vec(),
        to: call.address.to_vec(),
        value,
        ordinal: call.begin_ordinal,
        algorithm: Algorithm::Call,
    })
}

pub fn get_transfer_from_transaction_fee(trx: &TransactionTrace, base_fee_per_gas: &BigInt, coinbase: &Address) -> Vec<TransferStruct> {
    let mut transfers = Vec::new();
    let gas_price = match trx.gas_price {
        Some(ref data) => BigInt::from_unsigned_bytes_be(&data.bytes),
        None => BigInt::zero(),
    };
    let gas_used = BigInt::from(trx.gas_used);
    let transaction_fee = gas_price * &gas_used;
    let burn_fee = base_fee_per_gas * &gas_used;

    if transaction_fee.gt(&BigInt::zero()) {
        transfers.push(TransferStruct {
            from: trx.from.to_vec(),
            to: coinbase.to_vec(), // producer/miner address
            value: transaction_fee - burn_fee.clone(),
            ordinal: trx.begin_ordinal,
            algorithm: Algorithm::TransactionFee,
        });
    }
    if burn_fee.gt(&BigInt::zero()) {
        transfers.push(TransferStruct {
            from: trx.from.to_vec(),
            to: NULL_ADDRESS.to_vec(), // burned fee
            value: burn_fee,
            ordinal: trx.begin_ordinal,
            algorithm: Algorithm::TransactionFeeBurn,
        });
    }

    transfers
}