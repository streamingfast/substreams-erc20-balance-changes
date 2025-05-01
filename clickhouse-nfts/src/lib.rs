use common::{bytes_to_hex, to_global_sequence, update_genesis_clock};
use proto::pb::evm::erc1155::events::v1::Events as ERC1155Events;
use proto::pb::evm::erc721::events::v1::Events as ERC721Events;
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
    let mut tables = Tables::new();
    clock = update_genesis_clock(clock);
    let mut i = 0;

    // Transfers
    for transfer in erc721_events.transfers {
        i += 1;
        let row = tables.create_row(
            "erc721_transfers",
            [
                ("block_num", clock.number.to_string()),
                ("tx_hash", bytes_to_hex(&transfer.tx_hash)),
                ("evt_index", transfer.log_index.to_string()),
            ],
        );
        row.set("global_sequence", to_global_sequence(&clock, i))
            .set("timestamp", clock.timestamp.unwrap().seconds)
            .set("contract", bytes_to_hex(&transfer.contract))
            .set("token_id", &transfer.token_id)
            .set("from", bytes_to_hex(&transfer.from))
            .set("to", bytes_to_hex(&transfer.to));
    }

    // Transfers
    for transfer in erc1155_events.transfers {
        let Ok(amount) = u64::try_from(BigInt::try_from(&transfer.amount).expect("invalid amount")) else {
            continue;
        };
        i += 1;
        let row = tables.create_row(
            "erc1155_transfers",
            [
                ("block_num", clock.number.to_string()),
                ("tx_hash", bytes_to_hex(&transfer.tx_hash)),
                ("evt_index", transfer.log_index.to_string()),
            ],
        );
        row.set("global_sequence", to_global_sequence(&clock, i))
            .set("timestamp", clock.timestamp.unwrap().seconds)
            .set(
                "transfer_type",
                if amount == 1 {
                    TransferType::Single.to_string()
                } else {
                    TransferType::Batch.to_string()
                },
            )
            .set("contract", bytes_to_hex(&transfer.contract))
            .set("token_id", &transfer.token_id)
            .set("amount", amount)
            .set("from", bytes_to_hex(&transfer.from))
            .set("to", bytes_to_hex(&transfer.to))
            .set("operator", bytes_to_hex(&transfer.operator));
    }

    for token in erc721_events.tokens {
        i += 1;
        let row = tables.create_row("nft_tokens", [("contract", bytes_to_hex(&token.contract)), ("token_id", token.token_id)]);
        row.set("global_sequence", to_global_sequence(&clock, i))
            .set("block_num", clock.number.to_string())
            .set("tx_hash", bytes_to_hex(&token.tx_hash))
            .set("evt_index", token.log_index.to_string())
            .set("timestamp", clock.timestamp.unwrap().seconds)
            .set("token_standard", TokenStandard::ERC721.to_string())
            .set("uri", token.uri.as_deref().unwrap_or(""))
            .set("symbol", token.symbol.as_deref().unwrap_or(""))
            .set("name", token.name.as_deref().unwrap_or(""));
    }

    for token in erc1155_events.tokens {
        i += 1;
        let row = tables.create_row("nft_tokens", [("contract", bytes_to_hex(&token.contract)), ("token_id", token.token_id)]);
        row.set("global_sequence", to_global_sequence(&clock, i))
            .set("block_num", clock.number.to_string())
            .set("tx_hash", bytes_to_hex(&token.tx_hash))
            .set("evt_index", token.log_index.to_string())
            .set("timestamp", clock.timestamp.unwrap().seconds)
            .set("token_standard", TokenStandard::ERC1155.to_string())
            .set("uri", token.uri)
            .set("symbol", "")
            .set("name", "");
    }

    Ok(tables.to_database_changes())
}
