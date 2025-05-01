mod events;

use proto::pb::evm::erc721::events::v1::{Events, Tokens};
use substreams_ethereum::pb::eth::v2 as eth;

/// Extracts events events from the logs
#[substreams::handlers::map]
fn map_events(blk: eth::Block, tokens: Tokens) -> Result<Events, substreams::errors::Error> {
    Ok(Events {
        transfers: events::get_transfers(&blk).collect(),
        tokens: tokens.tokens,
    })
}
