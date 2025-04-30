use common::bigint_to_uint64;
use proto::pb::evm::tokens::ens::v1 as ens;
use substreams_abis::evm::ens::v0::ethregistrarcontroller::events;
use substreams_ethereum::{
    pb::eth::v2::{Call, Log, TransactionTrace},
    Event,
};

use crate::utils::label_to_node;

pub fn insert_v0_eth_registrar_controller<'a>(events: &mut ens::Events, transaction: &'a TransactionTrace, call: &'a Call, log: &'a Log) {
    // NameRegistered event
    if let Some(event) = events::NameRegistered::match_and_decode(log) {
        let node = label_to_node(&event.label);
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
            ordinal: log.ordinal,

            // -- event --
            owner: event.owner.to_vec(),
            expires,

            // -- event (optional) --
            name: Some(event.name),
            label: Some(event.label.to_vec()),
            node: Some(node.to_vec()),
            base_cost: None,
            token_id: None,
        });
    }

    // NameRenewed same ABI as V1
}
