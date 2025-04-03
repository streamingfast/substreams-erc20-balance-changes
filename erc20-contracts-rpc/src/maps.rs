use proto::pb::evm::tokens::erc20::contracts::v1::{ContractChange, Events};
use substreams::store::{DeltaBigInt, Deltas};
use substreams::{errors::Error, scalar::BigInt, Hex};

use crate::calls;

#[substreams::handlers::map]
pub fn map_events(store_erc20_transfers: Deltas<DeltaBigInt>) -> Result<Events, Error> {
    let contract_changes = store_erc20_transfers
        .deltas
        .into_iter()
        .filter_map(|delta| {
            // Extract Token Metadata only on first valid ERC20 transfer event
            if delta.new_value != BigInt::one() {
                return None;
            }

            // try to decode address and get contract details
            Hex::decode(&delta.key).ok().and_then(|address| {
                let decimals = calls::get_contract_decimals(address.to_vec());
                // Token must have decimals
                // do not attempt to perform any further RPC calls to get name,symbol
                if decimals.is_none() {
                    return None;
                }
                // Name & Symbol are optional, even after ERC20 transfer these fields can be modified
                let name = calls::get_contract_name(address.to_vec()).or(calls::get_contract_name_bytes32(address.to_vec()));
                let symbol = calls::get_contract_symbol(address.to_vec()).or(calls::get_contract_symbol_bytes32(address.to_vec()));

                Some(ContractChange {
                    transaction_id: None,

                    // -- call --
                    caller: None,

                    // -- ordering --
                    ordinal: None,

                    // -- contract --
                    address,
                    name,
                    symbol,
                    decimals,
                })
            })
        })
        .collect();

    Ok(Events {
        contract_changes,
    })
}
