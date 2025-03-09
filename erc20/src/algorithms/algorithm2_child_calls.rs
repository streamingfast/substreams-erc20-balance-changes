use std::collections::HashMap;

use proto::pb::evm::tokens::types::v1::Algorithm;
use substreams::log;
use substreams::Hex;
use substreams_abis::evm::token::erc20::events::Transfer;
use substreams_ethereum::pb::eth::v2::{Call, StorageChange, TransactionTrace};

use super::utils::is_erc20_valid_balance;
use super::utils::{get_keccak_address, is_erc20_valid_address};
use super::utils::{Address, Hash};

pub fn get_all_child_call_storage_changes<'a>(original: &'a Call, trx: &'a TransactionTrace) -> impl Iterator<Item = &'a StorageChange> + 'a {
    trx.calls
        .iter()
        .filter(move |call| call.parent_index == original.index)
        .flat_map(|call| call.storage_changes.iter())
}

// algorithm #2 (case where storage changes are not in the same call as the transfer event)
pub fn find_erc20_balance_changes_algorithm2<'a>(
    trx: &'a TransactionTrace,
    original: &'a Call,
    transfer: &'a Transfer,
    keccak_address_map: &'a HashMap<Hash, Address>,
) -> impl Iterator<Item = (Address, &'a StorageChange, Algorithm)> + 'a {
    get_all_child_call_storage_changes(original, trx).filter_map(move |storage_change| {
        // Attempt to resolve storage_change -> owner address
        let owner = get_keccak_address(keccak_address_map, storage_change)?;

        // Skip if the owner doesn't match transfer.from or transfer.to
        if !is_erc20_valid_address(&owner, transfer) {
            log::info!(
                "owner={} does not match transfer from={} to={}",
                Hex(owner),
                Hex(&transfer.from),
                Hex(&transfer.to)
            );
            return None;
        }

        // Yield one of two results depending on whether the storage change
        // matches the transfer's balance changes
        let algorithm = if is_erc20_valid_balance(transfer, storage_change) {
            Algorithm::Erc20ChildCalls
        } else {
            Algorithm::Erc20BalanceDoesNotMatchTransfer
        };

        // Yield the tuple
        Some((owner, storage_change, algorithm))
    })
}

// TO-DO: refactor to re-use algorith1_call.rs