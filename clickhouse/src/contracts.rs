
// Helper function to process a single contract_changes
fn process_contract_creation(tables: &mut substreams_database_change::tables::Tables, clock: &Clock, event: ContractCreation) {
    let address = bytes_to_hex(&event.address);
    let key = [("address", address.to_string())];
    set_clock(
        &clock,
        tables
            .create_row("contract_creations", key)
            // -- transaction --
            .set("transaction_id", bytes_to_hex(&event.transaction_id))
            .set("from", bytes_to_hex(&event.from))
            .set("to", bytes_to_hex(&event.to))

            // -- ordering --
            .set("ordinal", event.ordinal)
            .set("index", event.index)
            .set("global_sequence", event.global_sequence)

            // -- contract --
            .set("address", &address),
    );
}