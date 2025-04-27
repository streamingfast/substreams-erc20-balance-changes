use common::is_zero_address;
use proto::pb::evm::erc721::events::v1::Token;
use substreams_abis::evm::token::erc721;
use substreams_ethereum::pb::eth::v2 as eth;
use substreams_ethereum::Event as _;

pub fn get_mints<'a>(blk: &'a eth::Block) -> impl Iterator<Item = Token> + 'a {
    blk.receipts().flat_map(move |receipt| {
        let contract = &receipt.transaction.to;
        receipt.receipt.logs.iter().filter_map(move |log| {
            if let Some(event) = erc721::events::Transfer::match_and_decode(log) {
                if is_zero_address(event.from.as_ref() as &[u8]) {
                    Some(Token {
                        contract: contract.to_vec().into(),
                        token_id: event.token_id.to_string(),
                        ..Default::default()
                    })
                } else {
                    None
                }
            } else {
                None
            }
        })
    })
}
