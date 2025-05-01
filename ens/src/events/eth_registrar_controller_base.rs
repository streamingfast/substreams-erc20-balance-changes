use common::bigint_to_uint64;
use proto::pb::evm::tokens::ens::v1 as ens;
use substreams_abis::evm::ens::base::ethregistrarcontroller::events;
use substreams_ethereum::{
    pb::eth::v2::{Call, Log, TransactionTrace},
    Event,
};

pub fn insert_base_eth_registrar_controller<'a>(events: &mut ens::Events, transaction: &'a TransactionTrace, call: &'a Call, log: &'a Log) {
    // NameRegistered event
    if let Some(event) = events::NameRegistered::match_and_decode(log) {
        let expires = if bigint_to_uint64(&event.expires).is_some() {
            event.expires.to_u64()
        } else {
            return;
        };
        events.name_registered.push(ens::NameRegistered {
            // -- transaction --
            transaction_hash: transaction.hash.to_vec(),

            // -- call --
            contract: log.address.to_vec(),

            // -- log --
            caller: call.caller.to_vec(),

            // -- event --
            ordinal: log.ordinal,
            owner: event.owner.to_vec(),
            expires,

            // -- event (optional) --
            token_id: Some(event.id.to_string()),
            name: None,
            label: None,
            node: None,
            base_cost: None,
        });
    }

    // NameRenewed same ABI as V1
}
