use substreams::scalar::BigInt;
use substreams_abis::evm::token::erc20;

// ETH Call to retrieve ERC20 balance
pub fn get_balance_of(account: Vec<u8>, address: Vec<u8>) -> Option<BigInt> {
    erc20::functions::BalanceOf{account}.call(address)
}
