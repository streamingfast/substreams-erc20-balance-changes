use substreams::errors::Error;

use substreams::pb::substreams::Clock;
use substreams_ethereum::pb::eth::v2::{Block};

#[substreams::handlers::map]
pub fn map_events(clock: Clock, block: Block) -> Result<Clock, Error> {
    Ok(clock)
}