use substreams::pb::substreams::Clock;
use substreams_database_change::tables::Row;

pub fn common_key(clock: &Clock, ordinal: u64) -> [(&'static str, String); 3] {
    let seconds = clock.timestamp.expect("clock.timestamp is required").seconds;
    [
        ("timestamp", seconds.to_string()),
        ("block_num", clock.number.to_string()),
        ("ordinal", ordinal.to_string()),
    ]
}

// Helper function to set clock data in a row
pub fn set_clock(clock: &Clock, row: &mut Row) {
    row.set("block_num", clock.number.to_string())
        .set("block_hash", format!("0x{}", &clock.id))
        .set("timestamp", clock.timestamp.expect("missing timestamp").seconds.to_string());
}
