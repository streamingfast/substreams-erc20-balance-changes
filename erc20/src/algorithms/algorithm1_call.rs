use std::collections::HashMap;

use proto::pb::evm::tokens::types::v1::BalanceChangeType;
use substreams::log;
use substreams::Hex;
use substreams_abis::evm::token::erc20::events::Transfer;
use substreams_ethereum::pb::eth::v2::{Call, StorageChange};

use super::utils::{get_keccak_address, is_erc20_valid_address, is_erc20_valid_balance, Address, Hash};

// algorithm #1 (normal case)
pub fn find_erc20_balance_changes_algorithm1<'a>(
    call: &'a Call,
    transfer: &'a Transfer,
    keccak_address_map: &'a HashMap<Hash, Address>,
) -> impl Iterator<Item = (Address, &'a StorageChange, BalanceChangeType)> + 'a {
    call.storage_changes.iter().filter_map(move |storage_change| {
        // Extract the owner address
        let owner = get_keccak_address(keccak_address_map, storage_change)?;

        // Ensure owner is either the sender or receiver
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
        let balance_type = if is_erc20_valid_balance(transfer, storage_change) {
            BalanceChangeType::BalanceChangeType1
        } else {
            BalanceChangeType::Unspecified
        };

        Some((owner, storage_change, balance_type))
    })
}
