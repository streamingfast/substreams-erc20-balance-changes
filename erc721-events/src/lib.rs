mod db_out;
mod events;
mod pb;
mod transactions;

use pb::events::{Burn, Events, Mint, Mints, Transfer};
use substreams::scalar::BigInt;
use substreams_abis::evm::token::erc721;
use substreams_ethereum::pb::eth::v2 as eth;

/// Extracts events events from the logs
#[substreams::handlers::map]
fn map_events(blk: eth::Block) -> Result<Events, substreams::errors::Error> {
    let transfers: Vec<Transfer> = events::get_transfers(&blk).collect();
    let mints: Vec<Mint> = events::get_mints(&blk).collect();
    let burns: Vec<Burn> = events::get_burns(&blk).collect();

    // Collect all transaction hashes involved in any ERC721 event
    let event_tx_hashes: std::collections::HashSet<Vec<u8>> = transfers
        .iter()
        .map(|t| t.tx_hash.to_vec())
        .chain(mints.iter().map(|m| m.tx_hash.to_vec()))
        .chain(burns.iter().map(|b| b.tx_hash.to_vec()))
        .collect();

    let transactions = transactions::get_transactions(&blk, &event_tx_hashes);

    Ok(Events {
        transfers,
        mints,
        burns,
        transactions,
    })
}

/// Extracts mints with uri from the logs
/// We do this in a separate module to avoid re-processing RPC calls if we change something in map_events
#[substreams::handlers::map]
fn map_mints_with_uri(blk: eth::Block) -> Result<Mints, substreams::errors::Error> {
    let mints = events::get_mints(&blk)
        .map(|mint| {
            let token_id = mint.token_id.parse::<BigInt>().expect("invalid token_id");
            let uri = get_uri(mint.contract.clone().into(), token_id);
            Mint { uri, ..mint }
        })
        .collect();

    Ok(Mints { mints })
}

fn get_uri(address: Vec<u8>, token_id: BigInt) -> Option<String> {
    erc721::functions::TokenUri { token_id }.call(address)
}
