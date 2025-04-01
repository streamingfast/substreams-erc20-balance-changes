use common::to_global_sequence;
use proto::pb::evm::tokens::erc20::contracts::v1::{Events, ContractChange};
use substreams::errors::Error;
use substreams::pb::substreams::Clock;
use substreams_ethereum::pb::eth::v2::Block;

use crate::metadata::get_metadata;

#[substreams::handlers::map]
pub fn map_events(clock: Clock, block: Block) -> Result<Events, Error> {
    let mut events = Events::default();
    let mut index = 0;

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
                        index,
                        global_sequence: to_global_sequence(&clock, call.index.into()),

                        // -- contract --
                        address: call.address.to_vec(),
                        name: result.name,
                        symbol: result.symbol,
                        decimals: result.decimals,
                    });
                    index += 1;
                }
                _ => {}
            }
        }
    }
    Ok(events)

    // TO-DO: pull from known symbol & name contract updates
    // - setMetadata
    // - setNameAndTicker
    // - setName
    // https://github.com/pinax-network/substreams-evm-tokens/issues/13
}
