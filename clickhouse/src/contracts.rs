use common::bytes_to_hex;
use proto::pb::evm::tokens::contracts::v1::{ContractChange, ContractCreation, Events};
use substreams::pb::substreams::Clock;

use common::clickhouse::{common_key, set_caller, set_clock, set_ordering, set_transaction_id};

pub fn process_contracts(tables: &mut substreams_database_change::tables::Tables, clock: &Clock, events: Events, mut index: u64) -> u64 {
    for event in events.contract_creations {
        process_contract_creation(tables, clock, event, index);
        index += 1;
    }

    for event in events.contract_changes {
        process_contract_change(tables, clock, event, index);
        index += 1;
    }
    index
}

fn process_contract_change(tables: &mut substreams_database_change::tables::Tables, clock: &Clock, event: ContractChange, index: u64) {
    let key = common_key(clock, index);

    // handle Nullable values
    let decimals = event
        .decimals
        .map(|d| d.to_string()) // only allocate when we have a value
        .unwrap_or_default();

    let row = tables
        .create_row("contract_changes", key)
        .set("address", &bytes_to_hex(&event.address))
        .set("name", &event.name.unwrap_or("".to_string()))
        .set("symbol", &event.symbol.unwrap_or("".to_string()))
        .set("decimals", decimals);

    set_caller(event.caller, row);
    set_ordering(index, event.ordinal, clock, row);
    set_transaction_id(event.transaction_id, row);
    set_clock(clock, row);
}

fn process_contract_creation(tables: &mut substreams_database_change::tables::Tables, clock: &Clock, event: ContractCreation, index: u64) {
    let key = [("address", bytes_to_hex(&event.address))];
    let row = tables
        .create_row("contract_creations", key)
        .set("address", &bytes_to_hex(&event.address))
        .set("from", &bytes_to_hex(&event.from))
        .set("to", &bytes_to_hex(&event.to))
        .set("hash", &bytes_to_hex(&event.hash));

    set_caller(Some(event.caller), row);
    set_ordering(index, Some(event.ordinal), clock, row);
    set_transaction_id(Some(event.transaction_id), row);
    set_clock(clock, row);
}
