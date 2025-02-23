use substreams::log::info;
use substreams_ethereum::pb::eth::v2::{Call, TransactionTrace};

use crate::algorithms::algorithm2_child_calls::get_all_child_calls;

// Ignore fishing token transfers, typically do not have any storage changes associated with them or child calls
// https://github.com/streamingfast/substreams-erc20-balance-changes/issues/18
pub fn ignore_fishing_transfers(trx: &TransactionTrace, call: &Call) -> bool {
    let mut count = call.storage_changes.len();
    let child_calls = get_all_child_calls(&call, &trx);
    for child_call in child_calls {
        count += child_call.storage_changes.len();
    }
    if count == 0 {
        return true;
    }
    info!("ignoring fishing token transfer trx: {:?}", trx.hash);
    return false;
}
