-- NFT Transfers Table
CREATE TABLE IF NOT EXISTS erc721_transfers (
    block_num       UInt64,
    tx_hash         FixedString(66),
    evt_index       UInt64,
    global_sequence UInt64,
    timestamp       DateTime(0, 'UTC'),
    contract        FixedString(42),
    token_id        String,
    from            FixedString(42),
    to              FixedString(42),

) ENGINE = ReplacingMergeTree(global_sequence)
ORDER BY (block_num, tx_hash, evt_index);


CREATE TABLE IF NOT EXISTS erc1155_transfers (
    block_num       UInt64,
    tx_hash         FixedString(66),
    evt_index       UInt64,
    global_sequence UInt64,
    timestamp       DateTime(0, 'UTC'),
    transfer_type   Enum8('Single' = 1, 'Batch' = 2),
    contract        FixedString(42),
    token_id        String,
    amount          UInt64,
    from            FixedString(42),
    to              FixedString(42),
    operator        FixedString(42),

) ENGINE = ReplacingMergeTree(global_sequence)
ORDER BY (block_num, tx_hash, evt_index);

CREATE TABLE IF NOT EXISTS nft_transfers (
    block_num       UInt64,
    tx_hash         FixedString(66),
    evt_index       UInt64,
    global_sequence UInt64,
    timestamp       DateTime(0, 'UTC'),
    token_standard  Enum8('ERC721' = 1, 'ERC1155' = 2),
    transfer_type   Enum8('Single' = 1, 'Batch' = 2),
    contract        FixedString(42),
    token_id        String,
    amount          UInt64,
    from            FixedString(42),
    to              FixedString(42),
    operator        FixedString(42),

    INDEX idx_global_sequence      (global_sequence)        TYPE minmax GRANULARITY 4,
    INDEX idx_tx_hash              (tx_hash)                TYPE bloom_filter GRANULARITY 4,
    INDEX idx_from                 (from)                   TYPE bloom_filter GRANULARITY 4,
    INDEX idx_to                   (to)                     TYPE bloom_filter GRANULARITY 4,
    INDEX idx_amount               (amount)                 TYPE minmax GRANULARITY 4,
    INDEX idx_contract             (contract)               TYPE bloom_filter GRANULARITY 4,
    INDEX idx_token_id             (token_id)               TYPE bloom_filter GRANULARITY 4,
    INDEX idx_transfer_type        (transfer_type)          TYPE set(8) GRANULARITY 4,
    INDEX idx_token_standard       (token_standard)         TYPE set(8) GRANULARITY 4,
) ENGINE = ReplacingMergeTree(global_sequence)
ORDER BY (block_num, tx_hash, evt_index);

CREATE MATERIALIZED VIEW IF NOT EXISTS erc721_to_nft_transfers_mv
TO nft_transfers AS
SELECT
    block_num,
    tx_hash,
    evt_index,
    global_sequence,
    timestamp,
    CAST('ERC721', 'Enum8(\'ERC721\' = 1, \'ERC1155\' = 2)') AS token_standard,
    CAST('Single', 'Enum8(\'Single\' = 1, \'Batch\' = 2)') AS transfer_type,
    contract,
    token_id,
    1 AS amount,
    `from`,
    `to`,
    '' AS operator
FROM erc721_transfers;

CREATE MATERIALIZED VIEW IF NOT EXISTS erc1155_to_nft_transfers_mv
TO nft_transfers AS
SELECT
    block_num,
    tx_hash,
    evt_index,
    global_sequence,
    timestamp,
    CAST('ERC1155', 'Enum8(\'ERC721\' = 1, \'ERC1155\' = 2)') AS token_standard,
    transfer_type,
    contract,
    token_id,
    amount,
    `from`,
    `to`,
    operator
FROM erc1155_transfers;

CREATE TABLE IF NOT EXISTS nft_tokens (
    contract        FixedString(42),
    token_id        String,
    token_standard  Enum8('ERC721' = 1, 'ERC1155' = 2),
    global_sequence UInt64,
    uri             String DEFAULT '',
    symbol          String DEFAULT '',
    name            String DEFAULT '',

    INDEX idx_global_sequence       (global_sequence)       TYPE minmax GRANULARITY 4,
    INDEX idx_uri                   (uri)                   TYPE bloom_filter GRANULARITY 4,
    INDEX idx_symbol                (symbol)                TYPE bloom_filter GRANULARITY 4,
    INDEX idx_name                  (name)                  TYPE bloom_filter GRANULARITY 4,
    INDEX idx_token_standard        (token_standard)        TYPE set(8) GRANULARITY 4,
) ENGINE = ReplacingMergeTree(global_sequence)
ORDER BY (contract, token_id);

-- CREATE TABLE IF NOT EXISTS nft_offchain (
--     contract        FixedString(42),
--     token_id        String,
--     token_standard  Enum8('ERC721' = 1, 'ERC1155' = 2),
--     global_sequence UInt64,
--     metadata_json   String DEFAULT '',
--     image_url       String DEFAULT '',
--     image_id        String DEFAULT '',
--     description     String DEFAULT '',
--     rarity          LowCardinality(String) DEFAULT '',

--     INDEX idx_global_sequence       (global_sequence)       TYPE minmax GRANULARITY 4,
--     INDEX idx_owner                 (owner)                 TYPE bloom_filter GRANULARITY 4,
--     INDEX idx_uri                   (uri)                   TYPE bloom_filter GRANULARITY 4,
--     INDEX idx_symbol                (symbol)                TYPE bloom_filter GRANULARITY 4,
--     INDEX idx_token_standard        (token_standard)        TYPE set(8) GRANULARITY 4,
--     INDEX idx_rarity                (rarity)                TYPE set(32) GRANULARITY 4,
-- ) ENGINE = ReplacingMergeTree(global_sequence)
-- ORDER BY (contract, token_id);


CREATE TABLE IF NOT EXISTS nft_transactions (
    block_num                UInt64,
    block_hash               FixedString(66),
    timestamp                DateTime(0, 'UTC'),
    tx_hash                  FixedString(66),
    nonce                    UInt64,
    position                 UInt32,
    from_address             FixedString(42),
    to_address               FixedString(42),
    value                    String,
    tx_fee                   String,
    gas_price                String,
    gas_limit                UInt64,
    gas_used                 UInt64,
    cumulative_gas_used      UInt64,
    max_fee_per_gas          String,
    max_priority_fee_per_gas String,
    input                    String,
    type                     Int32,
    v                        String,
    r                        String,
    s                        String

) ENGINE = ReplacingMergeTree
ORDER BY tx_hash;
