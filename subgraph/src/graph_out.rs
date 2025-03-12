use erc20::utils::clock_to_date;
use proto::pb::evm::tokens::types::v1::Events;
use substreams::{errors::Error, pb::substreams::Clock};
use substreams_entity_change::pb::entity::EntityChanges;

#[substreams::handlers::map]
pub fn graph_out(clock: Clock, erc20: Events, native: Events) -> Result<EntityChanges, Error> {
    let mut tables = substreams_entity_change::tables::Tables::new();

    // merge erc20 + native events
    let events = Events {
        balance_changes: erc20.balance_changes.into_iter().chain(native.balance_changes).collect(),
        transfers: erc20.transfers.into_iter().chain(native.transfers).collect(),
        contracts: vec![],
    };

    for balance_change in events.balance_changes {
        tables
            .create_row("Balance", format!("{}:{}", balance_change.contract, balance_change.owner))
            // -- block --
            .set_bigint("block_num", &clock.number.to_string())
            .set("timestamp", clock.timestamp.expect("missing timestamp"))
            .set("date", clock_to_date(&clock))

            // -- balance --
            .set("contract", balance_change.contract)
            .set("owner", balance_change.owner)
            .set_bigint("balance", &balance_change.new_balance);
    }

    Ok(tables.to_entity_changes())
}
