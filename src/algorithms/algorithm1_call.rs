use std::collections::HashMap;

use crate::abi::erc20::events::Transfer;
use crate::pb::erc20::types::v1::BalanceChangeType;
use substreams::log;
use substreams::Hex;
use substreams_ethereum::pb::eth::v2::{Call, StorageChange};

use super::utils::{
    get_keccak_address, is_erc20_valid_address, is_erc20_valid_balance, Hash, Address
};

// algorithm #1 (normal case)
pub fn find_erc20_balance_changes_algorithm1<'a>(
    call: &'a Call,
    transfer: &'a Transfer,
    keccak_address_map: &'a HashMap<Hash, Address>,
) -> Vec<(Address, &'a StorageChange, BalanceChangeType)> {
    let mut out = Vec::new();

    for storage_change in &call.storage_changes {
        // extract the owner address
        let owner = match get_keccak_address(keccak_address_map, &storage_change) {
            Some(address) => address,
            None => continue,
        };

        // make sure owner is either the sender or receiver
        if !is_erc20_valid_address(&owner, transfer) {
            log::info!(
                "owner={} does not match transfer from={} to={}",
                Hex(owner),
                Hex(&transfer.from),
                Hex(&transfer.to)
            );
            continue;
        }

        // Check if the transfer matches the storage change balance changes
        if is_erc20_valid_balance(transfer, storage_change) {
            out.push((
                owner,
                storage_change,
                BalanceChangeType::BalanceChangeType1,
            ));

        // Storage Change does not match the transfer value, but does match owner address
        } else {
            out.push((
                owner,
                storage_change,
                BalanceChangeType::Unspecified,
            ));
        }
    }
    out
}
