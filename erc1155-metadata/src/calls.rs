use std::{collections::HashMap, str::FromStr};

use common::Address;
use substreams::{scalar::BigInt, Hex};
use substreams_abis::evm::token::erc1155;
use substreams_abis::evm::token::erc721;
use substreams_ethereum::rpc::RpcBatch;

// Returns the token collection name.
pub fn batch_name<'a>(contracts: &'a [&Address], chunk_size: usize) -> HashMap<&'a Address, String> {
    let mut results: HashMap<&'a Address, String> = HashMap::new();
    for chunks in contracts.chunks(chunk_size) {
        let batch = chunks
            .iter()
            .fold(RpcBatch::new(), |batch, address| batch.add(erc721::functions::Name {}, address.to_vec()));
        let responses = batch.execute().expect("failed to execute erc721::functions::Name RpcBatch").responses;
        for (i, address) in chunks.iter().enumerate() {
            if let Some(name) = RpcBatch::decode::<String, erc721::functions::Name>(&responses[i]) {
                if name.is_empty() {
                    continue;
                }
                results.insert(address, name);
            } else {
                substreams::log::info!("Failed to decode erc721::functions::Name for address={:?}", Hex::encode(address));
            }
        }
    }
    results
}

pub fn batch_symbol<'a>(contracts: &'a [&Address], chunk_size: usize) -> HashMap<&'a Address, String> {
    let mut results: HashMap<&'a Address, String> = HashMap::new();
    for chunks in contracts.chunks(chunk_size) {
        let batch = chunks
            .iter()
            .fold(RpcBatch::new(), |batch, address| batch.add(erc721::functions::Symbol {}, address.to_vec()));
        let responses = batch.execute().expect("failed to execute erc721::functions::Symbol RpcBatch").responses;
        for (i, address) in chunks.iter().enumerate() {
            if let Some(symbol) = RpcBatch::decode::<String, erc721::functions::Symbol>(&responses[i]) {
                if symbol.is_empty() {
                    substreams::log::info!("Empty symbol for address={:?}", Hex::encode(address));
                    continue;
                }
                results.insert(address, symbol);
            } else {
                substreams::log::info!("Failed to decode erc721::functions::Symbol for address={:?}", Hex::encode(address));
            }
        }
    }
    results
}

// Returns the token URI.
pub fn batch_uri<'a>(token_ids: &'a [(&'a Address, &'a String)], chunk_size: usize) -> HashMap<(&'a Address, &'a String), String> {
    let mut results: HashMap<(&'a Address, &'a String), String> = HashMap::with_capacity(token_ids.len());

    for chunk in token_ids.chunks(chunk_size) {
        let batch = chunk.iter().fold(RpcBatch::new(), |batch, (address, id)| {
            batch.add(
                erc1155::functions::Uri {
                    id: BigInt::from_str(id).unwrap(),
                },
                address.to_vec(),
            )
        });
        let responses = batch.execute().expect("failed to execute erc1155::functions::Uri batch").responses;
        for (i, (address, token_id)) in chunk.iter().enumerate() {
            if let Some(uri) = RpcBatch::decode::<String, erc1155::functions::Uri>(&responses[i]) {
                if !uri.is_empty() {
                    results.insert((address, token_id), parse_uri(&uri));
                    continue;
                }
            } else {
                substreams::log::info!("Failed to decode erc1155::Uri for address={:?} token_id={:?}", Hex::encode(address), token_id);
            }
        }
    }
    results
}

pub fn parse_uri(uri: &str) -> String {
    uri.trim_end_matches('\0').to_string()
}
