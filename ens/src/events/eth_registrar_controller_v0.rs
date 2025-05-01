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
        let base_cost = if bigint_to_uint64(&event.cost).is_some() {
            event.cost.to_u64()
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
            name: event.name,
            label: event.label.to_vec(),
            node: node.to_vec(),
            base_cost: base_cost,
            premium: None,
        });
    }

    // NameRenewed same ABI as V1
}
