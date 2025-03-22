use common::to_global_sequence;
use proto::pb::evm::tokens::contracts::types::v1::{ContractCreation, Events};
use substreams::errors::Error;
use substreams::pb::substreams::Clock;
use substreams_ethereum::pb::eth::v2::{Block, CallType};

#[substreams::handlers::map]
pub fn map_events(clock: Clock, block: Block) -> Result<Events, Error> {
    let contract_creations: Vec<ContractCreation> = block
        .transactions()
        .flat_map(|trx| {
            trx.calls
                .iter()
                .filter(|call| call.call_type() == CallType::Create)
                .flat_map(move |call| call.code_changes.iter().map(move |code_change| (trx, call, code_change)))
        })
        .enumerate()
        .map(|(idx, (trx, _call, code_change))| {
            ContractCreation {
                // -- transaction --
                transaction_id: trx.hash.clone(),
                from: trx.from.clone(),
                to: trx.to.clone(),

                // // -- call --
                // TO-DO: https://github.com/pinax-network/substreams-evm-tokens/issues/17
                // caller: call.caller.to_vec(),

                // -- ordering --
                ordinal: code_change.ordinal,
                index: idx as u64,
                global_sequence: to_global_sequence(&clock, idx as u64),

                // -- contract --
                address: code_change.address.clone(),
            }
        })
        .collect();

    // Construct and return the events
    Ok(Events {
        contract_creations,
        ..Default::default()
    })
    // TO-DO: pull from known symbol & name contract updates
    // - setMetadata
    // - setNameAndTicker
    // - setName
    // https://github.com/pinax-network/substreams-evm-tokens/issues/13
}
