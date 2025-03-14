
use common::to_global_sequence;
use substreams::{errors::Error, log, pb::substreams::Clock};
use proto::pb::evm::tokens::types::v1::{Algorithm, Contract, Events};
use substreams_ethereum::pb::eth::v2::{Block, CallType};

use crate::calls;

#[substreams::handlers::map]
pub fn map_events(clock: Clock, block: Block) -> Result<Events, Error> {
    let mut events = Events::default();
    let mut index = 0;

    for trx in block.transactions() {
        for call in &trx.calls {
            if call.call_type() != CallType::Create { continue }
            for code_change in &call.code_changes {
                let address = code_change.address.clone();
                let contract = calls::get_contract(address.clone());
                if contract.is_none() { continue }
                let (name, symbol, decimals) = contract.unwrap();

                events.contracts.push(Contract {
                    // -- transaction
                    transaction_id: trx.hash.clone(),
                    from: trx.from.clone(),
                    to: trx.to.clone(),

                    // -- ordering --
                    ordinal: code_change.ordinal,
                    global_sequence: to_global_sequence(&clock, &index),

                    // -- contract --
                    address: code_change.address.clone(),
                    name,
                    symbol,
                    decimals: decimals.into(),

                    // -- debug --
                    algorithm: Algorithm::ContractCreation.into(),
                });
                index += 1;
            }
        }
    }
    log::info!("index: {:?}", index);
    Ok(events)
}
