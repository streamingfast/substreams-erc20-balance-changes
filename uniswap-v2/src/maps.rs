use proto::pb::evm::tokens::uniswap::v2::{Events, PairCreated, Swap, Sync};
use substreams::errors::Error;
use substreams_abis::evm::uniswap::v2::factory::events::PairCreated as PairCreatedAbi;
use substreams_abis::evm::uniswap::v2::pair::events::{Swap as SwapAbi, Sync as SyncAbi};
use substreams_ethereum::pb::eth::v2::Block;
use substreams_ethereum::Event;

#[substreams::handlers::map]
pub fn map_events(block: Block) -> Result<Events, Error> {
    let mut events = Events::default();

    // === Uniswap::V2 ===
    // https://github.com/Uniswap/v2-core
    // https://github.com/pinax-network/substreams-abis/tree/main/abi/evm/uniswap/v2
    for trx in block.transactions() {
        for (log, call_view) in trx.logs_with_calls() {
            // Uniswap::V2::Pair:Sync
            if let Some(event) = SyncAbi::match_and_decode(log) {
                events.syncs.push(Sync {
                    // -- transaction --
                    transaction_id: trx.hash.to_vec(),
                    // -- call --
                    caller: call_view.call.caller.to_vec(),
                    // -- ordering --
                    ordinal: log.ordinal,
                    // -- sync --
                    address: log.address.to_vec(),
                    reserve0: event.reserve0.to_string(),
                    reserve1: event.reserve1.to_string(),
                });
            // Uniswap::V2::Pair:Swap
            } else if let Some(event) = SwapAbi::match_and_decode(log) {
                events.swaps.push(Swap {
                    // -- transaction --
                    transaction_id: trx.hash.to_vec(),
                    // -- call --
                    caller: call_view.call.caller.to_vec(),
                    // -- ordering --
                    ordinal: log.ordinal,
                    // -- swap --
                    address: log.address.to_vec(),
                    amount0_in: event.amount0_in.to_string(),
                    amount0_out: event.amount0_out.to_string(),
                    amount1_in: event.amount1_in.to_string(),
                    amount1_out: event.amount1_out.to_string(),
                    sender: event.sender,
                    to: event.to,
                });
            // Uniswap::V2::Factory:PairCreated
            } else if let Some(event) = PairCreatedAbi::match_and_decode(log) {
                events.pairs_created.push(PairCreated {
                    // -- transaction --
                    transaction_id: trx.hash.to_vec(),
                    // -- call --
                    caller: call_view.call.caller.to_vec(),
                    // -- ordering --
                    ordinal: log.ordinal,
                    // -- pair created --
                    address: log.address.to_vec(),
                    pair: event.pair,
                    token0: event.token0,
                    token1: event.token1,
                });
            }
        }
    }

    Ok(events)
}
