use std::collections::{HashMap, HashSet};

use common::{is_zero_address, Address};
use proto::pb::evm::erc721::metadata::v1::{Events, MetadataByContract, MetadataByToken};
use proto::pb::evm::erc721::v1::{Events as ERC721Transfers, Transfer};
use substreams::log;
use substreams::scalar::BigInt;

use crate::calls::{batch_base_uri, batch_name, batch_symbol, batch_token_uri, batch_total_supply};

#[substreams::handlers::map]
fn map_events(params: String, erc721_transfers: ERC721Transfers) -> Result<Events, substreams::errors::Error> {
    let chunk_size = params.parse::<usize>().expect("Failed to parse chunk size");
    let mut events = Events::default();
    let mints: Vec<Transfer> = get_mints(erc721_transfers.transfers).collect();

    // Collect unique contracts and token IDs
    let mut contracts: HashSet<&Address> = HashSet::new();
    let mut contracts_by_token_id: HashSet<(&Address, &String)> = HashSet::new();
    for transfer in &mints {
        contracts.insert(&transfer.contract);
        contracts_by_token_id.insert((&transfer.contract, &transfer.token_id));
    }

    log::info!("\ncontracts={}\ncontracts_by_token_id={}", contracts.len(), contracts_by_token_id.len());

    // Fetch RPC calls for tokens and contracts
    let contract_vec: Vec<&Address> = contracts.into_iter().collect();
    let contract_by_token_id_vec: Vec<(&Address, &String)> = contracts_by_token_id.into_iter().collect();
    let mut symbols: HashMap<&Address, String> = batch_symbol(&contract_vec, chunk_size);
    let mut names: HashMap<&Address, String> = batch_name(&contract_vec, chunk_size);
    let mut base_uris: HashMap<&Address, String> = batch_base_uri(&contract_vec, chunk_size);
    let mut total_supplies: HashMap<&Address, BigInt> = batch_total_supply(&contract_vec, chunk_size);
    let mut uris: HashMap<(&Address, &String), String> = batch_token_uri(&contract_by_token_id_vec, chunk_size);

    // Metadata By Token events
    for transfer in &mints {
        events.metadata_by_tokens.push(MetadataByToken {
            contract: transfer.contract.to_vec(),
            token_id: transfer.token_id.to_string(),
            uri: uris.remove(&(&transfer.contract, &transfer.token_id)),
        });
    }

    // Metadata By Contract events
    for &contract in &contract_vec {
        events.metadata_by_contracts.push(MetadataByContract {
            contract: contract.to_vec(),
            symbol: symbols.remove(contract),
            name: names.remove(contract),
            base_uri: base_uris.remove(contract),
            total_supply: total_supplies.remove(contract).map(|value| value.to_string()),
        });
    }

    Ok(events)
}

pub fn get_mints<'a>(erc721_transfers: Vec<Transfer>) -> impl Iterator<Item = Transfer> + 'a {
    erc721_transfers.into_iter().filter(|transfer| !is_zero_address(&transfer.from))
}
