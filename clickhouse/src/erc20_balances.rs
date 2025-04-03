use common::bytes_to_hex;
use proto::pb::evm::tokens::balances::v1::{BalanceChange, Events, Transfer};
use substreams::pb::substreams::Clock;

use crate::common::{common_key, set_algorithm, set_caller, set_clock, set_ordering, set_transaction_id};

pub fn process_erc20_balances(tables: &mut substreams_database_change::tables::Tables, clock: &Clock, events: Events, mut index: u64) -> u64 {
    // Process ERC-20 balance changes
    for event in events.balance_changes {
        process_erc20_balance_change(tables, clock, event, index);
        index += 1;
    }

    // Process ERC-20 transfers
    for event in events.transfers {
        process_erc20_transfer(tables, clock, event, index);
        index += 1;
    }
    index
}

fn process_erc20_balance_change(tables: &mut substreams_database_change::tables::Tables, clock: &Clock, event: BalanceChange, index: u64) {
    let key = common_key(clock, index);
    let row = tables
        .create_row("balance_changes", key)
        .set("contract", bytes_to_hex(&event.contract))
        .set("address", bytes_to_hex(&event.address))
        .set("old_balance", event.old_balance.as_ref().unwrap_or(&"0".to_string()))
        .set("new_balance", &event.new_balance);

    set_algorithm(event.algorithm(), row);
    set_caller(event.caller, row);
    set_ordering(index, event.ordinal, clock, row);
    set_transaction_id(event.transaction_id, row);
    set_clock(clock, row);
}

fn process_erc20_transfer(tables: &mut substreams_database_change::tables::Tables, clock: &Clock, event: Transfer, index: u64) {
    let key = common_key(clock, event.ordinal);
    let row = tables
        .create_row("transfers", key)
        .set("contract", bytes_to_hex(&event.contract))
        .set("from", bytes_to_hex(&event.from))
        .set("to", bytes_to_hex(&event.to))
        .set("value", &event.value);

    set_algorithm(event.algorithm(), row);
    set_ordering(index, Some(event.ordinal), clock, row);
    set_caller(event.caller, row);
    set_transaction_id(event.transaction_id, row);
    set_clock(clock, row);
}
