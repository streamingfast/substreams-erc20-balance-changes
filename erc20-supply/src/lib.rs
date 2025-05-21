mod calls;

use std::collections::HashSet;

use common::Address;
use proto::pb::evm::erc20::supply::v1::{Events, TotalSupplyByContract};
use proto::pb::evm::erc20::transfers::v1::Events as ERC20Transfers;

use crate::calls::batch_total_supply;

#[substreams::handlers::map]
fn map_events(params: String, erc20: ERC20Transfers) -> Result<Events, substreams::errors::Error> {
    let mut events = Events::default();
    let chunk_size = params.parse::<usize>().expect("Failed to parse chunk_size");

    // Collect unique contracts
    let contracts: Vec<&Address> = erc20.transfers.iter().map(|t| &t.contract).collect::<HashSet<_>>().into_iter().collect();

    // Fetch RPC calls for tokens
    let total_supplies = batch_total_supply(&contracts, chunk_size);

    // Metadata By Contract
    for (contract, total_supply) in total_supplies {
        events.total_supply_by_contracts.push(TotalSupplyByContract {
            contract: contract.to_vec(),
            total_supply: total_supply.to_string(),
        });
    }

    Ok(events)
}
