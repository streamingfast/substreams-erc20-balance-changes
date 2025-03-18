use std::collections::HashMap;

use common::{Address, Hash};
use substreams_abis::evm::token::erc20::events::Transfer;
use substreams_ethereum::pb::eth::v2::StorageChange;

use super::utils::{get_keccak_address, is_erc20_valid_address, is_erc20_valid_balance};

pub fn get_owner_from_erc20_balance_change<'a>(
    transfer: &'a Transfer,
    storage_change: &'a StorageChange,
    keccak_address_map: &'a HashMap<Hash, Address>,
) -> Option<Address> {
    // Extract the owner address
    let owner = get_keccak_address(keccak_address_map, storage_change)?;

    // Ensure owner is either the sender or receiver
    if !is_erc20_valid_address(owner, transfer) {
        return None;
    }

    // Yield one of two results depending on whether the storage change
    // matches the transfer's balance changes
    if !is_erc20_valid_balance(transfer, storage_change) {
        return None;
    }
    Some(owner.clone())
}
