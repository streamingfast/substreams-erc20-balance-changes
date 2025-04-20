mod db_out;
mod events;
mod pb;
mod transactions;

use pb::evm::erc721::events::v1::{Events, Mints, Token, Transfer};
use std::collections::{HashMap, HashSet};
use substreams::scalar::BigInt;
use substreams_abis::evm::token::erc721::functions;
use substreams_ethereum::{pb::eth::v2 as eth, rpc::RpcBatch};

/// Extracts events events from the logs
#[substreams::handlers::map]
fn map_events(blk: eth::Block) -> Result<Events, substreams::errors::Error> {
    let transfers: Vec<Transfer> = events::get_all(&blk).collect();

    // Collect all transaction hashes involved in any ERC721 event
    let event_tx_hashes: std::collections::HashSet<Vec<u8>> = transfers.iter().map(|t| t.tx_hash.to_vec()).collect();

    let transactions = transactions::get_transactions(&blk, &event_tx_hashes);

    Ok(Events { transfers, transactions })
}

/// Extracts mints with uri, symbol and name from the logs
/// We do this in a separate module to avoid re-processing RPC calls if we change something in map_events
#[substreams::handlers::map]
fn map_mints(blk: eth::Block) -> Result<Mints, substreams::errors::Error> {
    let mints: Vec<Transfer> = events::get_mints(&blk).collect();
    if mints.is_empty() {
        return Ok(Mints { tokens: vec![] });
    }
    let contracts: HashSet<_> = mints.iter().map(|m| &m.contract).collect();
    let mut symbols: HashMap<Vec<u8>, (Option<String>, Option<String>)> = HashMap::new();
    let mut batch = RpcBatch::new();
    for address in &contracts {
        batch = batch.add(functions::Name {}, address.to_vec()).add(functions::Symbol {}, address.to_vec());
    }
    let responses = batch.execute().expect("failed to execute rpc batch").responses;
    for (i, address) in contracts.iter().enumerate() {
        let name = RpcBatch::decode::<String, functions::Name>(&responses[i * 2]);
        let symbol = RpcBatch::decode::<String, functions::Symbol>(&responses[i * 2 + 1]);
        symbols.insert(address.to_vec(), (symbol, name));
    }

    let mint_keys: Vec<(&Vec<u8>, &String)> = mints.iter().map(|mint| (&mint.contract, &mint.token_id)).collect();
    let mut uris: HashMap<(&Vec<u8>, &String), Option<String>> = HashMap::new();
    for chunk in mint_keys.chunks(50) {
        let mut batch = RpcBatch::new();
        for (contract, token_id) in chunk {
            let token_id = token_id.parse::<BigInt>().expect("invalid token_id");
            batch = batch.add(functions::TokenUri { token_id }, contract.to_vec());
        }
        let responses = batch.execute().unwrap().responses;
        for (i, (contract, token_id)) in chunk.iter().enumerate() {
            let uri = RpcBatch::decode::<String, functions::TokenUri>(&responses[i]);
            uris.insert((contract, token_id), uri);
        }
    }

    let tokens = mints
        .iter()
        .map(|mint| {
            let (symbol, name) = symbols.get(&mint.contract).cloned().unwrap_or((None, None));
            let uri = uris.get(&(&mint.contract, &mint.token_id)).cloned().unwrap_or(None);
            Token {
                uri,
                symbol,
                name,
                contract: mint.contract.clone(),
                token_id: mint.token_id.clone(),
            }
        })
        .collect::<Vec<_>>();

    substreams::log::info!("contracts: {}, mints: {}", contracts.len(), tokens.len());

    Ok(Mints { tokens })
}
