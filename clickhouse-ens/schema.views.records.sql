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

CREATE MATERIALIZED VIEW IF NOT EXISTS records_mv
TO records AS
SELECT
    global_sequence,
    node,
    key,
    value
FROM text_changed
WHERE contract IN ('0x231b0ee14048e9dccd1d247744d114a4eb5e8e63', '0x4976fb03c32e5b8cfe2b6ccb31c09ba78ebaba41'); -- ENS: Public Resolver

CREATE TABLE IF NOT EXISTS agg_records (
    node                FixedString(66),
    kv_pairs_state      AggregateFunction(groupArray, Tuple(String, String))
)
ENGINE = AggregatingMergeTree
ORDER BY (node);

CREATE MATERIALIZED VIEW IF NOT EXISTS agg_records_mv
TO agg_records AS
SELECT
    node,
    groupArrayState( (key, value) )  AS kv_pairs_state
FROM records
GROUP BY node;

-- INSERT INTO text_changed SELECT * FROM text_changed;
-- SELECT node, groupArrayMerge(kv_pairs_state) FROM agg_records GROUP BY node;
-- SELECT node, groupArray( (key, value) ) FROM records GROUP BY node;