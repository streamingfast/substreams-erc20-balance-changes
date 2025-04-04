use common::update_genesis_clock;
use proto::pb::evm::tokens::balances::v1::Events as EventsBalances;
// use proto::pb::evm::tokens::contracts::v1::Events as EventsContracts;
// use proto::pb::evm::tokens::uniswap::v2::Events as EventsPricesUniswapV2;
// use proto::pb::evm::tokens::uniswap::v3::Events as EventsPricesUniswapV3;
use substreams::{errors::Error, pb::substreams::Clock};
use substreams_database_change::pb::database::DatabaseChanges;

use crate::balances::process_balances;

#[substreams::handlers::map]
// pub fn db_out(mut clock: Clock, erc20_balances: EventsBalances, erc20_balances_rpc: EventsBalances, erc20_contracts: EventsContracts, erc20_contracts_rpc: EventsContracts, native_balances: EventsBalances, uniswap_v2: EventsPricesUniswapV2, uniswap_v3: EventsPricesUniswapV3) -> Result<DatabaseChanges, Error> {
pub fn db_out(mut clock: Clock, erc20_balances: EventsBalances, erc20_balances_rpc: EventsBalances, native_balances: EventsBalances) -> Result<DatabaseChanges, Error> {
    let mut tables = substreams_database_change::tables::Tables::new();
    clock = update_genesis_clock(clock);

    // -- Balances/Transfers --
    let mut index = 0;
    index = process_balances("erc20_", &mut tables, &clock, erc20_balances, index);
    index = process_balances("erc20_", &mut tables, &clock, erc20_balances_rpc, index);
    index = process_balances("native_", &mut tables, &clock, native_balances, index);

    // // Pre-compute frequently used values
    // let block_num = clock.number.to_string();
    // let date = clock_to_date(&clock);

    // // -- balance changes --
    // // Process ERC-20 balance changes
    // for event in erc20.balance_changes {
    //     process_balance_change(&mut tables, &clock, &block_num, &date, event);
    // }
    // for event in erc20_rpc.balance_changes {
    //     process_balance_change(&mut tables, &clock, &block_num, &date, event);
    // }

    // // Process native balance changes
    // for event in native.balance_changes {
    //     process_balance_change(&mut tables, &clock, &block_num, &date, event);
    // }

    // // -- transfers --
    // // Process ERC-20 transfers
    // for event in erc20.transfers {
    //     process_transfer(&mut tables, &clock, &block_num, &date, event);
    // }

    // // Process native transfers
    // for event in native.transfers {
    //     process_transfer(&mut tables, &clock, &block_num, &date, event);
    // }

    // // -- contract changes --
    // for event in erc20_contracts.contract_changes {
    //     process_contract_change(&mut tables, &clock, &block_num, &date, event);
    // }
    // for event in erc20_contracts_rpc.contract_changes {
    //     process_contract_change(&mut tables, &clock, &block_num, &date, event);
    // }

    // // -- contract creations --
    // for event in erc20_contracts.contract_creations {
    //     process_contract_creation(&mut tables, &clock, event);
    // }
    // for event in erc20_contracts_rpc.contract_creations {
    //     process_contract_creation(&mut tables, &clock, event);
    // }

    // // -- Uniswap V2: created pairs --
    // for event in prices_uniswap_v2.pairs_created {
    //     process_uniswap_v2_pair_created(&mut tables, &clock, &block_num, &date, event);
    // }

    // // -- Uniswap V2: syncs --
    // for event in prices_uniswap_v2.syncs {
    //     process_uniswap_v2_sync(&mut tables, &clock, &block_num, &date, event);
    // }

    // // -- Uniswap V2: swaps --
    // for event in prices_uniswap_v2.swaps {
    //     process_uniswap_v2_swap(&mut tables, &clock, &block_num, &date, event);
    // }

    Ok(tables.to_database_changes())
}
