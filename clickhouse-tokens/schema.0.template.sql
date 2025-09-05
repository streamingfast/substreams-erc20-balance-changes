CREATE TABLE IF NOT EXISTS base_events (
    -- block --
    block_num            UInt32,
    block_hash           FixedString(66),
    timestamp            DateTime(0, 'UTC'),

    -- ordering --
    `index`              UInt64, -- relative index
    global_sequence      UInt64, -- latest global sequence (block_num << 32 + index)

    -- transaction --
    tx_hash              FixedString(66),

    -- indexes --
    INDEX idx_tx_hash            (tx_hash)            TYPE bloom_filter(0.005) GRANULARITY 1,

    -- indexes (event) --
    INDEX idx_from               (`from`)             TYPE bloom_filter(0.005) GRANULARITY 1,
    INDEX idx_to                 (`to`)               TYPE bloom_filter(0.005) GRANULARITY 1,
    INDEX idx_value              (value)              TYPE minmax GRANULARITY 1
)
ENGINE = ReplacingMergeTree
PARTITION BY toYYYYMM(timestamp)
ORDER BY (
    timestamp, block_num,
    block_hash, `index`
);


CREATE TABLE OR REPLACE log_events AS base_events
ALTER TABLE log_events
    -- call --
    ADD COLUMN IF NOT EXISTS caller                      FixedString(42),
    ADD COLUMN IF NOT EXISTS contract                    FixedString(42),
    -- log --
    ADD COLUMN IF NOT EXISTS ordinal                     UInt64,
    ADD COLUMN IF NOT EXISTS log_index                   UInt32,

    -- indexes --
    ADD INDEX IF NOT EXISTS idx_caller                 (caller)                 TYPE bloom_filter(0.005)     GRANULARITY 1,
    ADD INDEX IF NOT EXISTS idx_contract               (contract)               TYPE bloom_filter(0.005)     GRANULARITY 1,
    ADD INDEX IF NOT EXISTS idx_ordinal                (ordinal)                TYPE bloom_filter(0.005)     GRANULARITY 1,
    ADD INDEX IF NOT EXISTS idx_log_index              (log_index)              TYPE minmax                  GRANULARITY 1;
