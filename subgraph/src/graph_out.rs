use common::clock_to_date;
use proto::pb::evm::tokens::types::v1::Events;
use substreams::{errors::Error, pb::substreams::Clock, Hex};
use substreams_entity_change::pb::entity::EntityChanges;

#[substreams::handlers::map]
pub fn graph_out(clock: Clock, erc20: Events, native: Events) -> Result<EntityChanges, Error> {
    let mut tables = substreams_entity_change::tables::Tables::new();

    // merge erc20 + native events
    let events = Events {
        balance_changes: erc20.balance_changes.into_iter().chain(native.balance_changes).collect(),
        transfers: vec![],
        contracts: vec![],
    };

    for balance_change in events.balance_changes {
        let contract = bytes_to_hex(&balance_change.contract);
        let owner = bytes_to_hex(&balance_change.owner);
        tables
            .create_row("Balance", format!("{}:{}", contract, owner))
            // -- block --
            .set_bigint("block_num", &clock.number.to_string())
            .set("timestamp", clock.timestamp.expect("missing timestamp"))
            .set("date", clock_to_date(&clock))

            // -- balance --
            .set("contract", contract)
            .set("owner", owner)
            .set_bigint("balance", &balance_change.new_balance);
    }

    Ok(tables.to_entity_changes())
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