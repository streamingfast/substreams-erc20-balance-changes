use common::bytes_to_hex;
use common::clickhouse::{common_key, set_bytes, set_caller, set_clock, set_ordering, set_tx_hash};
use proto::pb::evm::tokens::ens::v1::Events;
use substreams::pb::substreams::Clock;
use substreams_database_change::pb::database::DatabaseChanges;
use substreams_database_change::tables::Tables;

#[substreams::handlers::map]
pub fn db_out(clock: Clock, events: Events) -> Result<DatabaseChanges, substreams::errors::Error> {
    let mut tables = Tables::new();
    let mut index = 0; // incremental index for each event

    for event in events.name_registered {
        let key = common_key(&clock, index);
        let row = tables
            .create_row("name_registered", key)
            .set("contract", bytes_to_hex(&event.contract))
            .set("expires", event.expires)
            // Optional values
            .set("name", &event.name.unwrap_or("".to_string()))
            .set("owner", &event.owner)
            .set("base_cost", event.base_cost.unwrap_or(0))
            .set("token_id", event.token_id.unwrap_or("".to_string()));

        // Optional Hex values
        set_bytes(event.label, "label", row);
        set_bytes(event.node, "node", row);

        // Default values
        set_caller(Some(event.caller), row);
        set_tx_hash(Some(event.transaction_hash), row);
        set_ordering(index, Some(event.ordinal), &clock, row);
        set_clock(&clock, row);
        index += 1;
    }

    for event in events.text_changed {
        let key = common_key(&clock, index);
        let row = tables
            .create_row("text_changed", key)
            .set("contract", bytes_to_hex(&event.contract))
            .set("node", &event.node)
            .set("key", &event.key)
            .set("value", &event.value);

        set_caller(Some(event.caller), row);
        set_ordering(index, Some(event.ordinal), &clock, row);
        set_tx_hash(Some(event.transaction_hash), row);
        set_clock(&clock, row);
        index += 1;
    }

    for event in events.reverse_claimed {
        let key = common_key(&clock, index);
        let row = tables
            .create_row("reverse_claimed", key)
            .set("contract", bytes_to_hex(&event.contract))
            .set("address", bytes_to_hex(&event.address))
            .set("node", bytes_to_hex(&event.node));

        set_caller(Some(event.caller), row);
        set_ordering(index, Some(event.ordinal), &clock, row);
        set_tx_hash(Some(event.transaction_hash), row);
        set_clock(&clock, row);
        index += 1;
    }

    for event in events.name_changed {
        let key = common_key(&clock, index);
        let row = tables
            .create_row("name_changed", key)
            .set("contract", bytes_to_hex(&event.contract))
            .set("name", &event.name)
            .set("node", bytes_to_hex(&event.node));

        set_caller(Some(event.caller), row);
        set_ordering(index, Some(event.ordinal), &clock, row);
        set_tx_hash(Some(event.transaction_hash), row);
        set_clock(&clock, row);
        index += 1;
    }

    for event in events.address_changed {
        let key = common_key(&clock, index);
        let row = tables
            .create_row("address_changed", key)
            .set("contract", bytes_to_hex(&event.contract))
            .set("address", bytes_to_hex(&event.address))
            .set("node", bytes_to_hex(&event.node));

        set_caller(Some(event.caller), row);
        set_ordering(index, Some(event.ordinal), &clock, row);
        set_tx_hash(Some(event.transaction_hash), row);
        set_clock(&clock, row);
        index += 1;
    }

    for event in events.content_hash_changed {
        let key = common_key(&clock, index);
        let row = tables
            .create_row("content_hash_changed", key)
            .set("contract", bytes_to_hex(&event.contract))
            .set("hash", bytes_to_hex(&event.hash))
            .set("node", bytes_to_hex(&event.node));

        set_caller(Some(event.caller), row);
        set_ordering(index, Some(event.ordinal), &clock, row);
        set_tx_hash(Some(event.transaction_hash), row);
        set_clock(&clock, row);
        index += 1;
    }

    for event in events.new_owner {
        let key = common_key(&clock, index);
        let row = tables
            .create_row("new_owner", key)
            .set("contract", bytes_to_hex(&event.contract))
            .set("node", bytes_to_hex(&event.node))
            .set("label", bytes_to_hex(&event.label))
            .set("owner", bytes_to_hex(&event.owner));

        set_caller(Some(event.caller), row);
        set_ordering(index, Some(event.ordinal), &clock, row);
        set_tx_hash(Some(event.transaction_hash), row);
        set_clock(&clock, row);
        index += 1;
    }

    Ok(tables.to_database_changes())
}
