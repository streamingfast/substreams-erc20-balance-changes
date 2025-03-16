use common::clock_to_date;
use proto::pb::evm::tokens::types::v1::{BalanceChange, Events, Transfer};
use proto::pb::evm::tokens::contracts::types::v1::Events as EventsContracts;
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
pub fn db_out(clock: Clock, erc20: Events, native: Events, contracts: EventsContracts) -> Result<DatabaseChanges, Error> {
    let mut tables = substreams_database_change::tables::Tables::new();

    // -- combine events (ERC-20 + Native) --
    let balance_changes: Vec<BalanceChange> = erc20.balance_changes.into_iter().chain(native.balance_changes).collect();
    let transfers: Vec<Transfer> = erc20.transfers.into_iter().chain(native.transfers).collect();

    // -- balance changes --
    for balance_change in balance_changes {
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

    // -- transfers --
    for transfer in transfers {
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

    // -- contract changes --
    for contract in contracts.contracts {
        let address = bytes_to_hex(&contract.address);
        let key = [
            ("address", address.to_string()),
            ("block_num", clock.number.to_string()),
        ];
        let row = tables.create_row("contract_changes", key)
            // -- contract --
            .set("address", &address)
            .set("name", &contract.name)
            .set("symbol", &contract.symbol)
            .set("decimals", &contract.decimals.to_string());

        set_clock(&clock, row);
    }

    // -- contract creations --
    for row in contracts.contract_creations {
        let address = bytes_to_hex(&row.address);
        let key = [
            ("address", address.to_string()),
        ];
        let row = tables.create_row("contract_creations", key)
            // -- transaction --
            .set("transaction_id", bytes_to_hex(&row.transaction_id))
            .set("from", bytes_to_hex(&row.from))
            .set("to", bytes_to_hex(&row.to))

            // -- contract --
            .set("address", &address);

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