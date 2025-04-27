use proto::pb::evm::tokens::ens::v1 as ens;
use substreams_abis::evm::ens::v1::publicresolver::events;
use substreams_ethereum::{
    pb::eth::v2::{Call, Log, TransactionTrace},
    Event,
};

pub fn insert_public_resolver<'a>(events: &mut ens::Events, transaction: &'a TransactionTrace, call: &'a Call, log: &'a Log) {
    // AddrChanged event
    if let Some(event) = events::AddrChanged::match_and_decode(log) {
        events.address_changed.push(ens::AddrChanged {
            contract: log.address.to_vec(),
            transaction_hash: transaction.hash.to_vec(),
            caller: call.caller.to_vec(),
            node: event.node.to_vec(),
            address: event.a.to_vec(),
        });
    }

    // NameChanged event
    if let Some(event) = events::NameChanged::match_and_decode(log) {
        events.name_changed.push(ens::NameChanged {
            contract: log.address.to_vec(),
            transaction_hash: transaction.hash.to_vec(),
            caller: call.caller.to_vec(),
            node: event.node.to_vec(),
            name: event.name,
        });
    }

    // ContenthashChanged event
    if let Some(event) = events::ContenthashChanged::match_and_decode(log) {
        let node = event.node;
        let hash = event.hash;
        events.content_hash_changed.push(ens::ContentHashChanged {
            contract: log.address.to_vec(),
            transaction_hash: transaction.hash.to_vec(),
            caller: call.caller.to_vec(),
            node: node.to_vec(),
            hash: hash.to_vec(),
        });
    }

    // TextChanged event
    if let Some(event) = events::TextChanged::match_and_decode(log) {
        events.text_changed.push(ens::TextChanged {
            contract: log.address.to_vec(),
            transaction_hash: transaction.hash.to_vec(),
            caller: call.caller.to_vec(),
            node: event.node.to_vec(),
            key: event.key,
            value: event.value,
        });
    }
}
