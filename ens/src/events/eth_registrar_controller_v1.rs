use common::bigint_to_uint64;
use proto::pb::evm::tokens::ens::v1 as ens;
use substreams_abis::evm::ens::v1::ethregistrarcontroller::events;
use substreams_ethereum::{
    pb::eth::v2::{Call, Log, TransactionTrace},
    Event,
};

use crate::utils::label_to_node;

pub fn insert_eth_registrar_controller_v1<'a>(events: &mut ens::Events, transaction: &'a TransactionTrace, call: &'a Call, log: &'a Log) {
    // NameRegistered event
    if let Some(event) = events::NameRegistered::match_and_decode(log) {
        let node = label_to_node(&event.label);
        let expires = if bigint_to_uint64(&event.expires).is_some() {
            event.expires.to_u64()
        } else {
            return;
        };
        let base_cost = if bigint_to_uint64(&event.base_cost).is_some() {
            event.base_cost.to_u64()
        } else {
            return;
        };
        let premium = if bigint_to_uint64(&event.premium).is_some() {
            event.premium.to_u64()
        } else {
            return;
        };
        events.name_registered.push(ens::NameRegistered {
            // -- transaction --
            contract: log.address.to_vec(),
            transaction_hash: transaction.hash.to_vec(),

            // -- call --
            caller: call.caller.to_vec(),

            // -- log --
            ordinal: log.ordinal,

            // -- event --
            owner: event.owner.to_vec(),
            expires,

            // -- event (v1 & v0) --
            label: Some(event.label.to_vec()),
            name: Some(event.name),
            node: Some(node.to_vec()),
            base_cost: Some(base_cost),

            // -- event (v1) --
            premium: Some(premium),

            // -- event (base) --
            token_id: None,
        });
    }

    // NameRenewed event
    if let Some(event) = events::NameRenewed::match_and_decode(log) {
        let node = label_to_node(&event.label);
        let expires = if bigint_to_uint64(&event.expires).is_some() {
            event.expires.to_u64()
        } else {
            return;
        };
        let cost = if bigint_to_uint64(&event.cost).is_some() {
            event.cost.to_u64()
        } else {
            return;
        };

        events.name_renewed.push(ens::NameRenewed {
            contract: log.address.to_vec(),
            transaction_hash: transaction.hash.to_vec(),
            caller: call.caller.to_vec(),
            ordinal: log.ordinal,
            name: event.name,
            label: event.label.to_vec(),
            cost,
            expires,
            node: node.to_vec(),
        });
    }
}
