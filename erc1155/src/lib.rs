mod events;

use proto::pb::evm::erc1155::events::v1::{Events, Token, Transfer};
use substreams_ethereum::pb::eth::v2 as eth;

/// Extracts events events from the logs
#[substreams::handlers::map]
fn map_events(blk: eth::Block) -> Result<Events, substreams::errors::Error> {
    let transfers: Vec<Transfer> = events::get_transfers(&blk).collect();
    let tokens: Vec<Token> = events::get_tokens(&blk).collect();

    Ok(Events { transfers, tokens })
}
