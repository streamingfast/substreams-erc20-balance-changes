// ETH Call to retrieve ERC20 token Name
pub fn get_contract_name(address: String) -> Option<String> {
    let call = abi::erc20::functions::Name{};
    log::info!("get_contract_name: {:?}", address);
    let hex = Hex::decode(address).unwrap();
    call.call(hex)
}

// ETH Call to retrieve ERC20 token Symbol
pub fn get_contract_symbol(address: String) -> Option<String> {
    let call = abi::erc20::functions::Symbol{};
    log::info!("get_contract_symbol: {:?}", address);
    let hex = Hex::decode(address).unwrap();
    call.call(hex)
}

// ETH Call to retrieve ERC20 token Decimal
pub fn get_contract_decimals(address: String) -> Option<BigInt> {
    let call = abi::erc20::functions::Decimals{};
    log::info!("get_contract_decimals: {:?}", address);
    let hex: Vec<u8> = Hex::decode(address).unwrap();
    log::info!("before call decimals:");
    call.call(hex)
}