use common::to_global_sequence;
use proto::pb::evm::tokens::contracts::types::v1::{Algorithm, ContractChange, Events};
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
            // every 10,000 blocks (~24h ETH, ~6h BSC, ~4h Base) token has a transfer
            if delta.new_value != BigInt::one() && delta.new_value.to_u64() % 10000 != 0 {
                return None;
            }

            // try to decode address and get contract details
            Hex::decode(&delta.key).ok().and_then(|address| {
                calls::get_contract(&address).map(|(name, symbol, decimals)| {
                    ContractChange {
                        // -- ordering --
                        index: idx as u64,
                        global_sequence: to_global_sequence(&clock, &(idx as u64)),

                        // -- contract --
                        address,
                        name,
                        symbol,
                        decimals: decimals.into(),

                        // -- debug --
                        algorithm: Algorithm::Rpc.into(),

                        // -- the remaining values --
                        ..Default::default()
                    }
                })
            })
        })
        .collect();

    Ok(Events {
        contract_changes,
        ..Default::default()
    })
}
