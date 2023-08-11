use crate::pb::erc20::types::v1::{BalanceChanges};
use substreams::{errors::Error, pb::substreams::Clock};
use substreams_entity_change::pb::entity::EntityChanges;
use substreams_entity_change::tables::Tables;

#[substreams::handlers::map]
pub fn graph_out(clock: Clock, balance_changes: BalanceChanges) -> Result<EntityChanges, Error> {
    let mut tables = Tables::new();
    let block_num = clock.number.to_string();
    let timestamp = clock.timestamp.unwrap().seconds.to_string();

    for balance_change in balance_changes.balance_changes {
        let id = format!("{}:{}:{}", balance_change.contract, balance_change.owner, balance_change.transaction);

        tables
            .create_row("Balance", id)
            // contract address
            .set("address", balance_change.contract)
            // balance change
            .set("owner", balance_change.owner)
            .set("old_balance", balance_change.old_balance)
            .set("new_balance", balance_change.new_balance)
            // trace information
            .set("transaction", balance_change.transaction)
            .set_bigint("block_num", &block_num)
            .set_bigint("timestamp", &timestamp);
    }

    Ok(tables.to_entity_changes())
}
