use substreams::pb::substreams::Clock;
use substreams_ethereum::pb::eth::v2::StorageChange;

// Timestamp to date conversion
// ex: 2015-07-30T16:02:18Z => 2015-07-30
pub fn clock_to_date(clock: &Clock) -> String {
    match clock.timestamp.expect("timestamp missing").to_string().split('T').next() {
        Some(date) => date.to_string(),
        None => "".to_string(),
    }
}

// In ClickHouse, an aggregate function like argMax can only take one expression as the “ordering” argument.
// So we typically combine (block_num, index) into a single monotonic integer.
// For example, if each of block_num and index fits in 32 bits, we can do:
// max(toUInt64(block_num) * 2^32 + index) AS version
pub fn index_to_version(clock: &Clock, storage_change: &StorageChange) -> u64 {
    (clock.number << 32) + storage_change.ordinal
}
