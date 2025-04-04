use proto::pb::evm::tokens::contracts::v1::{Events, ContractChange};
use substreams::errors::Error;
use substreams_ethereum::pb::eth::v2::Block;

use crate::metadata::get_metadata;

#[substreams::handlers::map]
pub fn map_events(block: Block) -> Result<Events, Error> {
    let mut events = Events::default();

    for trx in block.transactions() {
        for call_view in trx.calls() {
            let call = call_view.call;

            match get_metadata(call) {
                Some(result) => {
                    events.contract_changes.push(ContractChange {
                        // -- transaction --
                        transaction_id: Some(trx.hash.to_vec()),

                        // -- call --
                        caller: Some(call.caller.to_vec()),

                        // -- ordering --
                        ordinal: Some(call.begin_ordinal),

                        // -- contract --
                        address: call.address.to_vec(),
                        name: result.name,
                        symbol: result.symbol,
                        decimals: result.decimals,
                    });
                }
                _ => {}
            }
        }
    }
    Ok(events)
}
