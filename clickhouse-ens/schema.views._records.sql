CREATE TABLE IF NOT EXISTS records (
    global_sequence      UInt64, -- latest global sequence (block_num << 32 + index)
    node                 FixedString(66),
    key                  LowCardinality(String),
    value                String,

   -- indexes --
   INDEX idx_key                  (key)               TYPE bloom_filter GRANULARITY 4,
   INDEX idx_value                (value)             TYPE bloom_filter GRANULARITY 4
)
ENGINE = ReplacingMergeTree(global_sequence)
ORDER BY (node, key);

CREATE MATERIALIZED VIEW IF NOT EXISTS mv_text_changed
TO records AS
SELECT
    global_sequence,
    node,
    key,
    value
FROM text_changed
WHERE contract IN (
    '0x231b0ee14048e9dccd1d247744d114a4eb5e8e63', -- ENS: Public Resolver
    '0x4976fb03c32e5b8cfe2b6ccb31c09ba78ebaba41'  -- ENS: Public Resolver 2
);
