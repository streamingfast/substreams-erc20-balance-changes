mod abi;
mod calls;
mod functions;

use std::collections::HashSet;

use common::Address;
use proto::pb::evm::erc20::metadata::v1::{Events, MetadataChanges, MetadataInitialize};
use proto::pb::evm::erc20::stores::v1::Events as ERC20FirstTransfer;
use substreams_ethereum::pb::eth::v2::Block;

use crate::calls::{batch_decimals, batch_name, batch_symbol};
use crate::functions::get_metadata;

#[substreams::handlers::map]
fn map_events(chunk_size: String, block: Block, erc20: ERC20FirstTransfer) -> Result<Events, substreams::errors::Error> {
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

    // Extended blocks ONLY since we need to extract data from calls
    for trx in block.transactions() {
        for call_view in trx.calls() {
            let call = call_view.call;

            if let Some(result) = get_metadata(call) {
                events.metadata_changes.push(MetadataChanges {
                    // -- transaction --
                    tx_hash: trx.hash.to_vec(),

                    // -- call --
                    caller: call.caller.to_vec(),
                    address: call.address.to_vec(),
                    begin_ordinal: call.begin_ordinal,
                    end_ordinal: call.end_ordinal,

                    // -- function --
                    name: result.name,
                    symbol: result.symbol,
                });
            }
        }
    }

    Ok(events)
}
