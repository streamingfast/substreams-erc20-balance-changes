CREATE TABLE IF NOT EXISTS addresses (
    global_sequence         UInt64, -- latest global sequence (block_num << 32 + index)
    address                 FixedString(42),
    node                    FixedString(66),

    INDEX idx_node       (node)       TYPE bloom_filter GRANULARITY 4
)
ENGINE = ReplacingMergeTree(global_sequence)
ORDER BY (address, node);

CREATE MATERIALIZED VIEW IF NOT EXISTS mv_address_changed
TO addresses AS
SELECT
    global_sequence,
    node,
    address
FROM address_changed
WHERE contract IN (
    '0x231b0ee14048e9dccd1d247744d114a4eb5e8e63', -- ENS: Public Resolver
    '0x4976fb03c32e5b8cfe2b6ccb31c09ba78ebaba41'  -- ENS: Public Resolver 2
) AND address != '0x0000000000000000000000000000000000000000';
