use proto::pb::evm::erc1155::events::v1::{Token, Transfer};
use substreams_abis::evm::token::erc1155::events::{TransferBatch, TransferSingle, Uri as UriEvent};
use substreams_ethereum::{pb::eth::v2 as eth, Event as _};

// Extracts ERC1155 Transfer events (both TransferSingle and TransferBatch)
pub fn get_transfers<'a>(blk: &'a eth::Block) -> impl Iterator<Item = Transfer> + 'a {
    blk.receipts().flat_map(move |receipt| {
        receipt.receipt.logs.iter().flat_map(move |log| {
            // Try TransferSingle
            if let Some(event) = TransferSingle::match_and_decode(log) {
                let token_id = event.id.to_string();
                let amount = event.value.to_string();
                return vec![Transfer {
                    block_num: blk.number,
                    tx_hash: receipt.transaction.hash.to_vec().into(),
                    log_index: log.block_index as u64,
                    contract: log.address.to_vec().into(),
                    operator: event.operator.into(),
                    from: event.from.into(),
                    to: event.to.into(),
                    token_id,
                    amount,
                    uri: None,
                }]
                .into_iter();
            }
            // Try TransferBatch
            if let Some(event) = TransferBatch::match_and_decode(log) {
                return event
                    .ids
                    .iter()
                    .zip(event.values.iter())
                    .map(|(id, value)| Transfer {
                        block_num: blk.number,
                        tx_hash: receipt.transaction.hash.to_vec().into(),
                        log_index: log.block_index as u64,
                        contract: log.address.to_vec().into(),
                        operator: event.operator.to_vec().into(),
                        from: event.from.to_vec().into(),
                        to: event.to.to_vec().into(),
                        token_id: id.into(),
                        amount: value.into(),
                        uri: None,
                    })
                    .collect::<Vec<_>>()
                    .into_iter();
            }
            // Not a transfer event
            Vec::new().into_iter()
        })
    })
}

// Extracts ERC1155 Uri events
pub fn get_tokens<'a>(blk: &'a eth::Block) -> impl Iterator<Item = Token> + 'a {
    blk.receipts().flat_map(move |receipt| {
        receipt.receipt.logs.iter().filter_map(move |log| {
            if let Some(event) = UriEvent::match_and_decode(log) {
                Some(Token {
                    block_num: blk.number,
                    tx_hash: receipt.transaction.hash.to_vec().into(),
                    log_index: log.block_index as u64,
                    contract: log.address.to_vec().into(),
                    token_id: event.id.into(),
                    uri: event.value.into(),
                })
            } else {
                None
            }
        })
    })
}
