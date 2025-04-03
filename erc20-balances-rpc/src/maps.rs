use std::collections::HashMap;

use common::Address;
use proto::pb::evm::tokens::balances::v1::{BalanceChange, Events, Algorithm};
use substreams::errors::Error;
use substreams::log;

use crate::calls::get_balance_of;

#[substreams::handlers::map]
pub fn map_events(erc20: Events) -> Result<Events, Error> {
    let mut events = Events::default();

    // detect missing balances to perform `balanceOf` RPC calls
    let mut existing_balances: HashMap<Address, Address> = HashMap::new();
    let mut missing_balances: HashMap<Address, Address> = HashMap::new();

    // find all the existing known balances
    for balance_changes in &erc20.balance_changes {
        existing_balances.insert(balance_changes.contract.to_vec(), balance_changes.address.to_vec());
    }

    // find all missing balances based on transfer events
    for transfer in &erc20.transfers {
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
    for (contract, address) in missing_balances {
        if address == common::NULL_ADDRESS.to_vec() {
            continue;
        }
        missing += 1;
        match get_balance_of(contract.to_vec(), address.to_vec()) {
            Some(balance) => {
                let balance_change = BalanceChange {
                    // -- transaction --
                    transaction_id: None,

                    // -- call --
                    caller: None,

                    // -- balance change --
                    contract,
                    address,
                    old_balance: None,
                    new_balance: balance.to_string(),

                    // -- ordering --
                    ordinal: None,

                    // -- debug --
                    algorithm: Algorithm::Rpc.into(),
                    reason: None,
                    trx_type: None,
                    call_type: None,
                };
                events.balance_changes.push(balance_change);
            }
            None => {
                log::info!(format!(
                    "|missing balance (contract/owner)|\n{}\n{}\n",
                    common::bytes_to_hex(&contract),
                    common::bytes_to_hex(&address)
                ));
                fail_rpc += 1;
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
