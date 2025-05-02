-- This file is generated. Do not edit.

CREATE TABLE IF NOT EXISTS cursors
(
    id        String,
    cursor    String,
    block_num Int64,
    block_id  String
)
    ENGINE = ReplacingMergeTree()
        PRIMARY KEY (id)
        ORDER BY (id);

CREATE TABLE IF NOT EXISTS name_registered (
    -- block --
    block_num            UInt32,
    block_hash           FixedString(66),
    timestamp            DateTime(0, 'UTC'),

    -- ordering --
    ordinal              UInt64, -- log.ordinal
    `index`              UInt64, -- relative index
    global_sequence      UInt64, -- latest global sequence (block_num << 32 + index)

    -- transaction --
    tx_hash              FixedString(66),

    -- call --
    caller               FixedString(42) COMMENT 'caller address', -- call.caller

    -- log --
    contract             FixedString(42) COMMENT 'contract address',

    -- event (v1 & v0) --
    name                 String,
    label                FixedString(66),
    node                 FixedString(66),
    owner                FixedString(42),
    base_cost            UInt64,
    expires              DateTime(0, 'UTC'),

    -- event (v1) --
    premium              UInt64,

    -- event (base) --
    token_id             UInt256
)
ENGINE = ReplacingMergeTree
PRIMARY KEY (timestamp, block_num, `index`)
ORDER BY (timestamp, block_num, `index`);

CREATE TABLE IF NOT EXISTS name_renewed (
    -- block --
    block_num            UInt32,
    block_hash           FixedString(66),
    timestamp            DateTime(0, 'UTC'),

    -- ordering --
    ordinal              UInt64, -- log.ordinal
    `index`              UInt64, -- relative index
    global_sequence      UInt64, -- latest global sequence (block_num << 32 + index)

    -- transaction --
    tx_hash              FixedString(66),

    -- call --
    caller               FixedString(42) COMMENT 'caller address', -- call.caller

    -- log --
    contract             FixedString(42) COMMENT 'contract address',

    -- event --
    name                 String,
    label                FixedString(66),
    node                 FixedString(66),
    cost                 UInt64,
    expires              DateTime(0, 'UTC')
)
ENGINE = ReplacingMergeTree
PRIMARY KEY (timestamp, block_num, `index`)
ORDER BY (timestamp, block_num, `index`);

CREATE TABLE IF NOT EXISTS text_changed (
    -- block --
    block_num            UInt32,
    block_hash           FixedString(66),
    timestamp            DateTime(0, 'UTC'),

    -- ordering --
    ordinal              UInt64, -- log.ordinal
    `index`              UInt64, -- relative index
    global_sequence      UInt64, -- latest global sequence (block_num << 32 + index)

    -- transaction --
    tx_hash              FixedString(66),

    -- call --
    caller               FixedString(42) COMMENT 'caller address', -- call.caller

    -- log --
    contract             FixedString(42) COMMENT 'contract address',

    -- event --
    node                 FixedString(66),
    key                  String,
    value                String,
    indexed_key          FixedString(66)
)
ENGINE = ReplacingMergeTree
PRIMARY KEY (timestamp, block_num, `index`)
ORDER BY (timestamp, block_num, `index`);

CREATE TABLE IF NOT EXISTS reverse_claimed (
    -- block --
    block_num            UInt32,
    block_hash           FixedString(66),
    timestamp            DateTime(0, 'UTC'),

    -- ordering --
    ordinal              UInt64, -- log.ordinal
    `index`              UInt64, -- relative index
    global_sequence      UInt64, -- latest global sequence (block_num << 32 + index)

    -- transaction --
    tx_hash              FixedString(66),

    -- call --
    caller               FixedString(42) COMMENT 'caller address', -- call.caller

    -- log --
    contract             FixedString(42) COMMENT 'contract address',

    -- event --
    node                 FixedString(66),
    address              FixedString(42),
)
ENGINE = ReplacingMergeTree
PRIMARY KEY (timestamp, block_num, `index`)
ORDER BY (timestamp, block_num, `index`);

CREATE TABLE IF NOT EXISTS name_changed (
    -- block --
    block_num            UInt32,
    block_hash           FixedString(66),
    timestamp            DateTime(0, 'UTC'),

    -- ordering --
    ordinal              UInt64, -- log.ordinal
    `index`              UInt64, -- relative index
    global_sequence      UInt64, -- latest global sequence (block_num << 32 + index)

    -- transaction --
    tx_hash              FixedString(66),

    -- call --
    caller               FixedString(42) COMMENT 'caller address', -- call.caller

    -- log --
    contract             FixedString(42) COMMENT 'contract address',

    -- event --
    node                 FixedString(66),
    name                 String
)
ENGINE = ReplacingMergeTree
PRIMARY KEY (timestamp, block_num, `index`)
ORDER BY (timestamp, block_num, `index`);

CREATE TABLE IF NOT EXISTS address_changed (
    -- block --
    block_num            UInt32,
    block_hash           FixedString(66),
    timestamp            DateTime(0, 'UTC'),

    -- ordering --
    ordinal              UInt64, -- log.ordinal
    `index`              UInt64, -- relative index
    global_sequence      UInt64, -- latest global sequence (block_num << 32 + index)

    -- transaction --
    tx_hash              FixedString(66),

    -- call --
    caller               FixedString(42) COMMENT 'caller address', -- call.caller

    -- log --
    contract             FixedString(42) COMMENT 'contract address',

    -- event --
    node                 String,
    address              FixedString(42),
    coin_type            UInt64 COMMENT 'coin type (e.g. 60 for ETH)'
)
ENGINE = ReplacingMergeTree
PRIMARY KEY (timestamp, block_num, `index`)
ORDER BY (timestamp, block_num, `index`);


CREATE TABLE IF NOT EXISTS content_hash_changed (
    -- block --
    block_num            UInt32,
    block_hash           FixedString(66),
    timestamp            DateTime(0, 'UTC'),

    -- ordering --
    ordinal              UInt64, -- log.ordinal
    `index`              UInt64, -- relative index
    global_sequence      UInt64, -- latest global sequence (block_num << 32 + index)

    -- transaction --
    tx_hash              FixedString(66),

    -- call --
    caller               FixedString(42) COMMENT 'caller address', -- call.caller

    -- log --
    contract             FixedString(42) COMMENT 'contract address',

    -- event --
    node                 FixedString(66),
    hash                 String
)
ENGINE = ReplacingMergeTree
PRIMARY KEY (timestamp, block_num, `index`)
ORDER BY (timestamp, block_num, `index`);

CREATE TABLE IF NOT EXISTS new_owner (
    -- block --
    block_num            UInt32,
    block_hash           FixedString(66),
    timestamp            DateTime(0, 'UTC'),

    -- ordering --
    ordinal              UInt64, -- log.ordinal
    `index`              UInt64, -- relative index
    global_sequence      UInt64, -- latest global sequence (block_num << 32 + index)

    -- transaction --
    tx_hash              FixedString(66),

    -- call --
    caller               FixedString(42) COMMENT 'caller address', -- call.caller

    -- log --
    contract             FixedString(42) COMMENT 'contract address',

    -- event --
    node                 FixedString(66),
    label                FixedString(66),
    owner                FixedString(42)
)
ENGINE = ReplacingMergeTree
PRIMARY KEY (timestamp, block_num, `index`)
ORDER BY (timestamp, block_num, `index`);


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


CREATE TABLE IF NOT EXISTS expires (
    global_sequence         SimpleAggregateFunction(max, UInt64), -- latest global sequence (block_num << 32 + index)
    node                    FixedString(66),
    name                    String,
    registered              SimpleAggregateFunction(min, DateTime(0, 'UTC')),
    expires                 SimpleAggregateFunction(max, DateTime(0, 'UTC')),

    INDEX idx_name          (name)          TYPE bloom_filter GRANULARITY 4,
    INDEX idx_registered    (registered)    TYPE minmax       GRANULARITY 4,
    INDEX idx_expires       (expires)       TYPE minmax       GRANULARITY 4
)
ENGINE = AggregatingMergeTree
ORDER BY (node);

CREATE MATERIALIZED VIEW IF NOT EXISTS mv_name_registered
TO expires AS
SELECT
    max(global_sequence) as global_sequence,
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

CREATE MATERIALIZED VIEW IF NOT EXISTS mv_name_renewed
TO expires AS
SELECT
    max(global_sequence) as global_sequence,
    node,
    name,
    min(timestamp) as registered,
    max(expires) as expires
FROM name_renewed
WHERE contract IN (
    '0x57f1887a8bf19b14fc0df6fd9b2acc9af147ea85', -- ENS: Base Registrar Implementation
    '0x283af0b28c62c092c9727f1ee09c02ca627eb7f5', -- ENS: Old ETH Registrar Controller
    '0x253553366da8546fc250f225fe3d25d0c782303b', -- ENS: ETH Registrar Controller
) AND name != '' AND node != ''
GROUP BY node, name;


CREATE TABLE IF NOT EXISTS names (
    global_sequence         SimpleAggregateFunction(max, UInt64), -- latest global sequence (block_num << 32 + index)
    node                    FixedString(66),
    name                    String,

    INDEX idx_name          (name)       TYPE bloom_filter GRANULARITY 4
)
ENGINE = ReplacingMergeTree(global_sequence)
ORDER BY (node);

CREATE MATERIALIZED VIEW IF NOT EXISTS mv_name_changed
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

CREATE TABLE IF NOT EXISTS ens (
    -- ordering --
    node                FixedString(66),
    global_sequence     UInt64,

    -- addresses --
    address             FixedString(42),

    -- names --
    name                String,
    registered          DateTime('UTC'),
    expires             DateTime('UTC'),

    -- records --
    records             Array(Tuple(String, String)),
)
ENGINE = ReplacingMergeTree(global_sequence)
ORDER BY (address, name);

-- FROM addresses --
CREATE MATERIALIZED VIEW IF NOT EXISTS mv_ens_from_addresses
REFRESH EVERY 10 SECOND
TO ens
AS
SELECT
    -- ordering --
    a.node as node,
    max(a.global_sequence) AS global_sequence,

    -- addresses --
    any(a.address) as address,

    -- names --
    any(n.name) as name,
    min(e.registered) as registered,
    max(e.expires) as expires,

    -- records --
    groupArrayMerge(r.kv_state) AS records

FROM addresses AS a
LEFT JOIN names AS n USING (node)
LEFT JOIN expires AS e USING (node)
LEFT JOIN records_by_node AS r USING (node)
GROUP BY
    a.node;


