use common::to_global_sequence;
use proto::pb::evm::tokens::erc20::contracts::v1::{ContractChange, Events};
use substreams::pb::substreams::Clock;
use substreams::store::{DeltaBigInt, Deltas};
use substreams::{errors::Error, scalar::BigInt, Hex};

use crate::calls;

#[substreams::handlers::map]
pub fn map_events(clock: Clock, store_erc20_transfers: Deltas<DeltaBigInt>) -> Result<Events, Error> {
    let contract_changes = store_erc20_transfers
        .deltas
        .into_iter()
        .enumerate()
        .filter_map(|(idx, delta)| {
            // Extract Token Metadata on first valid ERC20 transfer event
            if delta.new_value != BigInt::one() {
                return None;
            }

            // try to decode address and get contract details
            Hex::decode(&delta.key).ok().and_then(|address| {
                calls::get_contract(&address).map(|(name, symbol, decimals)| {
                    ContractChange {
                        transaction_id: None,

                        // -- ordering --
                        ordinal: None,
                        index: idx as u64,
                        global_sequence: to_global_sequence(&clock, idx as u64),

                        // -- contract --
                        address,
                        name: Some(name),
                        symbol: Some(symbol),
                        decimals: Some(decimals.into()),
                    }
                })
            })
        })
        .collect();

    Ok(Events {
        contract_changes,
    })
}
