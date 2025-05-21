mod calls;
use std::collections::{HashMap, HashSet};

use common::{is_zero_address, Address};
use proto::pb::evm::erc1155::metadata::v1 as erc1155;
use proto::pb::evm::erc1155::v1 as events;
use substreams::log;

use crate::calls::{batch_name, batch_symbol, batch_uri};

#[substreams::handlers::map]
fn map_events(params: String, erc1155_transfers: events::Events) -> Result<erc1155::Events, substreams::errors::Error> {
    let chunk_size = params.parse::<usize>().expect("Failed to parse chunk size");
    let mut events = erc1155::Events::default();

    // Collect unique contracts and token IDs
    let mut contracts: HashSet<&Address> = HashSet::new();
    let mut contracts_by_token_id: HashSet<(&Address, &String)> = HashSet::new();

    // TransferSingle
    for transfer in &erc1155_transfers.transfers_single {
        // EIP-1155 rule – “When minting/creating tokens, the _from argument MUST be set to 0x0 (i.e. zero address).”
        // https://eips.ethereum.org/EIPS/eip-1155
        if !is_zero_address(&transfer.from) {
            continue;
        }
        contracts.insert(&transfer.contract);
        contracts_by_token_id.insert((&transfer.contract, &transfer.id));
    }
    // TransferBatch
    for transfer in &erc1155_transfers.transfers_batch {
        if !is_zero_address(&transfer.from) {
            continue;
        }
        contracts.insert(&transfer.contract);
        for id in &transfer.ids {
            contracts_by_token_id.insert((&transfer.contract, id));
        }
    }

    log::info!("\ncontracts={}\ncontracts_by_token_id={}", contracts.len(), contracts_by_token_id.len());

    // Fetch RPC calls for tokens
    let contract_vec: Vec<&Address> = contracts.into_iter().collect();
    let contract_by_token_id_vec: Vec<(&Address, &String)> = contracts_by_token_id.into_iter().collect();

    // ERC-721
    let mut symbols: HashMap<&Address, String> = batch_symbol(&contract_vec, chunk_size);
    let mut names: HashMap<&Address, String> = batch_name(&contract_vec, chunk_size);

    // ERC-1155
    let mut uris: HashMap<(&Address, &String), String> = batch_uri(&contract_by_token_id_vec, chunk_size);

    // Metadata By Token
    for &(contract, token_id) in &contract_by_token_id_vec {
        // Requires URI to be set
        if let Some(uri) = uris.remove(&(contract, token_id)) {
            events.metadata_by_tokens.push(erc1155::MetadataByToken {
                contract: contract.to_vec(),
                token_id: token_id.to_string(),
                uri,
            });
        }
    }

    // Metadata By Contract
    for &contract in &contract_vec {
        // Skip if both symbol and name are None
        // This can happen if the contract is not an ERC-721 or ERC-1155 contract
        // or if the contract does not implement the symbol() or name() functions
        if symbols.get(contract).is_none() && names.get(contract).is_none() {
            continue;
        }
        // Add metadata to the events
        events.metadata_by_contracts.push(erc1155::MetadataByContract {
            contract: contract.to_vec(),
            symbol: symbols.remove(contract),
            name: names.remove(contract),
        });
    }

    Ok(events)
}
