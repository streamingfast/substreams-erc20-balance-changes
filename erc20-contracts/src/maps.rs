use common::to_global_sequence;
use proto::pb::evm::tokens::algorithm::v1::Algorithm;
use proto::pb::evm::tokens::erc20::contracts::v1::{Events, ContractChange};
use substreams::errors::Error;
use substreams::pb::substreams::Clock;
use substreams_abis::evm::token::erc20_name_symbol;
use substreams_ethereum::pb::eth::v2::Block;
use substreams_ethereum::Function;

#[substreams::handlers::map]
pub fn map_events(clock: Clock, block: Block) -> Result<Events, Error> {
    let mut events = Events::default();
    let mut index = 0;

    for trx in block.transactions() {
        for call_view in trx.calls() {
            let call = call_view.call;

            // setName
            match erc20_name_symbol::functions::SetName::match_and_decode(call) {
                Some(func) => {
                    let name = func.name;
                    events.contract_changes.push(ContractChange {
                        // -- transaction --
                        transaction_id: trx.hash.to_vec(),
                        from: trx.from.to_vec(),
                        to: trx.to.to_vec(),
                        caller: call.caller.to_vec(),

                        // -- ordering --
                        ordinal: call.begin_ordinal,
                        index,
                        global_sequence: to_global_sequence(&clock, call.index.into()),

                        // -- contract --
                        address: call.address.to_vec(),
                        name,
                        symbol: "".to_string(),
                        decimals: 0,
                        // -- debug --
                        algorithm: Algorithm::Call.into(),
                    });
                    index += 1;
                }
                None => {}
            }

            // setSymbol
            match erc20_name_symbol::functions::SetSymbol::match_and_decode(call) {
                Some(func) => {
                    let symbol = func.symbol;
                    events.contract_changes.push(ContractChange {
                        // -- transaction --
                        transaction_id: trx.hash.to_vec(),
                        from: trx.from.to_vec(),
                        to: trx.to.to_vec(),
                        caller: call.caller.to_vec(),

                        // -- ordering --
                        ordinal: call.begin_ordinal,
                        index,
                        global_sequence: to_global_sequence(&clock, call.index.into()),

                        // -- contract --
                        address: call.address.to_vec(),
                        name: "".to_string(),
                        symbol,
                        decimals: 0,
                        // -- debug --
                        algorithm: Algorithm::Call.into(),
                    });
                    index += 1;
                }
                None => {}
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
