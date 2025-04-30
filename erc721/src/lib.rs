mod events;

use common::is_zero_address;
use proto::pb::evm::erc721::events::v1::{Events, Mints, Token, Transfer};
use std::collections::HashMap;
use substreams_ethereum::pb::eth::v2 as eth;

/// Extracts events events from the logs
#[substreams::handlers::map]
fn map_events(blk: eth::Block, mints: Mints) -> Result<Events, substreams::errors::Error> {
    let mut transfers: Vec<Transfer> = events::get_transfers(&blk).collect();

    // Merge metadata from mints into transfers
    merge_metadata(&mut transfers, &mints.tokens);

    Ok(Events { transfers })
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
