use common::{bytes_to_hex, clock_to_date};
use proto::pb::evm::tokens::balances::types::v1::{BalanceChange, Events, Transfer};
use proto::pb::evm::tokens::contracts::types::v1::Events as EventsContracts;
use proto::pb::evm::tokens::prices::types::v1::Events as EventsPrices;
use substreams::{errors::Error, pb::substreams::Clock};
use substreams_database_change::{pb::database::DatabaseChanges, tables::Row};

pub fn set_clock(clock: &Clock, row: &mut Row) {
    row
        .set("block_num", clock.number.to_string())
        .set("block_hash", format!("0x{}", &clock.id))
        .set("timestamp", clock.timestamp.expect("missing timestamp").seconds.to_string())
        .set("date", clock_to_date(&clock));
}

#[substreams::handlers::map]
pub fn db_out(clock: Clock, erc20: Events, native: Events, contracts: EventsContracts, prices: EventsPrices) -> Result<DatabaseChanges, Error> {
    let mut tables = substreams_database_change::tables::Tables::new();

    // -- combine events (ERC-20 + Native) --
    let balance_changes: Vec<BalanceChange> = erc20.balance_changes.into_iter().chain(native.balance_changes).collect();
    let transfers: Vec<Transfer> = erc20.transfers.into_iter().chain(native.transfers).collect();

    // -- balance changes --
    for event in balance_changes {
        let algorithm = event.algorithm().as_str_name();
        let key = [
            ("date", clock_to_date(&clock)),
            ("block_num", clock.number.to_string()),
            ("ordinal", event.ordinal.to_string()),
        ];
        set_clock(&clock, tables
            .create_row("balance_changes", key)
            // -- transaction --
            .set("transaction_id", bytes_to_hex(&event.transaction_id))

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
            .set("algorithm_code", event.algorithm)
        );
    }

    // -- transfers --
    for event in transfers {
        let algorithm = event.algorithm().as_str_name();
        let key = [
            ("date", clock_to_date(&clock)),
            ("block_num", clock.number.to_string()),
            ("ordinal", event.ordinal.to_string()),
        ];
        set_clock(&clock, tables.create_row("transfers", key)
            // -- transaction --
            .set("transaction_id", bytes_to_hex(&event.transaction_id))

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
            .set("algorithm_code", event.algorithm)
        );
    }

    // -- contract changes --
    for event in contracts.contracts {
        let address = bytes_to_hex(&event.address);
        let key = [
            ("address", address.to_string()),
            ("block_num", clock.number.to_string()),
        ];
        set_clock(&clock, tables.create_row("contract_changes", key)
            // -- contract --
            .set("address", &address)
            .set("name", &event.name)
            .set("symbol", &event.symbol)
            .set("decimals", &event.decimals.to_string())
        );
    }

    // -- contract creations --
    for event in contracts.contract_creations {
        let address = bytes_to_hex(&event.address);
        let key = [
            ("address", address.to_string()),
        ];
        set_clock(&clock, tables.create_row("contract_creations", key)
            // -- transaction --
            .set("transaction_id", bytes_to_hex(&event.transaction_id))
            .set("from", bytes_to_hex(&event.from))
            .set("to", bytes_to_hex(&event.to))

            // -- contract --
            .set("address", &address)
        );
    }

    // -- prices swaps --
    for event in prices.swaps {
        let key = [
            ("date", clock_to_date(&clock)),
            ("block_num", clock.number.to_string()),
            ("ordinal", event.ordinal.to_string()),
        ];
        set_clock(&clock, tables.create_row("swaps", key)
            // -- transaction --
            .set("transaction_id", bytes_to_hex(&event.transaction_id))
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
            .set("to", &bytes_to_hex(&event.to))
        );
    }

        // -- prices syncs --
        for event in prices.syncs {
            let key = [
                ("date", clock_to_date(&clock)),
                ("block_num", clock.number.to_string()),
                ("ordinal", event.ordinal.to_string()),
            ];
            set_clock(&clock, tables.create_row("syncs", key)
                // -- transaction --
                .set("transaction_id", bytes_to_hex(&event.transaction_id))
                // -- log --
                .set("address", bytes_to_hex(&event.address))
                // -- ordinal --
                .set("ordinal", event.ordinal)
                .set("global_sequence", event.global_sequence)
                // -- log --
                .set("reserve0", &event.reserve0.to_string())
                .set("reserve1", &event.reserve1.to_string())
            );
        }

        // -- prices created pairs --
        for event in prices.pairs_created {
            let key = [
                ("factory", bytes_to_hex(&event.to)),
                ("pair", bytes_to_hex(&event.pair)),
            ];
            set_clock(&clock, tables.create_row("pairs_created", key)
                // -- transaction --
                .set("transaction_id", bytes_to_hex(&event.transaction_id))
                .set("creator", bytes_to_hex(&event.creator)) // trx.from
                .set("to", bytes_to_hex(&event.to))
                // -- log --
                .set("factory", bytes_to_hex(&event.factory)) // log.address
                // -- pair created --
                .set("token0", bytes_to_hex(&event.token0))
                .set("token1", bytes_to_hex(&event.token1))
                .set("pair", bytes_to_hex(&event.pair))
            );
        }

    Ok(tables.to_database_changes())
}
