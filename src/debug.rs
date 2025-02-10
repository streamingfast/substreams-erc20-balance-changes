use substreams::{errors::Error, pb::substreams::Clock, scalar::{BigDecimal, BigInt}, store::{StoreGet, StoreGetBigInt}};

use crate::pb::erc20::types::v1::BalanceChangeStats;

#[substreams::handlers::map]
pub fn balance_change_stats(clock: Clock, store: StoreGetBigInt) -> Result<BalanceChangeStats, Error> {
    let type_1 = store.get_last("type1").unwrap_or(BigInt::from(0));
    let type_2 = store.get_last("type2").unwrap_or(BigInt::from(0));
    let total = store.get_last("total").unwrap_or(BigInt::from(0));
    let mut valid_rate = BigDecimal::from(0);
    if !total.is_zero() {
        valid_rate = (BigDecimal::from(type_1.clone()) + BigDecimal::from(type_2.clone())) / BigDecimal::from(total.clone());
    }

    Ok(BalanceChangeStats {
        type0_count: store.get_last("type0").unwrap_or(BigInt::from(0)).to_u64(),
        type1_count: type_1.to_u64(),
        type2_count: type_2.to_u64(),
        total_count: total.to_u64(),
        block_number: clock.number,
        valid_rate: valid_rate.to_string(),
    })
}