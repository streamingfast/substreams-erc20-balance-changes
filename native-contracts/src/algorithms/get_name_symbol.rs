use substreams_ethereum::pb::eth::v2::Call;
use substreams_abis::evm::token::erc20_name_symbol::functions::{SetSymbol, SetName};

pub fn get_symbol<'a>(call: &'a Call) -> Option<String> {
    match SetSymbol::decode(call) {
        Ok(set_symbol) => Some(set_symbol.symbol),
        Err(_) => None,
    }
}

pub fn get_name<'a>(call: &'a Call) -> Option<String> {
    match SetName::decode(call) {
        Ok(result) => Some(result.name),
        Err(_) => None,
    }
}

// TO-DO: Implement Name/Symbol modifications after contract creation
// https://github.com/pinax-network/substreams-evm-tokens/issues/13