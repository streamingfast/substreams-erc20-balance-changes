use proto::pb::evm::ens::v1 as ens;
use substreams_abis::evm::ens::v1::reverseregistrar::events;
use substreams_ethereum::{
    pb::eth::v2::{Call, Log, TransactionTrace},
    Event,
};

pub fn insert_reverse_registrar<'a>(events: &mut ens::Events, transaction: &'a TransactionTrace, call: &'a Call, log: &'a Log) {
    // ReverseClaimed event
    if let Some(event) = events::ReverseClaimed::match_and_decode(log) {
        events.reverse_claimed.push(ens::ReverseClaimed {
            contract: log.address.to_vec(),
            transaction_hash: transaction.hash.to_vec(),
            caller: call.caller.to_vec(),
            ordinal: log.ordinal,
            node: event.node.to_vec(),
            address: event.addr.to_vec(),
        });
    }
}
