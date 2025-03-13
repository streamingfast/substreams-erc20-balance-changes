use substreams::scalar::BigInt;
use substreams_abis::evm::token::erc20;

// ETH Call to retrieve ERC20 token Name
pub fn get_contract_name(address: Vec<u8>) -> Option<String> {
    let method = erc20::functions::Name{};
    method.call(address)
}

// ETH Call to retrieve ERC20 token Symbol
pub fn get_contract_symbol(address: Vec<u8>) -> Option<String> {
    let method = erc20::functions::Symbol{};
    method.call(address)
}

// ETH Call to retrieve ERC20 token Decimal
pub fn get_contract_decimals(address: Vec<u8>) -> Option<BigInt> {
    let method = erc20::functions::Decimals{};
    method.call(address)
}

pub fn get_contract(address: Vec<u8>) -> Option<(String, String, BigInt)> {
    let name = get_contract_name(address.clone());
    let symbol = get_contract_symbol(address.clone());
    let decimals = get_contract_decimals(address.clone());

    // all must be required to be a valid ERC20 token contract
    match (name, symbol, decimals) {
        (Some(name), Some(symbol), Some(decimals)) => {
            Some((name, symbol, decimals))
        }
        _ => None,
    }
}