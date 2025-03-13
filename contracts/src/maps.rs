
use substreams::errors::Error;
use proto::pb::evm::tokens::types::v1::{Algorithm, Contract, Events};
use substreams_ethereum::pb::eth::v2::{Block, CallType};

use crate::calls;

#[substreams::handlers::map]
pub fn map_events(block: Block) -> Result<Events, Error> {
    let mut events = Events::default();

    for trx in block.transactions() {
        for call in &trx.calls {
            if call.call_type() != CallType::Create { continue }
            for code in &call.code_changes {
                let address = code.address.clone();
                let contract = calls::get_contract(address.clone());
                if contract.is_none() { continue }
                let (name, symbol, decimals) = contract.unwrap();

                events.contracts.push(Contract {
                    // -- transaction
                    transaction_id: trx.hash.clone(),

                    // -- contract --
                    address: code.address.clone(),
                    name,
                    symbol,
                    decimals: decimals.into(),

                    // -- debug --
                    algorithm: Algorithm::ContractCreation.into(),
                });
            }
        }
    }
    Ok(events)
}
