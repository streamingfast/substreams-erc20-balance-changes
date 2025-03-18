use substreams_abis::evm::token::erc20::events::Transfer;
use substreams_ethereum::pb::eth::v2::{Call, Log, TransactionTrace};
use substreams_ethereum::Event;

use super::fishing::is_fishing_transfer;

pub fn get_erc20_transfer(trx: &TransactionTrace, call: &Call, log: &Log) -> Option<Transfer> {
    let transfer = Transfer::match_and_decode(log)?;
    if transfer.value.is_zero() {
        return None;
    }
    // ignore fishing transfers
    if is_fishing_transfer(trx, call) {
        return None;
    };
    Some(transfer)
}
