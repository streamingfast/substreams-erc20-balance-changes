use common::{bytes_to_hex, clock_to_date};
use proto::pb::evm::tokens::balances::types::v1::{BalanceChange, Events, Transfer};
use proto::pb::evm::tokens::contracts::types::v1::Events as EventsContracts;
use proto::pb::evm::tokens::prices::types::v1::Events as EventsPrices;
use substreams::{errors::Error, pb::substreams::Clock};
use substreams_database_change::{pb::database::DatabaseChanges, tables::Row};

// Helper function to set clock data in a row
pub fn set_clock(clock: &Clock, row: &mut Row) {
    row.set("block_num", clock.number.to_string())
        .set("block_hash", format!("0x{}", &clock.id))
        .set("timestamp", clock.timestamp.expect("missing timestamp").seconds.to_string())
        .set("date", clock_to_date(&clock));
}

#[substreams::handlers::map]
pub fn db_out(clock: Clock, erc20: Events, native: Events, contracts: EventsContracts, prices: EventsPrices) -> Result<DatabaseChanges, Error> {
    let mut tables = substreams_database_change::tables::Tables::new();

    // Pre-compute frequently used values
    let block_num = clock.number.to_string();
    let date = clock_to_date(&clock);

    // -- balance changes --
    // Process ERC-20 balance changes
    for event in erc20.balance_changes {
        process_balance_change(&mut tables, &clock, &block_num, &date, event);
    }

    // Process native balance changes
    for event in native.balance_changes {
        process_balance_change(&mut tables, &clock, &block_num, &date, event);
    }

    // -- transfers --
    // Process ERC-20 transfers
    for event in erc20.transfers {
        process_transfer(&mut tables, &clock, &block_num, &date, event);
    }

    // Process native transfers
    for event in native.transfers {
        process_transfer(&mut tables, &clock, &block_num, &date, event);
    }

    // -- contract changes --
    for event in contracts.contracts {
        let address = bytes_to_hex(&event.address);
        let key = [("address", address.to_string()), ("block_num", clock.number.to_string())];
        set_clock(
            &clock,
            tables
                .create_row("contract_changes", key)
                // -- contract --
                .set("address", &address)
                .set("name", &event.name)
                .set("symbol", &event.symbol)
                .set("decimals", &event.decimals.to_string()),
        );
    }

    // -- contract creations --
    for event in contracts.contract_creations {
        let address = bytes_to_hex(&event.address);
        let key = [("address", address.to_string())];
        set_clock(
            &clock,
            tables
                .create_row("contract_creations", key)
                // -- transaction --
                .set("transaction_id", bytes_to_hex(&event.transaction_id))
                .set("from", bytes_to_hex(&event.from))
                .set("to", bytes_to_hex(&event.to))
                // -- contract --
                .set("address", &address),
        );
    }

    // -- prices swaps --
    for event in prices.swaps {
        let row = create_row_with_common_values(&mut tables, "swaps", &clock, &block_num, &date, event.ordinal);

        // -- transaction --
        row.set("transaction_id", bytes_to_hex(&event.transaction_id))
            // -- log --
            .set("address", bytes_to_hex(&event.address))
            // -- ordinal --
            .set("ordinal", event.ordinal)
            .set("global_sequence", event.global_sequence)
            // -- swaps --
            .set("amount0_in", &event.amount0_in.to_string())
            .set("amount0_out", &event.amount0_out.to_string())
            .set("amount1_in", &event.amount1_in.to_string())
            .set("amount1_out", &event.amount1_out.to_string())
            .set("sender", &bytes_to_hex(&event.sender))
            .set("to", &bytes_to_hex(&event.to));
    }

    // -- prices syncs --
    for event in prices.syncs {
        let row = create_row_with_common_values(&mut tables, "sync_changes", &clock, &block_num, &date, event.ordinal);

        // -- transaction --
        row.set("transaction_id", bytes_to_hex(&event.transaction_id))
            // -- log --
            .set("address", bytes_to_hex(&event.address))
            // -- ordinal --
            .set("ordinal", event.ordinal)
            .set("global_sequence", event.global_sequence)
            // -- log --
            .set("reserve0", &event.reserve0.to_string())
            .set("reserve1", &event.reserve1.to_string());
    }

    // -- prices created pairs --
    for event in prices.pairs_created {
        let key = [("factory", bytes_to_hex(&event.to)), ("pair", bytes_to_hex(&event.pair))];
        set_clock(
            &clock,
            tables
                .create_row("pairs_created", key)
                // -- transaction --
                .set("transaction_id", bytes_to_hex(&event.transaction_id))
                .set("creator", bytes_to_hex(&event.creator)) // trx.from
                .set("to", bytes_to_hex(&event.to))
                // -- log --
                .set("factory", bytes_to_hex(&event.factory)) // log.address
                // -- pair created --
                .set("token0", bytes_to_hex(&event.token0))
                .set("token1", bytes_to_hex(&event.token1))
                .set("pair", bytes_to_hex(&event.pair)),
        );
    }

    Ok(tables.to_database_changes())
}

// Helper function to create a row with common values
fn create_row_with_common_values<'a>(
    tables: &'a mut substreams_database_change::tables::Tables,
    table_name: &str,
    clock: &Clock,
    block_num: &str,
    date: &str,
    ordinal: u64,
) -> &'a mut Row {
    let ordinal_str = ordinal.to_string();
    let key = [("date", date.to_string()), ("block_num", block_num.to_string()), ("ordinal", ordinal_str)];

    let row = tables.create_row(table_name, key);
    set_clock(clock, row);
    row
}

// Helper function to process a single balance change
fn process_balance_change(tables: &mut substreams_database_change::tables::Tables, clock: &Clock, block_num: &str, date: &str, event: BalanceChange) {
    let algorithm = event.algorithm().as_str_name();
    let row = create_row_with_common_values(tables, "balance_changes", clock, block_num, date, event.ordinal);

    // -- transaction --
    row.set("transaction_id", bytes_to_hex(&event.transaction_id))
        // -- ordinal --
        .set("ordinal", event.ordinal)
        .set("global_sequence", event.global_sequence)
        // -- balance change --
        .set("contract", bytes_to_hex(&event.contract))
        .set("owner", bytes_to_hex(&event.owner))
        .set("old_balance", event.old_balance)
        .set("new_balance", event.new_balance)
        // -- debug --
        .set("algorithm", algorithm)
        .set("algorithm_code", event.algorithm);
}

// Helper function to process a single transfer
fn process_transfer(tables: &mut substreams_database_change::tables::Tables, clock: &Clock, block_num: &str, date: &str, event: Transfer) {
    let algorithm = event.algorithm().as_str_name();
    let row = create_row_with_common_values(tables, "transfers", clock, block_num, date, event.ordinal);

    // -- transaction --
    row.set("transaction_id", bytes_to_hex(&event.transaction_id))
        // -- ordering --
        .set("ordinal", event.ordinal)
        .set("global_sequence", event.global_sequence)
        // -- transfer --
        .set("contract", bytes_to_hex(&event.contract))
        .set("from", bytes_to_hex(&event.from))
        .set("to", bytes_to_hex(&event.to))
        .set("value", &event.value)
        // -- debug --
        .set("algorithm", algorithm)
        .set("algorithm_code", event.algorithm);
}
