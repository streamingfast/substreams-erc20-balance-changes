use common::to_global_sequence;
use proto::pb::evm::tokens::uniswap::v3::{Events, PoolCreated, Initialize, Swap};
use substreams::errors::Error;
use substreams::pb::substreams::Clock;
use substreams_abis::evm::uniswap::v3::factory::events::PoolCreated as PoolCreatedAbi;
use substreams_abis::evm::uniswap::v3::pool::events::{Swap as SwapAbi, Initialize as InitializeAbi};
use substreams_ethereum::pb::eth::v2::Block;
use substreams_ethereum::Event;

#[substreams::handlers::map]
pub fn map_events(clock: Clock, block: Block) -> Result<Events, Error> {
    let mut events = Events::default();
    let mut index = 0;

    // === Uniswap::V3 ===
    // https://github.com/Uniswap/v3-core
    // https://github.com/pinax-network/substreams-abis/tree/main/abi/evm/uniswap/v3
    for trx in block.transactions() {
        for (log, _) in trx.logs_with_calls() {
            // Uniswap::V3::Pair:Swap
            if let Some(event) = SwapAbi::match_and_decode(log) {
                events.swaps.push(Swap {
                    // -- transaction --
                    transaction_id: trx.hash.to_vec(),
                    // -- log --
                    address: log.address.to_vec(),
                    // -- ordering --
                    ordinal: log.ordinal,
                    index,
                    global_sequence: to_global_sequence(&clock, index),
                    // -- swap --
                    amount0: event.amount0.to_string(),
                    amount1: event.amount1.to_string(),
                    sender: event.sender,
                    recipient: event.recipient,
                    liquidity: event.liquidity.to_string(),
                    sqrt_price_x96: event.sqrt_price_x96.to_string(),
                    tick: event.tick.into(),
                });
                index += 1;
            // Uniswap::V3::Factory:PoolCreated
            } else if let Some(event) = PoolCreatedAbi::match_and_decode(log) {
                events.pools_created.push(PoolCreated {
                    // -- transaction --
                    transaction_id: trx.hash.to_vec(),
                    // -- log --
                    address: log.address.to_vec(),
                    // -- ordering --
                    ordinal: log.ordinal,
                    index,
                    global_sequence: to_global_sequence(&clock, index),
                    // -- pair created --
                    pool: event.pool,
                    token0: event.token0,
                    token1: event.token1,
                    tick_spacing: event.tick_spacing.to_i32(),
                    fee: event.fee.to_u64(),
                });
                index += 1;
            // Uniswap::V3::Pool:Initialize
            } else if let Some(event) = InitializeAbi::match_and_decode(log) {
                events.intializes.push(Initialize {
                    // -- transaction --
                    transaction_id: trx.hash.to_vec(),
                    // -- log --
                    address: log.address.to_vec(),
                    // -- ordering --
                    ordinal: log.ordinal,
                    index,
                    global_sequence: to_global_sequence(&clock, index),
                    // -- intialize --
                    sqrt_price_x96: event.sqrt_price_x96.to_string(),
                    tick: event.tick.to_i32(),
                });
                index += 1;
            }
        }
    }

    Ok(events)
}
