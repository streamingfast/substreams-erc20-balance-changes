use common::bytes_to_hex;
use proto::pb::evm::tokens::contracts::types::v1::ContractChange;
use substreams::pb::substreams::Clock;

use crate::common::{common_key, set_clock};

// Helper function to process a single contract_changes
pub fn process_erc20_contract_change(tables: &mut substreams_database_change::tables::Tables, clock: &Clock, event: ContractChange) {
    let key = common_key(clock, event.ordinal);
    let row = tables
        .create_row("contract_changes", key)
        // -- transaction --
        .set("transaction_id", bytes_to_hex(&event.transaction_id))
        .set("from", bytes_to_hex(&event.from))
        .set("to", bytes_to_hex(&event.to))

        // -- ordering --
        .set("ordinal", event.ordinal)
        .set("index", event.index)
        .set("global_sequence", event.global_sequence)

        // -- contract --
        .set("address", &bytes_to_hex(&event.address))
        .set("name", &event.name)
        .set("symbol", &event.symbol)
        .set("decimals", event.decimals.to_string())

        // -- debug --
        .set("algorithm", event.algorithm().as_str_name())
        .set("algorithm_code", event.algorithm.to_string());

    set_clock(clock, row);
}
