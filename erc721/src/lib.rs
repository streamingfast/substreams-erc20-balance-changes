mod events;
pub mod pb;
mod transactions;

use common::is_zero_address;
use pb::evm::erc721::events::v1::{Events, Transfer};
use pb::evm::erc721::mints::v1::{Mints, Token};
use std::collections::HashMap;
use substreams_ethereum::pb::eth::v2 as eth;

/// Extracts events events from the logs
#[substreams::handlers::map]
fn map_events(blk: eth::Block, mints: Mints) -> Result<Events, substreams::errors::Error> {
    let mut transfers: Vec<Transfer> = events::get_transfers(&blk).collect();

    // Merge metadata from mints into transfers
    merge_metadata(&mut transfers, &mints.tokens);

    // Collect all transaction hashes involved in any ERC721 event
    let event_tx_hashes: std::collections::HashSet<Vec<u8>> = transfers.iter().map(|t| t.tx_hash.to_vec()).collect();

    let transactions = transactions::get_transactions(&blk, &event_tx_hashes);

    Ok(Events { transfers, transactions })
}

fn merge_metadata(transfers: &mut [Transfer], tokens: &[Token]) {
    let mint_map: HashMap<(&[u8], &str), &Token> = tokens.iter().map(|token| ((token.contract.as_ref(), token.token_id.as_str()), token)).collect();

    for transfer in transfers {
        if is_zero_address(&transfer.from) {
            if let Some(token) = mint_map.get(&(transfer.contract.as_ref(), &transfer.token_id)) {
                transfer.uri = token.uri.clone();
                transfer.symbol = token.symbol.clone();
                transfer.name = token.name.clone();
            }
        }
    }
}
