use substreams_ethereum::pb::eth::v2::{Call, StorageChange, TransactionTrace};

pub fn get_all_child_call_storage_changes<'a>(original: &'a Call, trx: &'a TransactionTrace) -> impl Iterator<Item = &'a StorageChange> + 'a {
    trx.calls
        .iter()
        .filter(move |call| call.parent_index == original.index)
        .flat_map(|call| call.storage_changes.iter())
}
