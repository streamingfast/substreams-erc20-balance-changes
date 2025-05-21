mod calls;

use std::collections::HashSet;

use common::Address;
use proto::pb::evm::erc20::metadata::v1::{Events, MetadataInitialize};
use proto::pb::evm::erc20::stores::v1::Events as ERC20FirstTransfer;

use crate::calls::{batch_decimals, batch_name, batch_symbol};

#[substreams::handlers::map]
fn map_events(chunk_size: String, erc20: ERC20FirstTransfer) -> Result<Events, substreams::errors::Error> {
    let mut events = Events::default();
    let chunk_size = chunk_size.parse::<usize>().expect("Failed to parse chunk_size");

    let contracts: Vec<&Address> = erc20
        .first_transfer_by_contract
        .iter()
        .map(|transfer| &transfer.contract)
        .collect::<HashSet<_>>()
        .into_iter()
        .collect();

    // Fetch RPC calls for tokens
    let mut symbols = batch_symbol(&contracts, chunk_size);
    let mut names = batch_name(&contracts, chunk_size);
    let mut decimals = batch_decimals(&contracts, chunk_size);

    // Metadata By Contract
    for contract in &contracts {
        // decimals is REQUIRED
        if let Some(decimals) = decimals.remove(contract) {
            events.metadata_initialize.push(MetadataInitialize {
                address: contract.to_vec(),
                decimals, // decimals is REQUIRED for initialization
                symbol: symbols.remove(contract),
                name: names.remove(contract),
            });
        }
    }

    Ok(events)
}
