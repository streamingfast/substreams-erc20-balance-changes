mod events;
mod transactions;

use std::collections::HashMap;

use common::is_zero_address;
use proto::pb::evm::erc1155::events::v1::{Events, Transfer, Uri};
use substreams_ethereum::pb::eth::v2 as eth;

/// Extracts events events from the logs
#[substreams::handlers::map]
fn map_events(blk: eth::Block) -> Result<Events, substreams::errors::Error> {
    let mut transfers: Vec<Transfer> = events::get_transfers(&blk).collect();
    let uris: Vec<Uri> = events::get_uris(&blk).collect();

    merge_metadata(&mut transfers, &uris);

    // Collect all transaction hashes involved in any ERC1155 event
    let event_tx_hashes: std::collections::HashSet<Vec<u8>> = transfers.iter().map(|t| t.tx_hash.to_vec()).collect();

    let transactions = transactions::get_transactions(&blk, &event_tx_hashes);

    Ok(Events { transfers, transactions, uris })
}

fn merge_metadata(transfers: &mut [Transfer], tokens: &[Uri]) {
    let mint_map: HashMap<(&[u8], &str), &Uri> = tokens.iter().map(|token| ((token.contract.as_ref(), token.token_id.as_str()), token)).collect();

    for transfer in transfers {
        if is_zero_address(&transfer.from) {
            if let Some(token) = mint_map.get(&(transfer.contract.as_ref(), &transfer.token_id)) {
                transfer.uri = Some(token.uri.clone());
            }
        }
    }
}
