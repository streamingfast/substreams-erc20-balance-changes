use substreams::log;
use substreams_ethereum::pb::eth::v2::{Call, StorageChange};
use substreams::Hex;
use crate::abi::erc20::events::Transfer;
use crate::pb::erc20::types::v1::BalanceChangeType;

use super::utils::{get_owner_from_keccak_address_map, is_erc20_valid_address, is_erc20_valid_balance, StorageKeyToAddressMap};

// algorithm #1 (normal case)
pub fn find_erc20_balance_changes_algorithm1(
    call: &Call,
    transfer: &Transfer,
    keccak_address_map: &StorageKeyToAddressMap,
) -> Vec<(Vec<u8>, StorageChange, BalanceChangeType)> {
    let mut out = Vec::new();

    for storage_change in &call.storage_changes {
        // Check if the transfer matches the storage change balance changes
        if !is_erc20_valid_balance(transfer, storage_change) {
            continue;
        }

        // extract the owner address
        let owner = match get_owner_from_keccak_address_map(keccak_address_map, &storage_change) {
            Some(address) => address,
            None => continue
        };

        // make sure owner is either the sender or receiver
        if !is_erc20_valid_address(&owner, transfer) {
            log::info!("owner={} does not match transfer from={} to={}", Hex(owner), Hex(&transfer.from), Hex(&transfer.to));
            continue;
        }
        out.push((owner, storage_change.clone(), BalanceChangeType::BalanceChangeType1));
    }
    out
}