use std::vec;

use common::to_global_sequence;
use proto::pb::evm::tokens::contracts::types::v1::{Algorithm, ContractChange, ContractCreation, Events};
use substreams::pb::substreams::Clock;
use substreams::store::{DeltaBigInt, Deltas};
use substreams::{errors::Error, scalar::BigInt, Hex};
use substreams_ethereum::pb::eth::v2::{Block, CallType};

use crate::calls;

#[substreams::handlers::map]
pub fn map_events(clock: Clock, block: Block, store_erc20_transfers: Deltas<DeltaBigInt>) -> Result<Events, Error> {
    let mut events = Events::default();
    let mut index = 0;
    // -- contract creations --
    for trx in block.transactions() {
        for call in &trx.calls {
            if call.call_type() != CallType::Create {
                continue;
            }
            for code_change in &call.code_changes {
                events.contract_creations.push(ContractCreation {
                    // -- transaction --
                    transaction_id: trx.hash.clone(),
                    from: trx.from.clone(),
                    to: trx.to.clone(),

                    // -- ordering --
                    ordinal: 0,
                    index,
                    global_sequence: to_global_sequence(&clock, &index),

                    // -- contract --
                    address: code_change.address.clone(),
                });
                index += 1;
            }
        }
    }
    // TO-DO: pull from known symbol & name contract updates
    // - setMetadata
    // - setNameAndTicker
    // - setName
    // https://github.com/pinax-network/substreams-evm-tokens/issues/13

    // -- fetch contract metadata via RPC --
    for deltas in store_erc20_transfers.deltas {
        // match using 1st block which includes ERC20 token transfer event per address
        // or every 10,000 blocks (~24h ETH, ~6h BSC, ~4h Base)
        if deltas.new_value != BigInt::one() && deltas.new_value.to_u64() % 10000 != 0 {
            continue;
        }
        let address = Hex::decode(&deltas.key).expect("invalid address");
        let contract = calls::get_contract(address.clone());
        if contract.is_none() {
            continue;
        } // not valid ERC20 token contract
        let (name, symbol, decimals) = contract.unwrap();

        events.contract_changes.push(ContractChange {
            // -- transaction --
            transaction_id: vec![],
            from: vec![],
            to: vec![],

            // -- ordering --
            ordinal: 0,
            index,
            global_sequence: to_global_sequence(&clock, &index),

            // -- contract --
            address,
            name,
            symbol,
            decimals: decimals.into(),

            // -- debug --
            algorithm: Algorithm::Rpc.into(),
        });
        index += 1;
    }
    Ok(events)
}
