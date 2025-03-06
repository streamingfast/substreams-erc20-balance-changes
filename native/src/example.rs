use substreams::errors::Error;
use substreams::scalar::{BigInt};
use substreams::Hex;
use substreams_ethereum::pb::eth::v2::{Block, TransactionTraceStatus};
use substreams_ethereum::pb::eth::v2::balance_change::Reason;
use crate::pb::eth::types::v1::{BalanceChange, BalanceChanges};


#[substreams::handlers::map]
pub fn map_balance_changes(block: Block) -> Result<BalanceChanges, Error> {
    Ok(BalanceChanges {
        balance_changes: map_balance_changes_from_block(block),
    })
}

pub fn map_balance_changes_from_block(block: Block) -> Vec<BalanceChange> {
    let mut out = Vec::new();
    for bc  in block.balance_changes.iter() {
        let old_value = match bc.old_value.as_ref() {
            Some(v) => BigInt::from_unsigned_bytes_be(&v.bytes).to_string(),
            None => String::from("0"),
        };

        let new_value = match bc.new_value.as_ref() {
            Some(v) => BigInt::from_unsigned_bytes_be(&v.bytes).to_string(),
            None => String::from("0"),
        };

        out.push(BalanceChange{
            address: Hex::encode(&bc.address),
            old_value,
            new_value,
            reason: Reason::from_i32(bc.reason).unwrap().as_str_name().into(),
            ordinal: bc.ordinal,
            transaction: "".to_string(),
        })
    }

    for trx in block.transaction_traces {
        if trx.status == TransactionTraceStatus::Reverted as i32 || trx.status == TransactionTraceStatus::Failed as i32 {
            // We need to process gas buy (and refund if transaction reverted) as well as mining rewards when a transaction fails
            // because those are still recorded in the state of the blockchain.

            let root_call = trx.calls.get(0).unwrap(); // each trx is guaranteed to have a root call

            for bc in &root_call.balance_changes {
                let reason = Reason::from_i32(bc.reason).unwrap();
                if !(reason == Reason::GasBuy || reason == Reason::GasRefund || reason == Reason::RewardTransactionFee) {
                    continue
                }

                let old_value = match bc.old_value.as_ref() {
                    Some(v) => BigInt::from_unsigned_bytes_be(&v.bytes).to_string(),
                    None => String::from("0"),
                };

                let new_value = match bc.new_value.as_ref() {
                    Some(v) => BigInt::from_unsigned_bytes_be(&v.bytes).to_string(),
                    None => String::from("0"),
                };

                out.push(BalanceChange{
                    address: Hex::encode(&bc.address),
                    old_value,
                    new_value,
                    reason: reason.as_str_name().into(),
                    ordinal: bc.ordinal,
                    transaction: Hex::encode(&trx.hash),
                })
            }
        }
        
        for call in trx.calls.iter() {
            if call.state_reverted {
                continue;
            }

            for bc in call.balance_changes.iter() {
                let old_value = match bc.old_value.as_ref() {
                    Some(v) => BigInt::from_unsigned_bytes_be(&v.bytes).to_string(),
                    None => String::from("0"),
                };

                let new_value = match bc.new_value.as_ref() {
                    Some(v) => BigInt::from_unsigned_bytes_be(&v.bytes).to_string(),
                    None => String::from("0"),
                };

                out.push(BalanceChange{
                    address: Hex::encode(&bc.address),
                    old_value,
                    new_value,
                    reason: Reason::from_i32(bc.reason).unwrap().as_str_name().into(),
                    ordinal: bc.ordinal,
                    transaction: Hex::encode(&trx.hash)
                })
            }
        }
    }

    out
}
