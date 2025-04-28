use clickhouse::common::{common_key, set_bytes, set_caller, set_clock, set_ordering, set_transaction_id};
use proto::pb::evm::tokens::ens::v1::Events;
use substreams::errors::Error;
use substreams::pb::substreams::Clock;
use substreams_database_change::pb::database::DatabaseChanges;
use substreams_database_change::tables::Tables;

pub fn db_out(clock: &Clock, events: Events) -> Result<DatabaseChanges, Error> {
    let mut tables = Tables::new();
    let mut index = 0; // incremental index for each event

    // Process NameRegistered events
    for event in events.name_registered {
        let key = common_key(clock, index);
        let row = tables
            .create_row("name_registered", key)
            .set("name", &event.name)
            .set("label", &event.label)
            .set("owner", &event.owner)
            .set("base_cost", event.base_cost)
            .set("expires", event.expires);

        set_bytes(Some(event.contract), "contract", row);
        set_caller(Some(event.caller), row);
        set_transaction_id(Some(event.transaction_hash), row);
        set_ordering(index, Some(event.ordinal), clock, row);
        set_clock(clock, row);
        index += 1;
    }

    // Process TextChanged events
    for event in events.text_changed {
        let key = common_key(clock, index);
        let row = tables
            .create_row("text_changed", key)
            .set("node", &event.node)
            .set("key", &event.key)
            .set("value", &event.value);

        set_bytes(Some(event.contract), "contract", row);
        set_caller(Some(event.caller), row);
        set_ordering(index, Some(event.ordinal), clock, row);
        set_transaction_id(Some(event.transaction_hash), row);
        set_clock(clock, row);
    }

    // Process ReverseClaimed events (from name_changed events with .addr.reverse suffix)
    for event in events.reverse_claimed {
        let key = common_key(clock, index);
        let row = tables
            .create_row("reverse_claimed", key)
            .set("address", &event.address)
            .set("node", &event.node);

        set_bytes(Some(event.contract), "contract", row);
        set_caller(Some(event.caller), row);
        set_ordering(index, Some(event.ordinal), clock, row);
        set_transaction_id(Some(event.transaction_hash), row);
        set_clock(clock, row);
    }

    // Process NameChanged events
    for event in events.name_changed {
        let key = common_key(clock, index);
        let row = tables.create_row("name_changed", key).set("name", &event.name).set("node", &event.node);

        set_bytes(Some(event.contract), "contract", row);
        set_caller(Some(event.caller), row);
        set_ordering(index, Some(event.ordinal), clock, row);
        set_transaction_id(Some(event.transaction_hash), row);
        set_clock(clock, row);
    }

    // Process AddrChanged events
    for event in events.address_changed {
        let key = common_key(clock, index);
        let row = tables
            .create_row("address_changed", key)
            .set("address", &event.address)
            .set("node", &event.node);

        set_bytes(Some(event.contract), "contract", row);
        set_caller(Some(event.caller), row);
        set_ordering(index, Some(event.ordinal), clock, row);
        set_transaction_id(Some(event.transaction_hash), row);
        set_clock(clock, row);
    }

    // Process ContenthashChanged events
    for event in events.content_hash_changed {
        let key = common_key(clock, index);
        let row = tables.create_row("content_hash_changed", key).set("hash", &event.hash).set("node", &event.node);

        set_bytes(Some(event.contract), "contract", row);
        set_caller(Some(event.caller), row);
        set_ordering(index, Some(event.ordinal), clock, row);
        set_transaction_id(Some(event.transaction_hash), row);
        set_clock(clock, row);
    }

    Ok(tables.to_database_changes())
}
