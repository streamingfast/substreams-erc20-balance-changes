-- Offchain Metadata
CREATE TABLE IF NOT EXISTS nft_metadata (
    contract            FixedString(42),
    token_id            UInt256,
    type                LowCardinality(String),
    metadata_json       String,
    name                String,
    description         String,
    attributes          String,
    media_uri           String,
    created_at          DateTime(0, 'UTC') DEFAULT now(),
    INDEX idx_type (type) TYPE set(8) GRANULARITY 4,
    INDEX idx_name (name) TYPE bloom_filter GRANULARITY 4,
) ENGINE = ReplacingMergeTree(created_at)
ORDER BY (contract, token_id);

-- Metadata requests and query results
CREATE TABLE IF NOT EXISTS scrape_attempts (
    contract            FixedString(42),
    token_id            UInt256,
    uri                 String,
    attempt_num         UInt32,
    timestamp           DateTime(0, 'UTC') DEFAULT now(),
    result              LowCardinality(String),
    reason              LowCardinality(String),
    error_msg           String,
    duration            UInt32,
    INDEX idx_attempt_num (attempt_num) TYPE minmax GRANULARITY 4,
    INDEX idx_timestamp (timestamp) TYPE minmax GRANULARITY 4,
    INDEX idx_result (result) TYPE set(8) GRANULARITY 4,
    INDEX idx_reason (reason) TYPE set(32) GRANULARITY 4,
    INDEX idx_duration (duration) TYPE minmax GRANULARITY 4,
) ENGINE = MergeTree()
ORDER BY (contract, token_id, attempt_num);

-- Spam Scoring
CREATE TABLE IF NOT EXISTS spam_scoring (
    contract        FixedString(42),
    timestamp       DateTime(0, 'UTC') DEFAULT now(),  -- when the score was assigned
    spam_score      UInt16,                                 -- score from 0 (not spam) to 100 (definitely spam)
    classification  LowCardinality(String) DEFAULT '',      -- ? e.g., 'spam', 'not_spam', 'suspicious'
    reason          String DEFAULT '',                      -- ? optional: explanation or model output
    source          LowCardinality(String) DEFAULT '',      -- ? which service/model provided this score

    INDEX idx_timestamp      (timestamp)      TYPE minmax GRANULARITY 4,
    INDEX idx_spam_score     (spam_score)     TYPE minmax GRANULARITY 4,
    INDEX idx_classification (classification) TYPE set(8)  GRANULARITY 4,
    INDEX idx_source         (source)         TYPE set(8)  GRANULARITY 4,
    INDEX idx_reason         (reason)         TYPE bloom_filter GRANULARITY 4
) ENGINE = ReplacingMergeTree(timestamp)
PRIMARY KEY (contract)
ORDER BY (contract);
