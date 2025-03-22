use common::{bytes_to_hex, clock_to_date};
use proto::pb::evm::tokens::balances::types::v1::{BalanceChange, Events, Transfer};
use proto::pb::evm::tokens::contracts::types::v1::{ContractChange, ContractCreation, Events as EventsContracts};
use proto::pb::evm::tokens::prices::uniswap::v2::types::v1::Events as EventsPricesUniswapV2;
use proto::pb::evm::tokens::prices::uniswap::v3::types::v1::Events as EventsPricesUniswapV3;
use substreams::{errors::Error, pb::substreams::Clock};
use substreams_database_change::{pb::database::DatabaseChanges, tables::Row};

// Helper function to set clock data in a row
pub fn set_clock(clock: &Clock, row: &mut Row) {
    row.set("block_num", clock.number.to_string())
        .set("block_hash", format!("0x{}", &clock.id))
        .set("timestamp", clock.timestamp.expect("missing timestamp").seconds.to_string())
        .set("date", clock_to_date(clock));
}

#[substreams::handlers::map]
pub fn db_out(clock: Clock, erc20: Events, erc20_rpc: Events, erc20_contracts: EventsContracts, erc20_contracts_rpc: EventsContracts, native: Events, prices_uniswap_v2: EventsPricesUniswapV2, prices_uniswap_v3: EventsPricesUniswapV3) -> Result<DatabaseChanges, Error> {
    let mut tables = substreams_database_change::tables::Tables::new();

    // Pre-compute frequently used values
    let block_num = clock.number.to_string();
    let date = clock_to_date(&clock);

    // -- balance changes --
    // Process ERC-20 balance changes
    for event in erc20.balance_changes {
        process_balance_change(&mut tables, &clock, &block_num, &date, event);
    }
    for event in erc20_rpc.balance_changes {
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
    for event in erc20_contracts.contract_changes {
        process_contract_change(&mut tables, &clock, &block_num, &date, event);
    }
    for event in erc20_contracts_rpc.contract_changes {
        process_contract_change(&mut tables, &clock, &block_num, &date, event);
    }

    // -- contract creations --
    for event in erc20_contracts.contract_creations {
        process_contract_creation(&mut tables, &clock, event);
    }
    for event in erc20_contracts_rpc.contract_creations {
        process_contract_creation(&mut tables, &clock, event);
    }

    // -- Uniswap V2: swaps --
    for event in prices_uniswap_v2.swaps {
        let row = create_row_with_common_values(&mut tables, "uniswap_v2_swaps", &clock, &block_num, &date, event.ordinal);

        // -- transaction --
        row.set("transaction_id", bytes_to_hex(&event.transaction_id))
            // -- log --
            .set("address", bytes_to_hex(&event.address))
            // -- ordering --
            .set("ordinal", event.ordinal)
            .set("index", event.index)
            .set("global_sequence", event.global_sequence)
            // -- swaps --
            .set("amount0_in", event.amount0_in)
            .set("amount0_out", event.amount0_out)
            .set("amount1_in", event.amount1_in)
            .set("amount1_out", event.amount1_out)
            .set("sender", bytes_to_hex(&event.sender))
            .set("to", bytes_to_hex(&event.to));
    }

    // -- Uniswap V2: syncs --
    for event in prices_uniswap_v2.syncs {
        let row = create_row_with_common_values(&mut tables, "uniswap_v2_sync_changes", &clock, &block_num, &date, event.ordinal);

        // -- transaction --
        row.set("transaction_id", bytes_to_hex(&event.transaction_id))
            // -- log --
            .set("address", bytes_to_hex(&event.address))
            // -- ordering --
            .set("ordinal", event.ordinal)
            .set("index", event.index)
            .set("global_sequence", event.global_sequence)
            // -- log --
            .set("reserve0", event.reserve0.to_string())
            .set("reserve1", event.reserve1.to_string());
    }

    // -- Uniswap V2: created pairs --
    for event in prices_uniswap_v2.pairs_created {
        let key = [("address", bytes_to_hex(&event.address)), ("pair", bytes_to_hex(&event.pair))];
        set_clock(
            &clock,
            tables
                .create_row("uniswap_v2_pairs_created", key)
                // -- transaction --
                .set("transaction_id", bytes_to_hex(&event.transaction_id))
                // -- log --
                .set("address", bytes_to_hex(&event.address)) // log.address
                // -- ordering --
                .set("ordinal", event.ordinal)
                .set("index", event.index)
                .set("global_sequence", event.global_sequence)
                // -- pair created --
                .set("token0", bytes_to_hex(&event.token0))
                .set("token1", bytes_to_hex(&event.token1))
                .set("pair", bytes_to_hex(&event.pair)),
        );
    }

    // -- Uniswap V3: swaps --
    for event in prices_uniswap_v3.swaps {
        let row = create_row_with_common_values(&mut tables, "uniswap_v3_swaps", &clock, &block_num, &date, event.ordinal);

        // -- transaction --
        row.set("transaction_id", bytes_to_hex(&event.transaction_id))
            // -- log --
            .set("address", bytes_to_hex(&event.address))
            // -- ordering --
            .set("ordinal", event.ordinal)
            .set("index", event.index)
            .set("global_sequence", event.global_sequence)
            // -- swaps --
            .set("amount0", event.amount0)
            .set("amount1", event.amount1)
            .set("sender", bytes_to_hex(&event.sender))
            .set("recipient", bytes_to_hex(&event.recipient))
            .set("liquidity", &event.liquidity)
            .set("sqrt_price_x96", &event.sqrt_price_x96)
            .set("tick", &event.tick.to_string());
    }

    // -- Uniswap V3: initialize --
    for event in prices_uniswap_v3.intializes {
        let key = [("address", bytes_to_hex(&event.address))];
        set_clock(
            &clock,
            tables
                .create_row("uniswap_v3_initializes", key)
                // -- transaction --
                .set("transaction_id", bytes_to_hex(&event.transaction_id))
                // -- log --
                .set("address", bytes_to_hex(&event.address)) // log.address
                // -- ordering --
                .set("ordinal", event.ordinal)
                .set("index", event.index)
                .set("global_sequence", event.global_sequence)
                // -- pair created --
                .set("sqrt_price_x96", &event.sqrt_price_x96.to_string())
                .set("tick", &event.tick.to_string())
            );
    }

    // -- Uniswap V3: pool created --
    for event in prices_uniswap_v3.pools_created {
        let key = [("address", bytes_to_hex(&event.address)), ("pool", bytes_to_hex(&event.pool))];
        set_clock(
            &clock,
            tables
                .create_row("uniswap_v3_pools_created", key)
                // -- transaction --
                .set("transaction_id", bytes_to_hex(&event.transaction_id))
                // -- log --
                .set("address", bytes_to_hex(&event.address)) // log.address
                // -- ordering --
                .set("ordinal", event.ordinal)
                .set("index", event.index)
                .set("global_sequence", event.global_sequence)
                // -- pair created --
                .set("token0", bytes_to_hex(&event.token0))
                .set("token1", bytes_to_hex(&event.token1))
                .set("pool", bytes_to_hex(&event.pool))
                .set("tick_spacing", event.tick_spacing.to_string())
                .set("fee", event.fee.to_string())
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
        // -- ordering --
        .set("ordinal", event.ordinal)
        .set("index", event.index)
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
        .set("index", event.index)
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

// Helper function to process a single contract_changes
fn process_contract_change(tables: &mut substreams_database_change::tables::Tables, clock: &Clock, block_num: &str, date: &str, event: ContractChange) {
    let address = bytes_to_hex(&event.address);
    let row = create_row_with_common_values(tables, "contract_changes", &clock, &block_num, &date, event.ordinal);

    // -- transaction --
    row
        // .set("transaction_id", bytes_to_hex(&event.transaction_id))
        // .set("from", bytes_to_hex(&event.from))
        // .set("to", bytes_to_hex(&event.to))

        // -- ordering --
        // .set("ordinal", event.ordinal)
        .set("index", event.index)
        .set("global_sequence", event.global_sequence)

        // -- contract --
        .set("address", &address)
        .set("name", &event.name)
        .set("symbol", &event.symbol)
        .set("decimals", event.decimals.to_string());

        // // -- debug --
        // .set("algorithm", event.algorithm().as_str_name())
        // .set("algorithm_code", event.algorithm.to_string());
}

// Helper function to process a single contract_changes
fn process_contract_creation(tables: &mut substreams_database_change::tables::Tables, clock: &Clock, event: ContractCreation) {
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

            // -- ordering --
            .set("ordinal", event.ordinal)
            .set("index", event.index)
            .set("global_sequence", event.global_sequence)

            // -- contract --
            .set("address", &address),
    );
}