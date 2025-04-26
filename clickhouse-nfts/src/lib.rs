use common::{bytes_to_hex, to_global_sequence, update_genesis_clock};
use erc1155::pb::evm::erc1155::events::v1::Events as ERC1155Events;
use erc721::pb::evm::erc721::events::v1::Events as ERC721Events;
use substreams::pb::substreams::Clock;
use substreams::scalar::BigInt;
use substreams_database_change::pb::database::DatabaseChanges;
use substreams_database_change::tables::Tables;

use std::fmt;

#[derive(Debug, Clone, Copy)]
pub enum TokenStandard {
    ERC721,
    ERC1155,
}
impl fmt::Display for TokenStandard {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TokenStandard::ERC721 => write!(f, "ERC721"),
            TokenStandard::ERC1155 => write!(f, "ERC1155"),
        }
    }
}
#[derive(Debug, Clone, Copy)]
pub enum TransferType {
    Single,
    Batch,
}
impl fmt::Display for TransferType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TransferType::Single => write!(f, "Single"),
            TransferType::Batch => write!(f, "Batch"),
        }
    }
}

#[substreams::handlers::map]
pub fn db_out(mut clock: Clock, erc721_events: ERC721Events, erc1155_events: ERC1155Events) -> Result<DatabaseChanges, substreams::errors::Error> {
    substreams::log::info!(
        "block_num: {}, seconds: {}, nanos: {}",
        clock.number,
        clock.timestamp.unwrap().seconds,
        clock.timestamp.unwrap().nanos
    );
    let mut tables = Tables::new();
    clock = update_genesis_clock(clock);

    // Transfers
    for transfer in erc721_events.transfers {
        let row = tables.create_row(
            "nft_transfers",
            [
                ("block_num", clock.number.to_string()),
                ("tx_hash", bytes_to_hex(&transfer.tx_hash)),
                ("evt_index", transfer.log_index.to_string()),
            ],
        );
        row.set("contract", bytes_to_hex(&transfer.contract))
            .set("timestamp", clock.timestamp.unwrap().seconds)
            .set("global_sequence", to_global_sequence(&clock, transfer.log_index))
            .set("operator", bytes_to_hex(&transfer.from))
            .set("from", bytes_to_hex(&transfer.from))
            .set("to", bytes_to_hex(&transfer.to))
            .set("token_id", &transfer.token_id)
            .set("amount", 1)
            .set("transfer_type", TransferType::Single.to_string())
            .set("token_standard", TokenStandard::ERC721.to_string())
            .set("uri", transfer.uri.as_deref().unwrap_or(""))
            .set("symbol", transfer.symbol.as_deref().unwrap_or(""))
            .set("name", transfer.name.as_deref().unwrap_or(""));
    }

    // Transfers
    for transfer in erc1155_events.transfers {
        let Ok(amount) = u64::try_from(BigInt::try_from(&transfer.amount).expect("invalid amount")) else {
            continue;
        };
        let row = tables.create_row(
            "nft_transfers",
            [
                ("block_num", clock.number.to_string()),
                ("tx_hash", bytes_to_hex(&transfer.tx_hash)),
                ("evt_index", transfer.log_index.to_string()),
            ],
        );
        row.set("contract", bytes_to_hex(&transfer.contract))
            .set("timestamp", clock.timestamp.unwrap().seconds)
            .set("global_sequence", to_global_sequence(&clock, transfer.log_index))
            .set("from", bytes_to_hex(&transfer.from))
            .set("to", bytes_to_hex(&transfer.to))
            .set("token_id", &transfer.token_id)
            .set("operator", bytes_to_hex(&transfer.operator))
            .set("amount", amount)
            .set(
                "transfer_type",
                if amount == 1 {
                    TransferType::Single.to_string()
                } else {
                    TransferType::Batch.to_string()
                },
            )
            .set("token_standard", TokenStandard::ERC1155.to_string())
            .set("uri", transfer.uri.as_deref().unwrap_or(""))
            .set("symbol", "")
            .set("name", "");
    }

    // Transactions
    for tx in erc721_events.transactions {
        let row = tables.create_row("nft_transactions", [("tx_hash", bytes_to_hex(&tx.tx_hash))]);
        row.set("block_num", tx.block_number.to_string())
            .set("timestamp", tx.block_timestamp.to_string())
            .set("block_hash", bytes_to_hex(&tx.block_hash))
            .set("nonce", tx.nonce.to_string())
            .set("position", tx.position.to_string())
            .set("from_address", bytes_to_hex(&tx.from_address))
            .set("to_address", bytes_to_hex(&tx.to_address))
            .set("value", &tx.value)
            .set("tx_fee", &tx.tx_fee)
            .set("gas_price", &tx.gas_price)
            .set("gas_limit", tx.gas_limit.to_string())
            .set("gas_used", tx.gas_used.to_string())
            .set("cumulative_gas_used", tx.cumulative_gas_used.to_string())
            .set("max_fee_per_gas", &tx.max_fee_per_gas)
            .set("max_priority_fee_per_gas", &tx.max_priority_fee_per_gas)
            .set("input", bytes_to_hex(&tx.input))
            .set("type", tx.r#type.to_string())
            .set("v", bytes_to_hex(&tx.v))
            .set("r", bytes_to_hex(&tx.r))
            .set("s", bytes_to_hex(&tx.s));
    }
    for tx in erc1155_events.transactions {
        let row = tables.create_row("nft_transactions", [("tx_hash", bytes_to_hex(&tx.tx_hash))]);
        row.set("block_num", tx.block_number.to_string())
            .set("timestamp", tx.block_timestamp.to_string())
            .set("block_hash", bytes_to_hex(&tx.block_hash))
            .set("nonce", tx.nonce.to_string())
            .set("position", tx.position.to_string())
            .set("from_address", bytes_to_hex(&tx.from_address))
            .set("to_address", bytes_to_hex(&tx.to_address))
            .set("value", &tx.value)
            .set("tx_fee", &tx.tx_fee)
            .set("gas_price", &tx.gas_price)
            .set("gas_limit", tx.gas_limit.to_string())
            .set("gas_used", tx.gas_used.to_string())
            .set("cumulative_gas_used", tx.cumulative_gas_used.to_string())
            .set("max_fee_per_gas", &tx.max_fee_per_gas)
            .set("max_priority_fee_per_gas", &tx.max_priority_fee_per_gas)
            .set("input", bytes_to_hex(&tx.input))
            .set("type", tx.r#type.to_string())
            .set("v", bytes_to_hex(&tx.v))
            .set("r", bytes_to_hex(&tx.r))
            .set("s", bytes_to_hex(&tx.s));
    }

    Ok(tables.to_database_changes())
}
