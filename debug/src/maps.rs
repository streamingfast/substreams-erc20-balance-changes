use std::collections::HashSet;

use substreams::{
    errors::Error,
    log,
    pb::substreams::Clock,
    scalar::{BigDecimal, BigInt},
    store::{StoreGet, StoreGetBigInt},
};

use crate::pb::erc20::types::v1::{BalanceChangeStat, BalanceChangeStats, Events};

#[substreams::handlers::map]
pub fn balance_change_stats(clock: Clock, events: Events, store: StoreGetBigInt) -> Result<BalanceChangeStats, Error> {
    let mut current_type1_balance_changes = 0;
    let mut current_type2_balance_changes = 0;
    let mut current_balance_changes = 0;
    let mut current_transfers = 0;
    let mut current_transfers_not_matched = 0;
    let mut logs = HashSet::new();

    // current
    for balance_change in events.balance_changes {
        let key = format!("{}:{}", balance_change.transaction_id, balance_change.log_index);
        logs.insert(key);

        match balance_change.balance_change_type {
            1 => {
                current_type1_balance_changes += 1;
                current_balance_changes += 1;
            }
            2 => {
                current_type2_balance_changes += 1;
                current_balance_changes += 1;
            }
            _ => {}
        }
    }
    for transfer in events.transfers {
        let key = format!("{}:{}", transfer.transaction_id, transfer.log_index);
        if !logs.contains(&key) {
            current_transfers_not_matched += 1;
            log::info!("Transfer not matched: {:?} (log_index={:?})", transfer.transaction_id, transfer.log_index);
        }
        current_transfers += 1;
    }

    let mut current_valid_rate = BigDecimal::from(1);
    if current_transfers > 0 {
        current_valid_rate = current_valid_rate - BigDecimal::from(current_transfers_not_matched) / BigDecimal::from(current_transfers);
    }

    // total
    let total_type1_balance_changes = store.get_last("balance_changes_type_1").unwrap_or(BigInt::from(0)).to_u64();
    let total_type2_balance_changes = store.get_last("balance_changes_type_2").unwrap_or(BigInt::from(0)).to_u64();
    let total_balance_changes = store.get_last("balance_changes").unwrap_or(BigInt::from(0)).to_u64();
    let total_transfers = store.get_last("transfers").unwrap_or(BigInt::from(0)).to_u64();
    let total_transfers_not_matched = store.get_last("transfers_not_matched").unwrap_or(BigInt::from(0)).to_u64();
    let mut total_valid_rate = BigDecimal::from(1);

    if total_transfers > 0 {
        total_valid_rate = total_valid_rate - BigDecimal::from(total_transfers_not_matched) / BigDecimal::from(total_transfers);
    }

    Ok(BalanceChangeStats {
        // current
        current: Some(BalanceChangeStat {
            type1_balance_changes: current_type1_balance_changes,
            type2_balance_changes: current_type2_balance_changes,
            balance_changes: current_balance_changes,
            transfers: current_transfers,
            transfers_not_matched: current_transfers_not_matched,
            valid_rate: current_valid_rate.with_prec(4).to_string(),
        }),

        // total
        total: Some(BalanceChangeStat {
            type1_balance_changes: total_type1_balance_changes,
            type2_balance_changes: total_type2_balance_changes,
            balance_changes: total_balance_changes,
            transfers: total_transfers,
            transfers_not_matched: total_transfers_not_matched,
            valid_rate: total_valid_rate.with_prec(4).to_string(),
        }),

        // block
        block_number: clock.number,
        timestamp: clock.timestamp,
    })
}
