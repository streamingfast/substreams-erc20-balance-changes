
use common::to_global_sequence;
use proto::pb::evm::tokens::contracts::types::v1::{Algorithm, ContractChange, ContractCreation, Events};
use substreams::pb::substreams::Clock;
use substreams::errors::Error;
use substreams_ethereum::pb::eth::v2::{Block, CallType};

#[substreams::handlers::map]
pub fn map_events(clock: Clock, block: Block) -> Result<Events, Error> {
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
    Ok(events)
}
