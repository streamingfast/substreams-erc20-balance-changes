mod db_out;
mod events;
mod pb;
mod transactions;

use pb::evm::erc721::events::v1::{Events, Mints, Token, Transfer};
use substreams::scalar::BigInt;
use substreams_abis::evm::token::erc721;
use substreams_ethereum::pb::eth::v2 as eth;

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
    let mints = events::get_mints(&blk)
        .map(|mint| {
            let token_id = mint.token_id.parse::<BigInt>().expect("invalid token_id");
            let uri = get_uri(mint.contract.clone().into(), token_id);
            let symbol = get_symbol(mint.contract.clone().into());
            let name = get_name(mint.contract.clone().into());
            Token {
                uri,
                symbol,
                name,
                contract: mint.contract,
                token_id: mint.token_id,
            }
        })
        .collect();

    Ok(Mints { tokens: mints })
}

fn get_uri(address: Vec<u8>, token_id: BigInt) -> Option<String> {
    erc721::functions::TokenUri { token_id }.call(address)
}

fn get_symbol(address: Vec<u8>) -> Option<String> {
    erc721::functions::Symbol {}.call(address)
}

fn get_name(address: Vec<u8>) -> Option<String> {
    erc721::functions::Name {}.call(address)
}
