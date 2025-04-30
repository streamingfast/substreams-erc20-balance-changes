use common::bigint_to_uint64;
use proto::pb::evm::tokens::ens::v1 as ens;
use substreams_abis::evm::ens::v1::ensregistry::events;
use substreams_ethereum::{
    pb::eth::v2::{Call, Log, TransactionTrace},
    Event,
};

pub fn insert_ens_registry<'a>(events: &mut ens::Events, transaction: &'a TransactionTrace, call: &'a Call, log: &'a Log) {
    // NewOwner event
    if let Some(event) = events::NewOwner::match_and_decode(log) {
        events.new_owner.push(ens::NewOwner {
            contract: log.address.to_vec(),
            transaction_hash: transaction.hash.to_vec(),
            caller: call.caller.to_vec(),
            ordinal: log.ordinal,
            node: event.node.to_vec(),
            label: event.label.to_vec(),
            owner: event.owner.to_vec(),
        });
    }

    // Transfer event
    if let Some(event) = events::Transfer::match_and_decode(log) {
        events.transfer.push(ens::Transfer {
            contract: log.address.to_vec(),
            transaction_hash: transaction.hash.to_vec(),
            caller: call.caller.to_vec(),
            ordinal: log.ordinal,
            node: event.node.to_vec(),
            owner: event.owner.to_vec(),
        });
    }

    // NewResolver event
    if let Some(event) = events::NewResolver::match_and_decode(log) {
        events.new_resolver.push(ens::NewResolver {
            contract: log.address.to_vec(),
            transaction_hash: transaction.hash.to_vec(),
            caller: call.caller.to_vec(),
            ordinal: log.ordinal,
            node: event.node.to_vec(),
            resolver: event.resolver.to_vec(),
        });
    }

    // NewTTL event
    if let Some(event) = events::NewTtl::match_and_decode(log) {
        let ttl = if bigint_to_uint64(&event.ttl).is_some() {
            event.ttl.to_u64()
        } else {
            return;
        };
        events.new_ttl.push(ens::NewTtl {
            contract: log.address.to_vec(),
            transaction_hash: transaction.hash.to_vec(),
            caller: call.caller.to_vec(),
            ordinal: log.ordinal,
            node: event.node.to_vec(),
            ttl,
        });
    }
}
