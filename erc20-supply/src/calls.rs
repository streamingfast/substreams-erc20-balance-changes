use std::collections::HashMap;

use common::Address;
use substreams::{scalar::BigInt, Hex};
use substreams_abis::evm::nfts;
use substreams_ethereum::rpc::RpcBatch;

pub fn batch_total_supply<'a>(contracts: &'a [&Address], chunk_size: usize) -> HashMap<&'a Address, BigInt> {
    let mut results: HashMap<&'a Address, BigInt> = HashMap::new();
    for chunks in contracts.chunks(chunk_size) {
        let batch = chunks.iter().fold(RpcBatch::new(), |batch, address| {
            batch.add(nfts::boredapeyachtclub::functions::TotalSupply {}, address.to_vec())
        });
        let responses = batch.execute().expect("failed to execute functions::TotalSupply RpcBatch").responses;
        for (i, address) in chunks.iter().enumerate() {
            // TotalSupply
            if let Some(value) = RpcBatch::decode::<BigInt, nfts::boredapeyachtclub::functions::TotalSupply>(&responses[i]) {
                results.insert(address, value);
            } else {
                substreams::log::info!("Failed to decode functions::TotalSupply for address={:?}", Hex::encode(address));
            }
        }
    }
    results
}
