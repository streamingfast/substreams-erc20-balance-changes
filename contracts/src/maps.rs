
use substreams::{errors::Error, log, scalar::BigInt, Hex};
use proto::pb::evm::tokens::types::v1::{Algorithm, Contract, Events};
use substreams::store::{Deltas, DeltaBigInt, StoreGet, StoreGetString};

use crate::calls;

#[substreams::handlers::map]
pub fn map_events(store_erc20_transfers: Deltas<DeltaBigInt>, store_contract_creation: StoreGetString ) -> Result<Events, Error> {
    let mut events = Events::default();
    let mut index = 0;

    for deltas in store_erc20_transfers.deltas {
        // must be the 2nd block including ERC20 token transfer events per address
        // 1st transfer could be in the same block as contract creation which causes issues retrieving contract details
        if deltas.new_value != BigInt::from(2) { continue }
        let address = Hex::decode(&deltas.key).expect("invalid address");
        let contract = calls::get_contract(address.clone());
        if contract.is_none() { continue } // not valid ERC20 token contract
        let (name, symbol, decimals) = contract.unwrap();

        // get contract creation details
        let block_hash = store_contract_creation.get_first(format!("clock.id:{}", &deltas.key)).expect(format!("clock.id not found in {}", &deltas.key).as_str());
        let block_num = store_contract_creation.get_first(format!("clock.number:{}", &deltas.key)).expect("clock.number not found").parse::<u32>().expect("invalid clock.number");
        let timestamp = store_contract_creation.get_first(format!("clock.timestamp.seconds:{}", &deltas.key)).expect("clock.timestamp.seconds not found").parse::<u32>().expect("invalid clock.timestamp.seconds");
        let trx_hash = store_contract_creation.get_first(format!("trx.hash:{}", &deltas.key)).expect("trx.hash not found");
        let trx_from = store_contract_creation.get_first(format!("trx.from:{}", &deltas.key)).expect("trx.from not found");
        let trx_to = store_contract_creation.get_first(format!("trx.to:{}", &deltas.key)).expect("trx.to not found");

        events.contracts.push(Contract {
            // -- block (contract creation) --
            block_hash: Hex::decode(&block_hash).expect("invalid block hash"),
            block_num,
            timestamp,

            // -- transaction (contract creation) --
            transaction_id: Hex::decode(&trx_hash).expect("invalid transaction hash"),
            from: Hex::decode(&trx_from).expect("invalid transaction from"),
            to: Hex::decode(&trx_to).expect("invalid transaction to"),

            // -- contract --
            address,
            name,
            symbol,
            decimals: decimals.into(),

            // -- debug --
            algorithm: Algorithm::ContractCreation.into(),
        });
        index += 1;
    }
    log::info!("index: {:?}", index);
    Ok(events)
}
