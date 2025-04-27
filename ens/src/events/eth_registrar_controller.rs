use proto::pb::evm::tokens::ens::v1 as ens;
use substreams_abis::evm::ens::v1::ethregistrarcontroller::events;
use substreams_ethereum::{
    pb::eth::v2::{Call, Log, TransactionTrace},
    Event,
};

pub fn insert_eth_registrar_controller<'a>(events: &mut ens::Events, transaction: &'a TransactionTrace, call: &'a Call, log: &'a Log) {
    // NameRegistered event
    if let Some(event) = events::NameRegistered::match_and_decode(log) {
        events.name_registered.push(ens::NameRegistered {
            contract: log.address.to_vec(),
            transaction_hash: transaction.hash.to_vec(),
            caller: call.caller.to_vec(),
            name: event.name,
            label: event.label.to_vec(),
            owner: event.owner.to_vec(),
            base_cost: event.base_cost.to_u64(),
            expires: event.expires.to_u64(),
        });
    }

    // NameRenewed event
    if let Some(event) = events::NameRenewed::match_and_decode(log) {
        events.name_renewed.push(ens::NameRenewed {
            contract: log.address.to_vec(),
            transaction_hash: transaction.hash.to_vec(),
            caller: call.caller.to_vec(),
            name: event.name,
            label: event.label.to_vec(),
            cost: event.cost.to_u64(),
            expires: event.expires.to_u64(),
        });
    }
}
