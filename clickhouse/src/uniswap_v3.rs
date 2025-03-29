use common::bytes_to_hex;
use proto::pb::evm::tokens::prices::uniswap::v3::types::v1::{Initialize, PoolCreated, Swap};
use substreams::pb::substreams::Clock;

use crate::common::{common_key, set_clock};

pub fn process_uniswap_v3_swap(tables: &mut substreams_database_change::tables::Tables, clock: &Clock, event: Swap) {
    let key = common_key(clock, event.ordinal);
    let row = tables
        .create_row("uniswap_v3_swaps", key)
        // -- transaction --
        .set("transaction_id", bytes_to_hex(&event.transaction_id))
        // -- log --
        .set("address", bytes_to_hex(&event.address))
        // -- ordering --
        .set("ordinal", event.ordinal)
        .set("index", event.index)
        .set("global_sequence", event.global_sequence)
        // -- swaps --
        .set("amount0", event.amount0)
        .set("amount1", event.amount1)
        .set("sender", bytes_to_hex(&event.sender))
        .set("recipient", bytes_to_hex(&event.recipient))
        .set("liquidity", &event.liquidity)
        .set("sqrt_price_x96", &event.sqrt_price_x96)
        .set("tick", &event.tick.to_string());

    set_clock(clock, row);
}

pub fn process_uniswap_v3_initializes(tables: &mut substreams_database_change::tables::Tables, clock: &Clock, event: Initialize) {
    let key = [("address", bytes_to_hex(&event.address))];
    let row = tables
        .create_row("uniswap_v3_initializes", key)
        // -- transaction --
        .set("transaction_id", bytes_to_hex(&event.transaction_id))
        // -- log --
        .set("address", bytes_to_hex(&event.address)) // log.address
        // -- ordering --
        .set("ordinal", event.ordinal)
        .set("index", event.index)
        .set("global_sequence", event.global_sequence)
        // -- pair created --
        .set("sqrt_price_x96", &event.sqrt_price_x96.to_string())
        .set("tick", &event.tick.to_string());

    set_clock(clock, row);
}

pub fn process_uniswap_v3_pools_created(tables: &mut substreams_database_change::tables::Tables, clock: &Clock, event: PoolCreated) {
    let key = [("address", bytes_to_hex(&event.address)), ("pool", bytes_to_hex(&event.pool))];
    let row = tables
        .create_row("uniswap_v3_pools_created", key)
        // -- transaction --
        .set("transaction_id", bytes_to_hex(&event.transaction_id))
        // -- log --
        .set("address", bytes_to_hex(&event.address)) // log.address
        // -- ordering --
        .set("ordinal", event.ordinal)
        .set("index", event.index)
        .set("global_sequence", event.global_sequence)
        // -- pair created --
        .set("token0", bytes_to_hex(&event.token0))
        .set("token1", bytes_to_hex(&event.token1))
        .set("pool", bytes_to_hex(&event.pool))
        .set("tick_spacing", event.tick_spacing.to_string())
        .set("fee", event.fee.to_string());

    set_clock(clock, row);
}