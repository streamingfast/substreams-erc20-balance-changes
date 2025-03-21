use substreams::scalar::BigInt;
use substreams_abis::evm::token::erc20;

// ETH Call to retrieve ERC20 token Name
pub fn get_contract_name(address: Vec<u8>) -> Option<String> {
    erc20::functions::Name {}.call(address)
}

// ETH Call to retrieve ERC20 token Symbol
pub fn get_contract_symbol(address: Vec<u8>) -> Option<String> {
    erc20::functions::Symbol {}.call(address)
}

// ETH Call to retrieve ERC20 token Decimal
pub fn get_contract_decimals(address: Vec<u8>) -> Option<BigInt> {
    // decimals must be uint8 range
    erc20::functions::Decimals {}
        .call(address)
        .filter(|decimals| *decimals >= BigInt::from(0) && *decimals <= BigInt::from(255))
}

pub fn get_contract(address: &[u8]) -> Option<(String, String, BigInt)> {
    // exit early if name call fails
    get_contract_name(address.into())
        .and_then(|name| get_contract_symbol(address.into()).map(|symbol| (name, symbol)))
        .and_then(|(name, symbol)| get_contract_decimals(address.into()).map(|decimals| (name, symbol, decimals)))
}
