use proto::pb::evm::tokens::contracts::v1::{ContractCreation, Events};
use substreams::errors::Error;
use substreams_ethereum::pb::eth::v2::{Block, CallType};

#[substreams::handlers::map]
pub fn map_events(block: Block) -> Result<Events, Error> {
    let mut events = Events::default();

    for trx in block.transactions() {
        for call_view in trx.calls() {
            // Filter for contract creation calls
            let call = call_view.call;
            if call.call_type() != CallType::Create { continue; }

            // Code changes describe the contract creation
            for code_change in call.code_changes.iter() {
                // -- transaction --
                events.contract_creations.push(ContractCreation {
                    // -- transaction --
                    transaction_id: trx.hash.to_vec(),
                    from: trx.from.to_vec(),
                    to: trx.to.to_vec(),

                    // -- call --
                    caller: call.caller.to_vec(),

                    // -- ordering --
                    ordinal: code_change.ordinal,

                    // -- contract --
                    address: code_change.address.to_vec(),
                    hash: code_change.new_hash.to_vec(),
                });
            }
        }
    }

    Ok(events)
}
