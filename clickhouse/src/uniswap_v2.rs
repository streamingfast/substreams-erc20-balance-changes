use common::bytes_to_hex;
use proto::pb::evm::tokens::prices::uniswap::v2::types::v1::{PairCreated, Swap, Sync};
use substreams::pb::substreams::Clock;

use crate::common::{common_key, set_clock};

pub fn process_uniswap_v2_swap(tables: &mut substreams_database_change::tables::Tables, clock: &Clock, block_num: &str, date: &str, event: Swap) {
    let key = common_key(clock, event.ordinal);
    let row = tables
        .create_row("uniswap_v2_swaps", key)
        // -- transaction --
        .set("transaction_id", bytes_to_hex(&event.transaction_id))
        // -- log --
        .set("address", bytes_to_hex(&event.address))
        // -- ordering --
        .set("ordinal", event.ordinal)
        .set("index", event.index)
        .set("global_sequence", event.global_sequence)
        // -- swaps --
        .set("amount0_in", event.amount0_in)
        .set("amount0_out", event.amount0_out)
        .set("amount1_in", event.amount1_in)
        .set("amount1_out", event.amount1_out)
        .set("sender", bytes_to_hex(&event.sender))
        .set("to", bytes_to_hex(&event.to));

    set_clock(clock, row);
}

pub fn process_uniswap_v2_sync(tables: &mut substreams_database_change::tables::Tables, clock: &Clock, block_num: &str, date: &str, event: Sync) {
    let key = common_key(clock, event.ordinal);
    let row = tables
        .create_row("uniswap_v2_sync_changes", key)
        // -- transaction --
        .set("transaction_id", bytes_to_hex(&event.transaction_id))
        // -- log --
        .set("address", bytes_to_hex(&event.address))
        // -- ordering --
        .set("ordinal", event.ordinal)
        .set("index", event.index)
        .set("global_sequence", event.global_sequence)
        // -- log --
        .set("reserve0", event.reserve0.to_string())
        .set("reserve1", event.reserve1.to_string());

    set_clock(clock, row);
}

pub fn process_uniswap_v2_pair_created(tables: &mut substreams_database_change::tables::Tables, clock: &Clock, event: PairCreated) {
    let key = [("address", bytes_to_hex(&event.address)), ("pair", bytes_to_hex(&event.pair))];
    let row = tables
        .create_row("uniswap_v2_pairs_created", key)
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
        .set("pair", bytes_to_hex(&event.pair));

    set_clock(&clock, row);
}
