use std::collections::HashMap;

use common::Address;
use substreams::scalar::BigInt;
use substreams_ethereum::{
    pb::eth::rpc::{RpcGetBalanceRequest, RpcGetBalanceRequests},
    rpc::eth_get_balance,
};

// Returns the token URI.
pub fn batch_eth_balance_of<'a>(block: u64, owners: &'a [&Address], chunk_size: usize) -> HashMap<&'a Address, BigInt> {
    let mut results: HashMap<&Address, BigInt> = HashMap::with_capacity(owners.len());

    for chunk in owners.chunks(chunk_size) {
        let mut requests = RpcGetBalanceRequests {
            requests: Vec::with_capacity(chunk.len()),
        };
        for owner in chunk {
            requests.requests.push(RpcGetBalanceRequest {
                address: owner.to_vec(),
                block: block.to_string(),
            });
        }
        let balances = eth_get_balance(&requests);
        for (i, owner) in chunk.iter().enumerate() {
            let response = &balances.responses[i];
            if response.failed {
                continue;
            }
            results.insert(owner, BigInt::from_unsigned_bytes_be(&response.balance));
        }
    }
    results
}
