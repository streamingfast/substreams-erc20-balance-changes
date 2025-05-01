-- ERC721 Transfers Table
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

-- ERC1155 Transfers Table
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

-- Combined ERC721/ERC1155 Transfers Table
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
    'ERC721' AS token_standard,
    'Single' AS transfer_type,
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
    'ERC1155' AS token_standard,
    transfer_type,
    contract,
    token_id,
    amount,
    `from`,
    `to`,
    operator
FROM erc1155_transfers;

-- Onchain metadata
CREATE TABLE IF NOT EXISTS nft_tokens (
    contract        FixedString(42),
    token_id        String,
    block_num       UInt64,
    tx_hash         FixedString(66),
    evt_index       UInt64,
    global_sequence UInt64,
    timestamp       DateTime(0, 'UTC'),
    token_standard  Enum8('ERC721' = 1, 'ERC1155' = 2),
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

-- Offchain Metadata
CREATE TABLE IF NOT EXISTS metadata (
    contract            FixedString(42),
    token_id            String,
    timestamp           DateTime(0, 'UTC') MATERIALIZED now(),
    metadata_json       String DEFAULT '',                  -- metadata JSON response
    image_uri           String DEFAULT '',                  -- image URI
    description         String DEFAULT '',                  -- description from metadata
    rarity              LowCardinality(String) DEFAULT '',  -- rarity from metadata
    image_saved         Boolean DEFAULT false,              -- whether the image has been saved to S3
    image_mime_type     LowCardinality(String) DEFAULT '',  -- image MIME type
    image_size_bytes    UInt64 DEFAULT 0,                   -- image size in bytes

    INDEX idx_timestamp             (timestamp)            TYPE minmax GRANULARITY 4,
    INDEX idx_image_saved           (image_saved)          TYPE set(2) GRANULARITY 4,
    INDEX idx_description           (description)          TYPE bloom_filter GRANULARITY 4,
    INDEX idx_rarity                (rarity)               TYPE set(32) GRANULARITY 4,
) ENGINE = ReplacingMergeTree(timestamp)
ORDER BY (contract, token_id);



-- Metadata requests and query results
CREATE TABLE IF NOT EXISTS metadata_requests (
    contract        FixedString(42),
    token_id        String,
    timestamp       DateTime(0, 'UTC') MATERIALIZED now(),
    uri             String,                         -- metadata URI being requested
    url             String,                         -- actual URL being requested (can be different from uri, i.e. ipfs://xxx vs https://ipfs.com/xxx)
    status          Enum8('ok' = 0, 'timeout' = 1, 'host_not_found' = 2, 'connection_refused' = 3, 'connection_reset' = 4, 'http_error' = 5, 'other_error' = 6),
    http_code       UInt16 DEFAULT 0,               -- HTTP status code
    success         Boolean DEFAULT false,          -- whether the request was successful
    metadata_json   String DEFAULT '',              -- metadata JSON response
    duration_ms     UInt64 DEFAULT 0,               -- duration of the request in milliseconds

    INDEX idx_success       (success) TYPE set(2) GRANULARITY 4,
    INDEX idx_http_code     (http_code) TYPE set(32) GRANULARITY 4,
    INDEX idx_status        (status)  TYPE set(16) GRANULARITY 4,
    INDEX idx_duration_ms   (duration_ms) TYPE minmax GRANULARITY 4,
) ENGINE = MergeTree
ORDER BY (contract, token_id, timestamp);

-- Spam Scoring
CREATE TABLE IF NOT EXISTS spam_scoring (
    contract        FixedString(42),
    timestamp       DateTime(0, 'UTC') MATERIALIZED now(),  -- when the score was assigned
    spam_score      UInt16,                                 -- score from 0 (not spam) to 100 (definitely spam)
    classification  LowCardinality(String) DEFAULT '',      -- ? e.g., 'spam', 'not_spam', 'suspicious'
    reason          String DEFAULT '',                      -- ? optional: explanation or model output
    source          LowCardinality(String) DEFAULT '',      -- ? which service/model provided this score

    INDEX idx_timestamp      (timestamp)      TYPE minmax GRANULARITY 4,
    INDEX idx_spam_score     (spam_score)     TYPE minmax GRANULARITY 4,
    INDEX idx_classification (classification) TYPE set(8)  GRANULARITY 4,
    INDEX idx_source         (source)         TYPE set(8)  GRANULARITY 4,
    INDEX idx_reason         (reason)         TYPE bloom_filter GRANULARITY 4,
) ENGINE = ReplacingMergeTree(timestamp)
ORDER BY (contract);
