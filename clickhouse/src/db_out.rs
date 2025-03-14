use std::vec;

use common::clock_to_date;
use proto::pb::evm::tokens::types::v1::Events;
use substreams::{errors::Error, pb::substreams::Clock, Hex};
use substreams_database_change::{pb::database::DatabaseChanges, tables::Row};

pub fn set_clock(clock: &Clock, row: &mut Row) {
    row
        .set("block_num", clock.number.to_string())
        .set("block_hash", format!("0x{}", &clock.id))
        .set("timestamp", clock.timestamp.expect("missing timestamp").seconds.to_string())
        .set("date", clock_to_date(&clock));
}

#[substreams::handlers::map]
pub fn db_out(clock: Clock, erc20: Events, native: Events, contracts: Events) -> Result<DatabaseChanges, Error> {
    let mut tables = substreams_database_change::tables::Tables::new();

    // merge erc20 + native events
    let events = Events {
        balance_changes: erc20.balance_changes.into_iter().chain(native.balance_changes).collect(),
        transfers: erc20.transfers.into_iter().chain(native.transfers).collect(),
        contracts: vec![],
    };

    for balance_change in events.balance_changes {
        let algorithm = balance_change.algorithm().as_str_name();
        let key = [
            ("date", clock_to_date(&clock)),
            ("block_num", clock.number.to_string()),
            ("ordinal", balance_change.ordinal.to_string()),
        ];
        let row = tables
            .create_row("balance_changes", key)
            // -- transaction --
            .set("transaction_id", bytes_to_hex(&balance_change.transaction_id))

            // -- ordinal --
            .set("ordinal", balance_change.ordinal)
            .set("global_sequence", balance_change.global_sequence)

            // -- balance change --
            .set("contract", bytes_to_hex(&balance_change.contract))
            .set("owner", bytes_to_hex(&balance_change.owner))
            .set("old_balance", balance_change.old_balance)
            .set("new_balance", balance_change.new_balance)

            // -- debug --
            .set("algorithm", algorithm)
            .set("algorithm_code", balance_change.algorithm);

        set_clock(&clock, row);
    }

    for transfer in events.transfers {
        let algorithm = transfer.algorithm().as_str_name();
        let key = [
            ("date", clock_to_date(&clock)),
            ("block_num", clock.number.to_string()),
            ("ordinal", transfer.ordinal.to_string()),
        ];
        let row = tables.create_row("transfers", key)
            // -- transaction --
            .set("transaction_id", bytes_to_hex(&transfer.transaction_id))

            // -- ordering --
            .set("ordinal", transfer.ordinal)
            .set("global_sequence", transfer.global_sequence)

            // -- transfer --
            .set("contract", bytes_to_hex(&transfer.contract))
            .set("from", bytes_to_hex(&transfer.from))
            .set("to", bytes_to_hex(&transfer.to))
            .set("value", &transfer.value)

            // -- debug --
            .set("algorithm", algorithm)
            .set("algorithm_code", transfer.algorithm);

        set_clock(&clock, row);
    }

    for contract in contracts.contracts {
        let algorithm = contract.algorithm().as_str_name();
        let key = [
            ("date", clock_to_date(&clock)),
            ("block_num", clock.number.to_string()),
            ("ordinal", contract.ordinal.to_string()),
        ];
        let row = tables.create_row("contract_changes", key)
            // -- transaction --
            .set("transaction_id", bytes_to_hex(&contract.transaction_id))
            .set("from", bytes_to_hex(&contract.from))
            .set("to", bytes_to_hex(&contract.to))

            // -- ordering --
            .set("ordinal", contract.ordinal)
            .set("global_sequence", contract.global_sequence)

            // -- contract --
            .set("address", bytes_to_hex(&contract.address))
            .set("name", &contract.name)
            .set("symbol", &contract.symbol)
            .set("decimals", &contract.decimals.to_string())

            // -- debug --
            .set("algorithm", algorithm)
            .set("algorithm_code", contract.algorithm);

        set_clock(&clock, row);
    }

    Ok(tables.to_database_changes())
}

pub fn bytes_to_hex(bytes: &Vec<u8>) -> String {
    if bytes.is_empty() {
        return "".to_string();
    } else if "native".to_string().into_bytes() == *bytes {
        return "native".to_string();
    } else {
        format! {"0x{}", Hex::encode(bytes)}.to_string()
    }
}