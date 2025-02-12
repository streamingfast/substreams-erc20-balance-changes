use crate::pb::erc20::types::v1::Events;
use substreams::errors::Error;
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

            // Timestamp support has been added in v0.36.0
            // https://github.com/graphprotocol/graph-node/releases/tag/v0.36.0
            .set("timestamp", balance_change.timestamp.expect("missing timestamp"))
            .set("date", balance_change.date)

            // -- transaction --
            .set("transaction_id", balance_change.transaction_id)

            // -- call --
            .set_bigint("call_index", &balance_change.call_index.to_string())

            // -- log --
            .set_bigint("log_index", &balance_change.log_index.to_string())
            .set_bigint("log_block_index", &balance_change.log_block_index.to_string())
            .set_bigint("log_ordinal", &balance_change.log_ordinal.to_string())

            // -- storage --
            .set("storage_key", balance_change.storage_key)
            .set_bigint("storage_ordinal", &balance_change.storage_ordinal.to_string())

            // -- indexing --
            .set_bigint("index", &balance_change.index.to_string())
            .set_bigint("version", &balance_change.version.to_string())

            // -- balance change --
            .set("contract", balance_change.contract)
            .set("owner", balance_change.owner)
            .set_bigint("old_balance", &balance_change.old_balance)
            .set_bigint("new_balance", &balance_change.new_balance)

            // -- transfer --
            .set("from", balance_change.from)
            .set("to", balance_change.to)
            .set_bigint("value", &balance_change.value)

            // -- debug --
            .set_bigint("change_type", &balance_change.change_type.to_string());
    }


    for transfer in events.transfers {
        let key = format!("{}:{}", transfer.transaction_id, transfer.call_index);
        tables.create_row("Transfer", key)
            // -- block --
            .set_bigint("block_num", &transfer.block_num.to_string())
            .set("block_hash", transfer.block_hash)
            .set("timestamp", transfer.timestamp.expect("missing timestamp"))
            .set("date", transfer.date)

            // -- transaction --
            .set("transaction_id", transfer.transaction_id)

            // -- call --
            .set_bigint("call_index", &transfer.call_index.to_string())

            // -- log --
            .set_bigint("log_index", &transfer.log_index.to_string())
            .set_bigint("log_block_index", &transfer.log_block_index.to_string())
            .set_bigint("log_ordinal", &transfer.log_ordinal.to_string())

            // -- transfer --
            .set("contract", transfer.contract)
            .set("from", transfer.from)
            .set("to", transfer.to)
            .set_bigint("value", &transfer.value);
    }

    Ok(tables.to_entity_changes())
}

#[substreams::handlers::map]
pub fn db_out(events: Events) -> Result<DatabaseChanges, Error> {
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

            // -- call --
            .set("call_index", &balance_change.call_index.to_string())

            // -- log --
            .set("log_index", &balance_change.log_index.to_string())
            .set("log_block_index", &balance_change.log_block_index.to_string())
            .set("log_ordinal", &balance_change.log_ordinal.to_string())

            // -- storage --
            .set("storage_key", balance_change.storage_key)
            .set("storage_ordinal", balance_change.storage_ordinal)

            // -- indexing --
            .set("index", &balance_change.index.to_string())
            .set("version", &balance_change.version.to_string())

            // -- balance change --
            .set("contract", balance_change.contract)
            .set("owner", balance_change.owner)
            .set("old_balance", balance_change.old_balance)
            .set("new_balance", balance_change.new_balance)

            // -- transfer --
            .set("from", balance_change.from)
            .set("to", balance_change.to)
            .set("value", &balance_change.value)

            // -- debug --
            .set("change_type", &balance_change.change_type.to_string());
    }

    for transfer in events.transfers {
        tables.create_row("transfers", [
            ("transaction_id", (&transfer).transaction_id.to_string()),
            ("index", (&transfer).call_index.to_string())
        ])
            // -- block --
            .set("block_num", &transfer.block_num.to_string())
            .set("block_hash", transfer.block_hash)
            .set("timestamp", transfer.timestamp.expect("missing timestamp"))
            .set("date", transfer.date)

            // -- transaction --
            .set("transaction_id", transfer.transaction_id)

            // -- call --
            .set("call_index", &transfer.call_index.to_string())

            // -- log --
            .set("log_index", &transfer.log_index.to_string())
            .set("log_block_index", &transfer.log_block_index.to_string())
            .set("log_ordinal", &transfer.log_ordinal.to_string())

            // -- transfer --
            .set("contract", transfer.contract)
            .set("from", transfer.from)
            .set("to", transfer.to)
            .set("value", transfer.value);
    }

    Ok(tables.to_database_changes())
}
