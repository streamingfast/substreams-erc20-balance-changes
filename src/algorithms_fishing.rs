use substreams::log::info;
use substreams_ethereum::pb::eth::v2::{Call, TransactionTrace};

use crate::algorithms::get_all_child_calls;

// Ignore fishing token transfers, typically do not have any storage changes associated with them or child calls
// https://github.com/streamingfast/substreams-erc20-balance-changes/issues/18
pub fn ignore_fishing_transfers(trx: &TransactionTrace, call: &Call) -> bool {
    let child_calls = get_all_child_calls(&call, &trx);
    if call.storage_changes.is_empty() && child_calls.is_empty() {
        info!("ignoring fishing token transfer trx: {:?}", trx.hash);
        return false;
    }
    return true;
}