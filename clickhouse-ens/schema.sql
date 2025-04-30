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

    -- event --
    name                 String,
    label                FixedString(66),
    node                 FixedString(66),
    owner                FixedString(42),
    base_cost            UInt64,
    expires              DateTime(0, 'UTC'),
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
    value                String
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
    node                 FixedString(66),
    address              FixedString(42)
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


CREATE TABLE IF NOT EXISTS addresses (
    global_sequence         UInt64, -- latest global sequence (block_num << 32 + index)
    address                 FixedString(42),
    node                    FixedString(66),

    INDEX idx_node       (node)       TYPE bloom_filter GRANULARITY 4
)
ENGINE = ReplacingMergeTree(global_sequence)
ORDER BY (address, node);

CREATE MATERIALIZED VIEW IF NOT EXISTS address_changed_mv
TO addresses AS
SELECT
    global_sequence,
    node,
    address
FROM address_changed
WHERE contract IN ('0x231b0ee14048e9dccd1d247744d114a4eb5e8e63', '0x4976fb03c32e5b8cfe2b6ccb31c09ba78ebaba41'); -- ENS: Public Resolver


CREATE TABLE IF NOT EXISTS names (
    node                    FixedString(66),
    name                    String,
    registered              SimpleAggregateFunction(min, DateTime(0, 'UTC')),
    expires                 SimpleAggregateFunction(max, DateTime(0, 'UTC')),

    INDEX idx_name       (name)       TYPE bloom_filter GRANULARITY 4
)
ENGINE = AggregatingMergeTree
ORDER BY (node, name);

CREATE MATERIALIZED VIEW IF NOT EXISTS name_registered_mv
TO names AS
SELECT
    node,
    name,
    min(timestamp) as registered,
    max(expires) as expires
FROM name_registered
WHERE contract IN ('0x253553366da8546fc250f225fe3d25d0c782303b') -- ENS: ETH Registrar Controller
GROUP BY node, name;


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


