use common::to_global_sequence;
use proto::pb::evm::tokens::native::contracts::v1::{ContractCreation, Events};
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
        .map(|(idx, (trx, call, code_change))| {
            ContractCreation {
                // -- transaction --
                transaction_id: trx.hash.to_vec(),
                from: trx.from.to_vec(),
                to: trx.to.to_vec(),

                // -- call --
                caller: call.caller.to_vec(),

                // -- ordering --
                ordinal: code_change.ordinal,
                index: idx as u64,
                global_sequence: to_global_sequence(&clock, idx as u64),

                // -- contract --
                address: code_change.address.to_vec(),
                hash: code_change.new_hash.to_vec(),
            }
        })
        .collect();

    Ok(Events {
        contract_creations,
    })
}
