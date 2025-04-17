use substreams_database_change::pb::database::DatabaseChanges;

use crate::pb::events::{Events, Mints};

#[substreams::handlers::map]
pub fn db_out(events: Events, mints: Mints) -> Result<DatabaseChanges, substreams::errors::Error> {
    let mut tables = substreams_database_change::tables::Tables::new();

    Ok(tables.to_database_changes())
}
