use substreams_ethereum::{pb::eth::v2::Call, Function};
use crate::abi::functions::{SetMetadata, SetName, SetSymbol};

pub struct SetNameSymbol {
    pub name: Option<String>,
    pub symbol: Option<String>,
    pub decimals: Option<i32>,
}

pub fn get_metadata<'a>(call: &'a Call) -> Option<SetNameSymbol> {
    // extract both Name & Symbol from the call
    if let Some((name, symbol)) = get_name_symbol(call) {
        return Some(SetNameSymbol {
            name: Some(name),
            symbol: Some(symbol),
            decimals: None,
        })
    }

    // extracted only name from the call
    if let Some(name) = get_name(call) {
        return Some(SetNameSymbol {
            name: Some(name),
            symbol: None,
            decimals: None,
        })
    }

    // extracted only symbol from the call
    if let Some(symbol) = get_symbol(call) {
        return Some(SetNameSymbol {
            name: None,
            symbol: Some(symbol),
            decimals: None,
        })
    }
    None
}

pub fn get_symbol<'a>(call: &'a Call) -> Option<String> {
    if let Some(result) = SetSymbol::match_and_decode(call) {
        return Some(result.symbol);
    }
    None
}

pub fn get_name<'a>(call: &'a Call) -> Option<String> {
    if let Some(result) = SetName::match_and_decode(call) {
        return Some(result.name);
    }
    None
}

pub fn get_name_symbol<'a>(call: &'a Call) -> Option<(String, String)> {
    if let Some(result) = SetMetadata::match_and_decode(call) {
        return Some((result.name, result.symbol));
    }
    None
}
