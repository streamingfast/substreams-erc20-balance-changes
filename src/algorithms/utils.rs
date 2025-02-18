use std::collections::HashMap;

use substreams::log;
use substreams_ethereum::pb::eth::v2::{Call, StorageChange};
use substreams::scalar::BigInt;
use substreams::Hex;
use crate::abi::erc20::events::Transfer;
use hex_literal::hex;

// const NULL_ADDRESS: [u8; 20] = hex!("0000000000000000000000000000000000000000");
const ZERO_STORAGE_PREFIX: [u8; 16] = hex!("00000000000000000000000000000000");

pub type StorageKeyToAddressMap = HashMap<Vec<u8>, Vec<u8>>;

pub fn addresses_for_storage_keys(call: &Call) -> StorageKeyToAddressMap {
    let mut out = HashMap::new();

    for (storage_key, preimage) in &call.keccak_preimages {
        if preimage.len() != 128 {
            log::info!("Skipping storage key {} with invalid length", storage_key);
            continue;
        }

        // Ignoring 0x000 null balance changes
        //
        // Burn Address:
        // When tokens are “burned,” they are effectively sent to an address where no one has—or can ever have—the private key.
        // By convention, the zero address is often used to signal “tokens have been removed from circulation.”
        // ex: emit Transfer(holder, address(0), amount)
        //
        // Minting Source:
        // Conversely, some implementations also use the zero address as the “source” when tokens are minted.
        // ex: emit Transfer(address(0), recipient, amount);
        if &preimage[64..126] != "00000000000000000000000000000000000000000000000000000000000000" {
            log::info!("Skipping storage key {} with non-zero padding", storage_key);
            continue;
        }

        let address = &preimage[24..64];
        out.insert(
            hex::decode(storage_key).expect("Failed to decode hash hex string"),
            hex::decode(address).expect("Failed to decode address hex string"),
        );
    }
    out
}

pub fn get_owner_from_keccak_address_map(
    keccak_address_map: &StorageKeyToAddressMap,
    storage_change: &StorageChange
) -> Option<Vec<u8>> {
    // If found in the map, clone it; otherwise enter the or_else closure
    keccak_address_map
        .get(&storage_change.key)
        .cloned()
        .or_else(|| {
            // If key starts with zero prefix, log that we're skipping it
            if storage_change.key.starts_with(&ZERO_STORAGE_PREFIX[..]) {
                log::info!(
                    "skip zero key storage change key={}",
                    Hex(&storage_change.key)
                );
            } else {
                log::info!(
                    "storage change does not match any owner address key={}",
                    Hex(&storage_change.key)
                );
            }
            None
        })
}

pub fn is_erc20_valid_address(address: &Vec<u8>, transfer: &Transfer) -> bool {
    address == &transfer.from || address == &transfer.to
}

pub fn is_erc20_valid_balance(transfer: &Transfer, storage_change: &StorageChange) -> bool {
    let old_balance = BigInt::from_signed_bytes_be(&storage_change.old_value);
    let new_balance = BigInt::from_signed_bytes_be(&storage_change.new_value);

    // Absolute balance change
    let balance_change_abs = BigInt::absolute(&(new_balance - old_balance));

    // Absolute transfer value
    let transfer_value_abs = BigInt::absolute(&transfer.value);

    // Compare the difference in absolute terms.
    // We allow a difference of 0 or 1 wei, i.e., if |balance_change_abs - transfer_value_abs| > 1, we continue.
    // https://github.com/streamingfast/substreams-erc20-balance-changes/issues/14
    let diff = BigInt::absolute(&(&balance_change_abs - &transfer_value_abs));
    if diff > BigInt::one() {
        log::info!("Balance change does not match transfer value. Balance change: {}, transfer value: {}", balance_change_abs, transfer_value_abs);
        return false;
    }
    return true;
}
