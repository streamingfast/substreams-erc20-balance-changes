CREATE TABLE IF NOT EXISTS records (
    global_sequence      UInt64, -- latest global sequence (block_num << 32 + index)
    node                 FixedString(66),
    key                  LowCardinality(String),
    value                String,

   -- indexes --
   INDEX idx_node                 (node)              TYPE bloom_filter GRANULARITY 4,
   INDEX idx_value                (value)             TYPE bloom_filter GRANULARITY 4
)
ENGINE = ReplacingMergeTree(global_sequence)
ORDER BY (key, node);

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

CREATE TABLE IF NOT EXISTS records_by_node (
    global_sequence         UInt64, -- latest global sequence (block_num << 32 + index)
    node                    FixedString(66),
    kv_state                AggregateFunction(groupArray, Tuple(String, String))
) ENGINE = AggregatingMergeTree
ORDER BY node;

CREATE MATERIALIZED VIEW IF NOT EXISTS records_by_node_mv
TO records_by_node AS
SELECT
    max(global_sequence) AS global_sequence,
    node,
    groupArrayState((key, value)) AS kv_state
FROM records
GROUP BY node;