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

CREATE TABLE IF NOT EXISTS scrape_attempts_by_contract (
    contract                 FixedString(42),
    success_count            UInt32,
    error_count              UInt32,
    error_parse_count        UInt32,
    error_uri_count          UInt32,
    error_timeout_count      UInt32,
    error_http_count         UInt32,
    error_empty_count        UInt32,
    error_host_count         UInt32,
    error_other_count        UInt32,
    last_success_timestamp   DateTime,
    last_error_timestamp     DateTime,
    last_timestamp           DateTime,
    INDEX idx_last_timestamp (last_timestamp) TYPE minmax GRANULARITY 4,
    INDEX idx_error_count (error_count) TYPE minmax GRANULARITY 4,
    INDEX idx_success_count (success_count) TYPE minmax GRANULARITY 4
) ENGINE = SummingMergeTree()
ORDER BY contract;

CREATE MATERIALIZED VIEW IF NOT EXISTS scrape_attempts_by_contract_mv
    TO scrape_attempts_by_contract
    AS
    SELECT
        contract,
        sum(if(result = 'success', 1, 0)) AS success_count,
        sum(if(result = 'error', 1, 0)) AS error_count,
        sum(if(result = 'error' AND reason = 'parse', 1, 0)) AS error_parse_count,
        sum(if(result = 'error' AND reason = 'uri', 1, 0)) AS error_uri_count,
        sum(if(result = 'error' AND reason = 'timeout', 1, 0)) AS error_timeout_count,
        sum(if(result = 'error' AND reason = 'http', 1, 0)) AS error_http_count,
        sum(if(result = 'error' AND reason = 'empty', 1, 0)) AS error_empty_count,
        sum(if(result = 'error' AND reason = 'host', 1, 0)) AS error_host_count,
        sum(if(result = 'error' AND (reason = '' OR reason NOT IN ('parse', 'uri', 'timeout', 'http', 'empty', 'host')), 1, 0)) AS error_other_count,
        max(if(result = 'success', timestamp, toDateTime('1970-01-01 00:00:00'))) AS last_success_timestamp,
        max(if(result = 'error', timestamp, toDateTime('1970-01-01 00:00:00'))) AS last_error_timestamp,
        max(timestamp) AS last_timestamp
    FROM scrape_attempts
GROUP BY contract;

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
