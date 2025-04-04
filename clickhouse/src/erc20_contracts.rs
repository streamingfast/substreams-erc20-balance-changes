use common::bytes_to_hex;
use proto::pb::evm::tokens::erc20::contracts::v1::{Events, ContractChange};
use substreams::pb::substreams::Clock;

use crate::common::{common_key, set_caller, set_clock, set_ordering, set_transaction_id};

pub fn process_erc20_contracts(tables: &mut substreams_database_change::tables::Tables, clock: &Clock, events: Events, mut index: u64) -> u64 {
    // Process ERC-20 contract changes
    for event in events.contract_changes {
        process_contract_change(tables, clock, event, index);
        index += 1;
    }
    index
}

fn process_contract_change(tables: &mut substreams_database_change::tables::Tables, clock: &Clock, event: ContractChange, index: u64) {
    let key = common_key(clock, index);
    let row = tables
        .create_row("contract_changes", key)
        .set("address", &bytes_to_hex(&event.address))
        .set("name", &event.name.unwrap_or("".to_string()))
        .set("symbol", &event.symbol.unwrap_or("".to_string()))
        .set("decimals", event.decimals.unwrap_or(0).to_string());

    set_caller(event.caller, row);
    set_ordering(index, event.ordinal, clock, row);
    set_transaction_id(event.transaction_id, row);
    set_clock(clock, row);
}
