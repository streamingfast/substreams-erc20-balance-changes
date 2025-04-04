use common::bytes_to_hex;
use proto::pb::evm::tokens::uniswap::v2::{Sync, Swap, PairCreated, Events};
use substreams::pb::substreams::Clock;

use crate::common::{common_key, set_caller, set_clock, set_ordering, set_transaction_id};

pub fn process_uniswap_v2(tables: &mut substreams_database_change::tables::Tables, clock: &Clock, events: Events, mut index: u64) -> u64 {
    for event in events.swaps {
        process_uniswap_v2_swaps(tables, clock, event, index);
        index += 1;
    }

    for event in events.syncs {
        process_uniswap_v2_syncs(tables, clock, event, index);
        index += 1;
    }

    for event in events.pairs_created {
        process_uniswap_v2_pairs_created(tables, clock, event, index);
        index += 1;
    }
    index
}

fn process_uniswap_v2_swaps(tables: &mut substreams_database_change::tables::Tables, clock: &Clock, event: Swap, index: u64) {
    let key = common_key(clock, index);
    let row = tables
        .create_row("uniswap_v2_swaps", key)
        .set("address", &bytes_to_hex(&event.address))
        .set("amount0_in", event.amount0_in)
        .set("amount0_out", event.amount0_out)
        .set("amount1_in", event.amount1_in)
        .set("amount1_out", event.amount1_out)
        .set("sender", bytes_to_hex(&event.sender))
        .set("to", bytes_to_hex(&event.to));

    set_caller(Some(event.caller), row);
    set_ordering(index, Some(event.ordinal), clock, row);
    set_transaction_id(Some(event.transaction_id), row);
    set_clock(clock, row);
}

fn process_uniswap_v2_syncs(tables: &mut substreams_database_change::tables::Tables, clock: &Clock, event: Sync, index: u64) {
    let key = common_key(clock, index);
    let row = tables
        .create_row("uniswap_v2_syncs", key)
        .set("address", &bytes_to_hex(&event.address))
        .set("reserve0", event.reserve0.to_string())
        .set("reserve1", event.reserve1.to_string());

    set_caller(Some(event.caller), row);
    set_ordering(index, Some(event.ordinal), clock, row);
    set_transaction_id(Some(event.transaction_id), row);
    set_clock(clock, row);
}

fn process_uniswap_v2_pairs_created(tables: &mut substreams_database_change::tables::Tables, clock: &Clock, event: PairCreated, index: u64) {
    let key = [
        ("address", bytes_to_hex(&event.address)),
        ("pair", bytes_to_hex(&event.pair))
    ];
    let row = tables
        .create_row("uniswap_v2_pairs_created", key)
        .set("token0", bytes_to_hex(&event.token0))
        .set("token1", bytes_to_hex(&event.token1))
        .set("pair", bytes_to_hex(&event.pair));

    set_caller(Some(event.caller), row);
    set_ordering(index, Some(event.ordinal), clock, row);
    set_transaction_id(Some(event.transaction_id), row);
    set_clock(clock, row);
}
