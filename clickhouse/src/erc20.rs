use common::bytes_to_hex;
use proto::pb::evm::tokens::balances::types::v1::{BalanceChange, Events, Transfer};
use substreams::pb::substreams::Clock;

use crate::common::{common_key, set_clock};

fn process_erc20(tables: &mut substreams_database_change::tables::Tables, clock: &Clock, events: Events) {
}

fn process_erc20_balance_change(tables: &mut substreams_database_change::tables::Tables, clock: &Clock, event: BalanceChange) {
    let key = common_key(clock, event.ordinal);
    let row = tables
        .create_row("balance_changes", key)
        // -- transaction --
        .set("transaction_id", bytes_to_hex(&event.transaction_id))
        // -- ordering --
        .set("ordinal", event.ordinal)
        .set("index", event.index)
        .set("global_sequence", event.global_sequence)
        // -- balance change --
        .set("contract", bytes_to_hex(&event.contract))
        .set("owner", bytes_to_hex(&event.owner))
        .set("old_balance", &event.old_balance)
        .set("new_balance", &event.new_balance)
        // -- debug --
        .set("algorithm", event.algorithm().as_str_name())
        .set("algorithm_code", event.algorithm);

    set_clock(clock, row);
}

fn process_erc20_transfer(tables: &mut substreams_database_change::tables::Tables, clock: &Clock, event: Transfer) {
    let key = common_key(clock, event.ordinal);
    let row = tables
        .create_row("transfers", key)
        // -- transaction --
        .set("transaction_id", bytes_to_hex(&event.transaction_id))
        // -- ordering --
        .set("ordinal", event.ordinal)
        .set("index", event.index)
        .set("global_sequence", event.global_sequence)
        // -- transfer --
        .set("contract", bytes_to_hex(&event.contract))
        .set("from", bytes_to_hex(&event.from))
        .set("to", bytes_to_hex(&event.to))
        .set("value", &event.value)
        // -- debug --
        .set("algorithm", event.algorithm().as_str_name())
        .set("algorithm_code", event.algorithm);

    set_clock(clock, row);
}
