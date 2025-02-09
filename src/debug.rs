use substreams::store::{StoreGet, StoreGetBigInt};

#[substreams::handlers::map]
pub fn map_unknown_balance_changes(balance_changes: BalanceChanges) -> Result<BalanceChanges, Error> {
    let unknown_balance_changes: Vec<BalanceChange> = balance_changes.balance_changes
        .iter()
        .filter(|change| change.change_type == BalanceChangeType::Unspecified as i32)
        .cloned()
        .collect();

    Ok(BalanceChanges {
        balance_changes: unknown_balance_changes,
    })
}

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