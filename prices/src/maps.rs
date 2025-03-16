
use substreams::errors::Error;
use proto::pb::evm::tokens::prices::types::v1::{Events, Sync, PairCreated, Swap};
use substreams_abis::evm::uniswap::v2::pair::events::{Swap as SwapAbi, Sync as SyncAbi};
use substreams_abis::evm::uniswap::v2::factory::events::PairCreated as PairCreatedAbi;
use substreams_ethereum::pb::eth::v2::Block;
use substreams_ethereum::Event;

#[substreams::handlers::map]
pub fn map_events(block: Block ) -> Result<Events, Error> {
    let mut events = Events::default();

    // -- Uniswap V2 --
    for trx in block.transactions() {
        for (log, _) in trx.logs_with_calls() {
            // Syncs
            if let Some(event) = SyncAbi::match_and_decode(log) {
                events.syncs.push(Sync{
                    transaction_id: trx.hash.clone(),
                    reserve0: event.reserve0.to_string(),
                    reserve1: event.reserve1.to_string(),
                })
            // Swaps
            } else if let Some(event) = SwapAbi::match_and_decode(log) {
                events.swaps.push(Swap{
                    transaction_id: trx.hash.clone(),
                    amount0_in: event.amount0_in.to_string(),
                    amount0_out: event.amount0_out.to_string(),
                    amount1_in: event.amount1_in.to_string(),
                    amount1_out: event.amount1_out.to_string(),
                    sender: event.sender,
                    to: event.to,
                })
            // PairCreated
            } else if let Some(event) = PairCreatedAbi::match_and_decode(log) {
                events.pairs_created.push(PairCreated{
                    transaction_id: trx.hash.clone(),
                    from: trx.from.clone(),
                    factory: trx.to.clone(),
                    pair: event.pair,
                    token0: event.token0,
                    token1: event.token1,
                })
            }
        }
    }

    Ok(events)
}
