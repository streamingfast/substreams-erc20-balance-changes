CREATE TABLE IF NOT EXISTS names (
    global_sequence         SimpleAggregateFunction(max, UInt64), -- latest global sequence (block_num << 32 + index)
    node                    FixedString(66),
    name                    String,

    INDEX idx_name          (name)       TYPE bloom_filter GRANULARITY 4
)
ENGINE = ReplacingMergeTree(global_sequence)
ORDER BY (node);

CREATE MATERIALIZED VIEW IF NOT EXISTS mv_names_by_name_changed
TO names AS
SELECT
    global_sequence,
    node,
    name
FROM name_changed
WHERE contract IN (
    '0x231b0ee14048e9dccd1d247744d114a4eb5e8e63', -- ENS: Public Resolver
    '0x4976fb03c32e5b8cfe2b6ccb31c09ba78ebaba41'  -- ENS: Public Resolver 2
);

CREATE MATERIALIZED VIEW IF NOT EXISTS mv_names_by_name_registered
TO names AS
SELECT
    global_sequence,
    node,
    name,
FROM name_registered
WHERE contract IN (
    '0x57f1887a8bf19b14fc0df6fd9b2acc9af147ea85', -- ENS: Base Registrar Implementation
    '0x283af0b28c62c092c9727f1ee09c02ca627eb7f5', -- ENS: Old ETH Registrar Controller
    '0x253553366da8546fc250f225fe3d25d0c782303b', -- ENS: ETH Registrar Controller
);

CREATE MATERIALIZED VIEW IF NOT EXISTS mv_names_by_name_renewed
TO names AS
SELECT
    global_sequence,
    node,
    name,
FROM name_renewed
WHERE contract IN (
    '0x57f1887a8bf19b14fc0df6fd9b2acc9af147ea85', -- ENS: Base Registrar Implementation
    '0x283af0b28c62c092c9727f1ee09c02ca627eb7f5', -- ENS: Old ETH Registrar Controller
    '0x253553366da8546fc250f225fe3d25d0c782303b', -- ENS: ETH Registrar Controller
);