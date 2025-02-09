
pub fn map_balance_changes(clock: Clock, block: Block) -> Vec<BalanceChange> {
    let mut balance_changes = Vec::new();
    let mut index: u32 = 0;

    for (trx, call, transfer) in iter_transfers(block) {
        // Trying with algorithm #1
        let mut found_balance_changes = find_erc20_balance_changes_algorithm1(&clock, &trx, &call, &transfer);
        if !found_balance_changes.is_empty() {
            balance_changes.extend(found_balance_changes);
            continue;
        }

        // No balance changes found using algorithm #1, trying with algorithm #2
        found_balance_changes = find_erc20_balance_changes_algorithm2(&clock, &transfer, &call, &trx);
        if !found_balance_changes.is_empty() {
            balance_changes.extend(found_balance_changes);
            continue;
        }

        // No algorithm could extract the balance change, old/new balance is fixed at 0
        balance_changes.push(BalanceChange {
            // -- block --
            block_num: clock.number,
            block_hash: clock.id.clone(),
            date: clock_to_date(&clock),
            timestamp: clock.timestamp,

            // -- transaction
            transaction_id: Hex::encode(&trx.hash),
            call_index: call.index,

            // -- storage --
            index,
            version: index_to_version(&clock, index),
            storage_key: "".to_string(),

            // -- balance change --
            contract: Hex::encode(&call.address),
            owner: Hex::encode(&transfer.to),
            old_balance: "0".to_string(),
            new_balance: "0".to_string(),
            amount: transfer.value.to_string(),
            change_type: BalanceChangeType::Unspecified as i32,
        });
        // Increment the index for the next balance change
        index += 1;
    }
    balance_changes
}

/// normal case
fn find_erc20_balance_changes_algorithm1(
    clock: &Clock,
    trx: &TransactionTrace,
    call: &Call,
    transfer: &TransferAbi,
) -> Vec<BalanceChange> {
    let mut out = Vec::new();
    let mut keccak_address_map: Option<StorageKeyToAddressMap> = None;

    for storage_change in &call.storage_changes {
        let old_balance = BigInt::from_signed_bytes_be(&storage_change.old_value);
        let new_balance = BigInt::from_signed_bytes_be(&storage_change.new_value);

        let balance_change = new_balance - old_balance;
        let balance_change_abs = if balance_change < BigInt::zero() {
            balance_change.neg()
        } else {
            balance_change
        };

        let value = transfer.value.clone();
        let transfer_value_abs = if value.clone() < BigInt::zero() {
            value.neg()
        } else {
            value.clone()
        };

        // if balance_change_abs != transfer_value_abs {
        //     info!("Balance change does not match transfer value. Balance change: {}, transfer value: {}", balance_change_abs, transfer_value_abs);
        //     continue;
        // }

        // // We memoize the keccak address map by call because it is expensive to compute
        // if keccak_address_map.is_none() {
        //     keccak_address_map = Some(erc20_addresses_for_storage_keys(call));
        // }

        let keccak_address = match keccak_address_map
            .as_ref()
            .unwrap()
            .get(&storage_change.key)
        {
            Some(address) => address,
            None => {
                if storage_change.key[0..16] == ZERO_STORAGE_PREFIX {
                    info!("Skipping balance change for zero key");
                    continue;
                }

                info!(
                    "No keccak address found for key: {}, trx {}",
                    Hex(&storage_change.key),
                    Hex(&trx.hash)
                );
                continue;
            }
        };

        if !erc20_is_valid_address(keccak_address, transfer) {
            info!("Keccak address does not match transfer address. Keccak address: {}, sender address: {}, receiver address: {}, trx {}", Hex(keccak_address), Hex(&transfer.from), Hex(&transfer.to), Hex(&trx.hash));
            continue;
        }
        out.push(storage_change_to_balance_change(
            clock,
            call,
            trx,
            keccak_address,
            &value,
            storage_change,
            BalanceChangeType::BalanceChangeType1 as i32,
        ));
    }
    out
}

// case where storage changes are not in the same call as the transfer event
fn find_erc20_balance_changes_algorithm2(
    clock: &Clock,
    transfer: &TransferAbi,
    original_call: &Call,
    trx: &TransactionTrace,
) -> Vec<BalanceChange> {
    let mut out = Vec::new();

    //get all keccak keys for transfer.to and transfer.from

    let mut keys = HashMap::new();
    for call in trx.calls.iter() {
        let keccak_address_map = erc20_addresses_for_storage_keys(call);
        keys.extend(keccak_address_map);
    }

    let child_calls = get_all_child_calls(original_call, trx);

    //get all storage changes for these calls:
    let mut storage_changes = Vec::new();
    for call in child_calls.iter() {
        storage_changes.extend(call.storage_changes.clone());
    }

    let mut total_sent = BigInt::zero();
    let mut total_received = BigInt::zero();

    //check if any of the storage changes match the transfer.to or transfer.from
    for storage_change in storage_changes.clone().iter() {
        let keccak_address = match keys.get(&storage_change.key) {
            Some(address) => address,
            None => continue,
        };

        if !erc20_is_valid_address(keccak_address, transfer) {
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

        out.push(storage_change_to_balance_change(
            clock,
            original_call,
            trx,
            keccak_address,
            &transfer.value,
            storage_change,
            BalanceChangeType::BalanceChangeType2 as i32,
        ));
    }

    if total_sent == transfer.value {
        return out;
    }

    let mut diff = total_sent - total_received;
    if diff < BigInt::zero() {
        diff = diff.neg();
    }

    //look for a storage change that matches the diff
    for storage_change in storage_changes.iter() {
        let keccak_address = match keys.get(&storage_change.key) {
            Some(address) => address,
            None => continue,
        };

        let old_balance = BigInt::from_unsigned_bytes_be(&storage_change.old_value);
        let new_balance = BigInt::from_unsigned_bytes_be(&storage_change.new_value);

        let mut balance_change = new_balance - old_balance;
        if balance_change < BigInt::zero() {
            balance_change = balance_change.neg();
        }

        if balance_change != diff {
            continue;
        }

        out.push(storage_change_to_balance_change(
            clock,
            original_call,
            trx,
            keccak_address,
            &transfer.value,
            storage_change,
            BalanceChangeType::BalanceChangeType2 as i32,
        ));
    }

    out
}

fn storage_change_to_balance_change(clock: &Clock, call: &Call, trx: &TransactionTrace, owner: &Vec<u8>, value: &BigInt, storage_change: &StorageChange, change_type: i32, index: u32 ) -> BalanceChange {
    // Using `storage_change.address` is the correct way to get the contract address here
    // as it handles delegate calls correctly, for contract Proxy support.
    //
    // Indeed, the storage change holds the address of the contract that is actually holding
    // the real state of the storage slot, the proxy contract when the call is a delegate call.
    let contract = Hex::encode(&storage_change.address);

    BalanceChange {
        // -- block --
        block_num: clock.number,
        block_hash: clock.id.clone(),
        date: clock_to_date(clock),
        timestamp: clock.timestamp,

        // -- transaction
        transaction_id: Hex::encode(&trx.hash),
        call_index: call.index,

        // -- storage --
        storage_key: Hex::encode(&storage_change.key),

        // -- balance change --
        contract,
        owner: Hex::encode(owner),
        old_balance: BigInt::from_unsigned_bytes_be(&storage_change.old_value).to_string(),
        new_balance: BigInt::from_unsigned_bytes_be(&storage_change.new_value).to_string(),
        amount: value.to_string(),
        change_type,
    }
}