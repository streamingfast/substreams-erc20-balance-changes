use std::collections::HashMap;

use common::Address;
use substreams::{log, scalar::BigInt};
use substreams_ethereum::{
    pb::eth::rpc::{RpcGetBalanceRequest, RpcGetBalanceRequests},
    rpc::eth_get_balance,
};

// Returns the token URI.
pub fn batch_eth_balance_of<'a>(block: u64, accounts: &'a [&Address], chunk_size: usize) -> HashMap<&'a Address, BigInt> {
    let mut results: HashMap<&Address, BigInt> = HashMap::with_capacity(accounts.len());

    for chunk in accounts.chunks(chunk_size) {
        let mut requests = RpcGetBalanceRequests {
            requests: Vec::with_capacity(chunk.len()),
        };
        for account in chunk {
            requests.requests.push(RpcGetBalanceRequest {
                address: account.to_vec(),
                block: format!("{:#x}", block), // to hex
            });
        }
        let balances = eth_get_balance(&requests);
        for (i, account) in chunk.iter().enumerate() {
            let response = &balances.responses[i];
            if response.failed {
                continue;
            }
            results.insert(account, BigInt::from_unsigned_bytes_be(&response.balance));
        }
        log::info!("Processed {}/{} balance requests", chunk.len(), accounts.len());
    }
    results
}
