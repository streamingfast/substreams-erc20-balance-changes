use common::to_global_sequence;
use proto::pb::evm::tokens::contracts::types::v1::{ContractCreation, Events};
use substreams::errors::Error;
use substreams::pb::substreams::Clock;
use substreams_ethereum::pb::eth::v2::{Block, CallType};

#[substreams::handlers::map]
pub fn map_events(clock: Clock, block: Block) -> Result<Events, Error> {
    // Construct and return the events
    Ok(Events {
        ..Default::default()
    })
    // TO-DO: pull from known symbol & name contract updates
    // - setMetadata
    // - setNameAndTicker
    // - setName
    // https://github.com/pinax-network/substreams-evm-tokens/issues/13
}
