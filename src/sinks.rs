use crate::pb::erc20::types::v1::Events;
use substreams::{errors::Error, pb::substreams::Clock};
use substreams_entity_change::pb::entity::EntityChanges;
use substreams_database_change::pb::database::DatabaseChanges;

#[substreams::handlers::map]
pub fn graph_out(events: Events) -> Result<EntityChanges, Error> {
    let mut tables = substreams_entity_change::tables::Tables::new();

    for balance_change in events.balance_changes {
        let key = format!("{}:{}", balance_change.transaction_id, balance_change.index);
        tables.create_row("BalanceChange", key)
            // -- block --
            .set_bigint("block_num", &balance_change.block_num.to_string())
            .set("block_hash", balance_change.block_hash)
            .set("timestamp", balance_change.timestamp.expect("missing timestamp"))
            .set("date", balance_change.date)

            // -- transaction --
            .set("transaction_id", balance_change.transaction_id)
            .set_bigint("call_index", &balance_change.call_index.to_string())

            // -- storage --
            .set_bigint("index", &balance_change.index.to_string())
            .set_bigint("version", &balance_change.version.to_string())
            .set("storage_key", balance_change.storage_key)

            // -- balance change --
            .set("contract", balance_change.contract)
            .set("owner", balance_change.owner)
            .set("old_balance", balance_change.old_balance)
            .set("new_balance", balance_change.new_balance)
            .set("amount", balance_change.amount);
    }

    Ok(tables.to_entity_changes())
}

#[substreams::handlers::map]
pub fn db_out(clock: Clock, events: Events) -> Result<DatabaseChanges, Error> {
    let mut tables = substreams_database_change::tables::Tables::new();

    for balance_change in events.balance_changes {

        tables.create_row("balance_changes", [
            ("transaction_id", (&balance_change).transaction_id.to_string()),
            ("index", (&balance_change).index.to_string())
        ])
            // -- block --
            .set("block_num", &balance_change.block_num.to_string())
            .set("block_hash", balance_change.block_hash)
            .set("timestamp", balance_change.timestamp.expect("missing timestamp"))
            .set("date", balance_change.date)

            // -- transaction --
            .set("transaction_id", balance_change.transaction_id)
            .set("call_index", &balance_change.call_index.to_string())

            // -- storage --
            .set("index", &balance_change.index.to_string())
            .set("version", &balance_change.version.to_string())
            .set("storage_key", balance_change.storage_key)

            // -- balance change --
            .set("contract", balance_change.contract)
            .set("owner", balance_change.owner)
            .set("old_balance", balance_change.old_balance)
            .set("new_balance", balance_change.new_balance)
            .set("amount", balance_change.amount);
    }

    Ok(tables.to_database_changes())
}
