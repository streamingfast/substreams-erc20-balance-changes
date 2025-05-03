use proto::pb::evm::seaport::v1::Events;
use substreams_ethereum::pb::eth::v2::Block;

use crate::events::insert_events;

#[substreams::handlers::map]
fn map_events(block: Block) -> Result<Events, substreams::errors::Error> {
    let mut events = Events::default();

    for trx in block.transactions() {
        for (log, call_view) in trx.logs_with_calls() {
            let call = call_view.call;
            insert_events(&mut events, trx, call, log);
        }
    }
    Ok(events)
}
