use common::{bytes_to_hex, to_global_sequence};
use proto::pb::evm::tokens::balances::v1::{BalanceChange, Events, Transfer};
use substreams::pb::substreams::Clock;

use crate::common::{common_key, set_clock};

pub fn process_erc20_balances(tables: &mut substreams_database_change::tables::Tables, clock: &Clock, events: Events) {
    // Process ERC-20 balance changes
    for event in events.balance_changes {
        process_erc20_balance_change(tables, clock, event);
    }

    // Process ERC-20 transfers
    for event in events.transfers {
        process_erc20_transfer(tables, clock, event);
    }
}

fn process_erc20_balance_change(tables: &mut substreams_database_change::tables::Tables, clock: &Clock, event: BalanceChange) {
    let key = common_key(clock, event.ordinal);
    let row = tables
        .create_row("balance_changes", key)
        // -- transaction --
        .set("transaction_id", bytes_to_hex(&event.transaction_id.as_ref().unwrap_or(&vec![])))
        // -- ordering --
        .set("ordinal", event.ordinal)
        .set("index", event.index)
        .set("global_sequence", event.global_sequence)
        // -- balance change --
        .set("contract", bytes_to_hex(&event.contract))
        .set("address", bytes_to_hex(&event.address))
        .set("old_balance", event.old_balance.as_ref().unwrap_or(&"0".to_string()))
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
        .set("transaction_id", bytes_to_hex(&event.transaction_id.as_ref().unwrap_or(&vec![])))
        // -- caller --
        .set("caller", bytes_to_hex(&event.caller.as_ref().unwrap_or(&vec![])))
        // -- ordering --
        .set("index", event.index)
        .set("ordinal", event.ordinal)
        .set("global_sequence", to_global_sequence(clock, event.index))
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
