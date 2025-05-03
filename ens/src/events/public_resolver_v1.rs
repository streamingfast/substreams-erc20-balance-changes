use common::{bigint_to_uint64, bytes_to_address};
use proto::pb::evm::ens::v1 as ens;
use substreams_abis::evm::ens::v1::publicresolver::events;
use substreams_ethereum::{
    pb::eth::v2::{Call, Log, TransactionTrace},
    Event,
};

pub fn insert_public_resolver_v1<'a>(events: &mut ens::Events, transaction: &'a TransactionTrace, call: &'a Call, log: &'a Log) {
    // AddrChanged event
    if let Some(event) = events::AddressChanged::match_and_decode(log) {
        let coin_type = if bigint_to_uint64(&event.coin_type).is_some() {
            event.coin_type.to_u64()
        } else {
            return;
        };
        events.address_changed.push(ens::AddressChanged {
            contract: log.address.to_vec(),
            transaction_hash: transaction.hash.to_vec(),
            caller: call.caller.to_vec(),
            ordinal: log.ordinal,
            node: event.node.to_vec(),
            address: bytes_to_address(&event.new_address),
            coin_type: Some(coin_type),
        });
    }

    // NameChanged event
    if let Some(event) = events::NameChanged::match_and_decode(log) {
        events.name_changed.push(ens::NameChanged {
            contract: log.address.to_vec(),
            transaction_hash: transaction.hash.to_vec(),
            caller: call.caller.to_vec(),
            ordinal: log.ordinal,
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
            ordinal: log.ordinal,
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
            ordinal: log.ordinal,
            node: event.node.to_vec(),
            indexed_key: event.indexed_key.hash.to_vec(),
            key: event.key,
            value: event.value,
        });
    }
}
