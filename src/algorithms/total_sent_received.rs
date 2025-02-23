use crate::abi::erc20::events::Transfer;
use substreams::log;
use substreams::scalar::BigInt;
use substreams::Hex;
use substreams_ethereum::pb::eth::v2::Call;

use super::utils::{get_keccak_address, is_erc20_valid_address, StorageKeyToAddressMap};

pub fn compute_total_sent_received<'a>(
    child_calls: Vec<&'a Call>,
    transfer: &'a Transfer,
    keccak_address_map: &'a StorageKeyToAddressMap,
) -> (BigInt, BigInt) {
    let mut total_sent = BigInt::zero();
    let mut total_received = BigInt::zero();

    //check if any of the storage changes match the transfer.to or transfer.from
    for call in child_calls.iter() {
        for storage_change in call.storage_changes.iter() {
            let owner = match get_keccak_address(keccak_address_map, &storage_change) {
                Some(address) => address,
                None => continue,
            };

            if !is_erc20_valid_address(&owner, transfer) {
                log::info!(
                    "owner={} does not match transfer from={} to={}",
                    Hex(owner),
                    Hex(&transfer.from),
                    Hex(&transfer.to)
                );
                continue;
            }

            let old_balance = BigInt::from_unsigned_bytes_be(&storage_change.old_value);
            let new_balance = BigInt::from_unsigned_bytes_be(&storage_change.new_value);

            let balance_change = new_balance - old_balance;
            if balance_change < BigInt::zero() {
                total_sent = total_sent + balance_change.neg();
            } else {
                total_received = total_received + balance_change;
            };
        }
    }
    (total_sent, total_received)
}
