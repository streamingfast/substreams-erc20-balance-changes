use proto::pb::evm::tokens::types::v1::Events;
use substreams::errors::Error;
use substreams_database_change::pb::database::DatabaseChanges;

#[substreams::handlers::map]
pub fn db_out(erc20: Events, native: Events) -> Result<DatabaseChanges, Error> {
    let mut tables = substreams_database_change::tables::Tables::new();

    // merge erc20 + native events
    let events = Events {
        balance_changes: erc20.balance_changes.into_iter().chain(native.balance_changes).collect(),
        transfers: erc20.transfers.into_iter().chain(native.transfers).collect(),
    };

    for balance_change in events.balance_changes {
        let algorithm = balance_change.algorithm().as_str_name();
        tables
            .create_row(
                "balance_changes",
                [
                    ("block_num", (balance_change).block_num.to_string()),
                    ("ordinal", (balance_change).ordinal.to_string()),
                ],
            )
            // -- block --
            .set("block_num", balance_change.block_num.to_string())
            .set("block_hash", balance_change.block_hash)
            .set("timestamp", balance_change.timestamp.expect("missing timestamp").seconds.to_string())
            .set("date", balance_change.date)

            // -- transaction --
            .set("transaction_id", balance_change.transaction_id)

            // -- ordinal --
            .set("ordinal", balance_change.ordinal)
            .set("global_sequence", balance_change.global_sequence)

            // -- balance change --
            .set("contract", balance_change.contract)
            .set("owner", balance_change.owner)
            .set("old_balance", balance_change.old_balance)
            .set("new_balance", balance_change.new_balance)

            // -- debug --
            .set("algorithm", algorithm)
            .set("algorithm_code", balance_change.algorithm);
    }

    for transfer in events.transfers {
        let algorithm = transfer.algorithm().as_str_name();
        tables
            .create_row(
                "transfers",
                [
                    ("block_num", (transfer).block_num.to_string()),
                    ("ordinal", (transfer).ordinal.to_string()),
                ],
            )
            // -- block --
            .set("block_num", transfer.block_num.to_string())
            .set("block_hash", transfer.block_hash)
            .set("timestamp", transfer.timestamp.expect("missing timestamp").seconds.to_string())
            .set("date", transfer.date)

            // -- transaction --
            .set("transaction_id", transfer.transaction_id)

            // -- indexing --
            .set("ordinal", transfer.ordinal)
            .set("global_sequence", transfer.global_sequence)

            // -- transfer --
            .set("contract", transfer.contract)
            .set("from", transfer.from)
            .set("to", transfer.to)
            .set("value", &transfer.value)

            // -- debug --
            .set("algorithm", algorithm)
            .set("algorithm_code", transfer.algorithm);
    }

    Ok(tables.to_database_changes())
}
