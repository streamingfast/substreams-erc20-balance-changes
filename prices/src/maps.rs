
use substreams::{errors::Error, scalar::BigInt, Hex};
use proto::pb::evm::tokens::prices::types::v1::Events;
use substreams_abis::evm::uniswap::v2::pair::events::{Swap, Sync};
use substreams_abis::evm::uniswap::v2::factory::events::PairCreated;
use substreams_ethereum::pb::eth::v2::Block;
use substreams_ethereum::Event;

#[substreams::handlers::map]
pub fn map_events(block: Block ) -> Result<Events, Error> {
    let mut events = Events::default();

    // -- Uniswap V2 --
    for trx in block.transactions() {
        for (log, _) in trx.logs_with_calls() {
            // Swaps
            if let Some(event) = Swap::match_and_decode(log) {
            // Syncs
            } else if let Some(event) = Sync::match_and_decode(log) {
                // event.reserve0
            // PairCreated
            } else if let Some(event) = PairCreated::match_and_decode(log) {
                // let event = to_event(&block.clock, &trx, log, event, &block.index);
                // events.events.push(event);
            }
        }
    }

    Ok(events)
}
