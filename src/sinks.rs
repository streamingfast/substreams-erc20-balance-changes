use crate::pb::erc20::types::v1::{BalanceChanges};
use substreams::{errors::Error, pb::substreams::Clock};
use substreams_entity_change::pb::entity::EntityChanges;
use substreams_database_change::pb::database::DatabaseChanges;

#[substreams::handlers::map]
pub fn graph_out(clock: Clock, balance_changes: BalanceChanges) -> Result<EntityChanges, Error> {
    let mut tables = substreams_entity_change::tables::Tables::new();
    let block_num = clock.number.to_string();
    let timestamp = clock.timestamp.unwrap().seconds.to_string();

    for balance_change in balance_changes.balance_changes {
        if balance_change.change_type == 0 {
            continue;
        }

        let key = format!("{}:{}:{}", balance_change.contract, balance_change.owner, balance_change.transaction);

        tables
            .create_row("balance_change", key)
            .set("contract", balance_change.contract)
            .set("owner", balance_change.owner)
            .set("transfer_value", balance_change.transfer_value)
            .set("old_balance", balance_change.old_balance)
            .set("new_balance", balance_change.new_balance)
            .set("storage_key", balance_change.storage_key)
            .set("call_index", balance_change.call_index)
            .set("transaction", balance_change.transaction)
            .set("change_type", balance_change.change_type)
            .set_bigint("block_num", &block_num)
            .set_bigint("timestamp", &timestamp);
    }

    Ok(tables.to_entity_changes())
}

#[substreams::handlers::map]
pub fn db_out(clock: Clock, balance_changes: BalanceChanges) -> Result<DatabaseChanges, Error> {
    let mut tables = substreams_database_change::tables::Tables::new();
    let block_num = clock.number.to_string();
    let timestamp = clock.timestamp.unwrap().seconds.to_string();

    for balance_change in balance_changes.balance_changes {
        if balance_change.change_type == 0 {
            continue;
        }

        let key = format!("{}:{}:{}", balance_change.contract, balance_change.owner, balance_change.transaction);

        tables.create_row("balance_changes", key)
            .set("contract", balance_change.contract)
            .set("owner", balance_change.owner)
            .set("amount", balance_change.transfer_value)
            .set("old_balance", balance_change.old_balance)
            .set("new_balance", balance_change.new_balance)
            .set("transaction_id", balance_change.transaction)
            .set("block_num", &block_num)
            .set("timestamp", &timestamp);
    }

    Ok(tables.to_database_changes())
}
