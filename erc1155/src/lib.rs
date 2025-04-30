mod events;

use std::collections::HashMap;

use common::is_zero_address;
use proto::pb::evm::erc1155::events::v1::{Events, Transfer, Uri};
use substreams_ethereum::pb::eth::v2 as eth;

/// Extracts events events from the logs
#[substreams::handlers::map]
fn map_events(blk: eth::Block) -> Result<Events, substreams::errors::Error> {
    let transfers: Vec<Transfer> = events::get_transfers(&blk).collect();
    let uris: Vec<Uri> = events::get_uris(&blk).collect();

    Ok(Events { transfers, uris })
}
