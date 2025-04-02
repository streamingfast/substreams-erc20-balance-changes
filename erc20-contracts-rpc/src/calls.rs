use common::{bigint_to_int32, bytes32_to_string};
use substreams_abis::evm::token::erc20;
use substreams_abis::evm::tokens;

// ETH Call to retrieve ERC20 token Name
pub fn get_contract_name(address: Vec<u8>) -> Option<String> {
    erc20::functions::Name{}.call(address)
}

pub fn get_contract_name_bytes32(address: Vec<u8>) -> Option<String> {
    match (tokens::sai::functions::Name {}.call(address)) {
        Some(bytes) => {
            Some(bytes32_to_string(&bytes.to_vec()))
        },
        _ => None,
    }
}

// ETH Call to retrieve ERC20 token Symbol
pub fn get_contract_symbol(address: Vec<u8>) -> Option<String> {
    erc20::functions::Symbol {}.call(address)
}

pub fn get_contract_symbol_bytes32(address: Vec<u8>) -> Option<String> {
    match (tokens::sai::functions::Symbol {}.call(address)) {
        Some(bytes) => {
            Some(bytes32_to_string(&bytes.to_vec()))
        },
        _ => None,
    }
}

// ETH Call to retrieve ERC20 token Decimal
// Must be between 0 and 255
pub fn get_contract_decimals(address: Vec<u8>) -> Option<i32> {
    match (erc20::functions::Decimals{}.call(address)) {
        Some(decimals) => {
            bigint_to_int32(&decimals)
        },
        _ => None,
    }
}
