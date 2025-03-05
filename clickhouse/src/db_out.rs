use erc20_balances_transfers::pb::erc20::types::v1::Events;
use substreams::errors::Error;
use substreams_database_change::pb::database::DatabaseChanges;

#[substreams::handlers::map]
pub fn db_out(events: Events) -> Result<DatabaseChanges, Error> {
    let mut tables = substreams_database_change::tables::Tables::new();

    for balance_change in events.balance_changes {
        tables
            .create_row(
                "balance_changes",
                [
                    ("block_num", (balance_change).block_num.to_string()),
                    ("storage_ordinal", (balance_change).storage_ordinal.to_string()),
                ],
            )
            // -- block --
            .set("block_num", balance_change.block_num.to_string())
            .set("block_hash", balance_change.block_hash)
            .set("timestamp", balance_change.timestamp.expect("missing timestamp").seconds.to_string())
            .set("date", balance_change.date)
            // -- transaction --
            .set("transaction_id", balance_change.transaction_id)
            // -- call --
            .set("call_index", balance_change.call_index.to_string())
            // -- log --
            .set("log_index", balance_change.log_index.to_string())
            .set("log_block_index", balance_change.log_block_index.to_string())
            .set("log_ordinal", balance_change.log_ordinal.to_string())
            // -- storage --
            .set("storage_key", balance_change.storage_key)
            .set("storage_ordinal", balance_change.storage_ordinal.to_string())
            // -- balance change --
            .set("contract", balance_change.contract)
            .set("owner", balance_change.owner)
            .set("old_balance", balance_change.old_balance)
            .set("new_balance", balance_change.new_balance)
            // -- indexing --
            .set("version", balance_change.version)
            // -- debug --
            .set("balance_change_type", balance_change.balance_change_type.to_string());
    }

    for transfer in events.transfers {
        tables
            .create_row(
                "transfers",
                [
                    ("block_num", (transfer).block_num.to_string()),
                    ("log_block_index", (transfer).log_block_index.to_string()),
                ],
            )
            // -- block --
            .set("block_num", transfer.block_num.to_string())
            .set("block_hash", transfer.block_hash)
            .set("timestamp", transfer.timestamp.expect("missing timestamp").seconds.to_string())
            .set("date", transfer.date)
            // -- transaction --
            .set("transaction_id", transfer.transaction_id)
            // -- call --
            .set("call_index", transfer.call_index.to_string())
            .set("call_address", transfer.call_address.to_string())
            // -- log --
            .set("log_index", transfer.log_index.to_string())
            .set("log_block_index", transfer.log_block_index.to_string())
            .set("log_ordinal", transfer.log_ordinal.to_string())
            // -- transfer --
            .set("contract", transfer.contract)
            .set("from", transfer.from)
            .set("to", transfer.to)
            .set("value", transfer.value)
            // -- debug --
            .set("transfer_type", transfer.transfer_type.to_string());
    }

    Ok(tables.to_database_changes())
}
