CREATE TABLE IF NOT EXISTS names (
    node                    FixedString(66),
    name                    String,
    registered              SimpleAggregateFunction(min, DateTime(0, 'UTC')),
    expires                 SimpleAggregateFunction(max, DateTime(0, 'UTC')),

    INDEX idx_name       (name)       TYPE bloom_filter GRANULARITY 4
)
ENGINE = AggregatingMergeTree
ORDER BY (node, name);

CREATE MATERIALIZED VIEW IF NOT EXISTS mv_name_registered
TO names AS
SELECT
    node,
    name,
    min(timestamp) as registered,
    max(expires) as expires
FROM name_registered
WHERE contract IN (
    '0x57f1887a8bf19b14fc0df6fd9b2acc9af147ea85', -- ENS: Base Registrar Implementation
    '0x283af0b28c62c092c9727f1ee09c02ca627eb7f5', -- ENS: Old ETH Registrar Controller
    '0x253553366da8546fc250f225fe3d25d0c782303b', -- ENS: ETH Registrar Controller
) AND name != '' AND node != ''
GROUP BY node, name;
