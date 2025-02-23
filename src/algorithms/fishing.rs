use substreams::log::info;
use substreams_ethereum::pb::eth::v2::{Call, TransactionTrace};

use crate::algorithms::algorithm2_child_calls::get_all_child_call_storage_changes;

// Ignore fishing token transfers, typically do not have any storage changes associated with them or child calls
// https://github.com/streamingfast/substreams-erc20-balance-changes/issues/18
pub fn is_fishing_transfers<'a>(trx: &'a TransactionTrace, call: &'a Call) -> bool {
    let mut count = call.storage_changes.len();
    let storage_changes = get_all_child_call_storage_changes(&call, &trx);
    count += storage_changes.count();
    if count == 0 {
        info!("ignoring fishing token transfer trx: {:?}", trx.hash);
        return false;
    }
    return true;
}
