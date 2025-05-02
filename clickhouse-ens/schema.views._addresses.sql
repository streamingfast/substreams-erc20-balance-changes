CREATE TABLE IF NOT EXISTS addresses (
    global_sequence         UInt64, -- latest global sequence (block_num << 32 + index)
    address                 FixedString(42),
    node                    FixedString(66),

    INDEX idx_node          (node)       TYPE bloom_filter GRANULARITY 4
)
ENGINE = ReplacingMergeTree(global_sequence)
ORDER BY (address);

CREATE MATERIALIZED VIEW IF NOT EXISTS mv_reverse_claimed
TO addresses AS
SELECT
    global_sequence,
    node,
    address
FROM reverse_claimed
WHERE contract IN (
    '0xa58e81fe9b61b5c3fe2afd33cf304c454abfc7cb' -- ENS: Reverse Registrar
);
