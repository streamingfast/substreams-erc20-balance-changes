
use substreams::{errors::Error, scalar::BigInt, Hex};
use proto::pb::evm::tokens::types::v1::{Contract, ContractCreation, Events};
use substreams::store::{Deltas, DeltaBigInt};
use substreams_ethereum::pb::eth::v2::{Block, CallType};

use crate::calls;

#[substreams::handlers::map]
pub fn map_events(block: Block, store_erc20_transfers: Deltas<DeltaBigInt> ) -> Result<Events, Error> {
    let mut events = Events::default();
    // -- contract creations --
    for trx in block.transactions() {
        for call in &trx.calls {
            if call.call_type() != CallType::Create { continue }
            for code_change in &call.code_changes {
                events.contract_creations.push(ContractCreation{
                    // -- transaction --
                    transaction_id: trx.hash.clone(),
                    from: trx.from.clone(),
                    to: trx.to.clone(),

                    // -- contract --
                    address: code_change.address.clone(),
                });
            }
        }
    }

    // -- contract events --
    for deltas in store_erc20_transfers.deltas {
        // match using 1st block which includes ERC20 token transfer event per address
        if deltas.new_value != BigInt::one() { continue }
        let address = Hex::decode(&deltas.key).expect("invalid address");
        let contract = calls::get_contract(address.clone());
        if contract.is_none() { continue } // not valid ERC20 token contract
        let (name, symbol, decimals) = contract.unwrap();

        events.contracts.push(Contract {
            // -- contract --
            address,
            name,
            symbol,
            decimals: decimals.into(),
        });
    }
    Ok(events)
}
