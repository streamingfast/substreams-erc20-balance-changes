use std::collections::HashMap;

use common::{to_global_sequence, Address};
use proto::pb::evm::tokens::balances::types::v1::{Algorithm, BalanceChange, Events};
use substreams::errors::Error;
use substreams::log;

use substreams::pb::substreams::Clock;

use crate::calls::get_balance_of;

#[substreams::handlers::map]
pub fn map_events(clock: Clock, erc20: Events) -> Result<Events, Error> {
    let mut events = Events::default();

    // detect missing balances to perform `balanceOf` RPC calls
    let mut existing_balances: HashMap<Address, Address> = HashMap::new();
    let mut missing_balances: HashMap<Address, Address> = HashMap::new();
    let mut index = 0; // start with highest relative index of ERC-20 map events
    let mut ordinal = 0; // start with highest ordinal of ERC-20 map events

    // find all the existing known balances
    for balance_changes in &erc20.balance_changes {
        index = index.max(balance_changes.index); // highest index
        ordinal = ordinal.max(balance_changes.ordinal); // highest ordinal
        existing_balances.insert(balance_changes.contract.to_vec(), balance_changes.owner.to_vec());
    }

    // find all missing balances based on transfer events
    for transfer in &erc20.transfers {
        index = index.max(transfer.index); // highest index
        ordinal = ordinal.max(transfer.ordinal); // highest ordinal
        let contract = transfer.contract.to_vec();
        let from = transfer.from.to_vec();
        let to = transfer.to.to_vec();

        if existing_balances.contains_key(&contract) {
            if existing_balances.contains_key(&from) {
                missing_balances.insert(contract.to_vec(), from);
            }
            if existing_balances.contains_key(&to) {
                missing_balances.insert(contract.to_vec(), to);
            }
        } else {
            missing_balances.insert(contract.to_vec(), from);
            missing_balances.insert(contract.to_vec(), to);
        }
    }

    // process missing balance changes
    // ignore NULL address
    let mut missing = 0;
    let mut fail_rpc = 0;
    for (contract, owner) in missing_balances {
        if owner == common::NULL_ADDRESS.to_vec() {
            continue;
        }
        missing += 1;
        index += 1;
        ordinal += 1; // RPC calls are not ordered, but starting from max ordinal, allows to keep track of the order
        match get_balance_of(contract.to_vec(), owner.to_vec()) {
            Some(balance) => {
                let balance_change = BalanceChange {
                    // -- transaction --
                    transaction_id: vec![],

                    // -- balance change --
                    contract,
                    owner,
                    old_balance: "".to_string(), // cannot determine old balance from RPC call
                    new_balance: balance.to_string(),

                    // -- ordering --
                    ordinal,
                    index,
                    global_sequence: to_global_sequence(&clock, &index),

                    // -- debug --
                    algorithm: Algorithm::Rpc.into(),
                };
                events.balance_changes.push(balance_change);
            }
            None => {
                log::info!(format!("missing balance: contract={} owner={}", common::bytes_to_hex(&contract), common::bytes_to_hex(&owner)));
                let balance_change = BalanceChange {
                    // -- transaction --
                    transaction_id: vec![],

                    // -- balance change --
                    contract,
                    owner,
                    old_balance: "".to_string(), // cannot determine old balance from RPC call
                    new_balance: "".to_string(),

                    // -- ordering --
                    ordinal,
                    index,
                    global_sequence: to_global_sequence(&clock, &index),

                    // -- debug --
                    algorithm: Algorithm::Rpc.into(),
                };
                fail_rpc += 1;
                events.balance_changes.push(balance_change);
            }
        }
    }

    // log::info!(format!("balance_changes: {}", events.balance_changes.len()));
    log::info!(format!("erc20.transfers: {}", erc20.transfers.len()));
    log::info!(format!("erc20.balance_changes: {}", erc20.balance_changes.len()));
    log::info!(format!("missing balances: {}", missing));
    log::info!(format!("failed RPC: {}", fail_rpc));
    log::info!(format!("balance_changes: {}", events.balance_changes.len()));
    Ok(events)
}
