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
    records_kv          Array(Tuple(String, String)),
)
ENGINE = ReplacingMergeTree(global_sequence)
ORDER BY (address, name);

-- FROM addresses --
CREATE MATERIALIZED VIEW IF NOT EXISTS mv_from_addresses
TO ens
AS
SELECT
    -- ordering --
    a.node as node,
    max(a.global_sequence) AS global_sequence,

    -- addresses --
    a.address as address,

    -- names --
    any(n.name) as name,
    min(n.registered) as registered,
    max(n.expires) as expires,

    -- records --
    groupArrayMerge(r.kv_state) AS records_kv

FROM addresses AS a
LEFT JOIN names AS n USING (node)
LEFT JOIN agg_records AS r USING (node)
GROUP BY
    a.address,
    a.node;

-- FROM names --
CREATE MATERIALIZED VIEW IF NOT EXISTS mv_from_names
TO ens
AS
SELECT
    -- ordering --
    n.node as node,
    max(n.global_sequence) AS global_sequence,

    -- addresses --
    any(a.address)    AS address,   -- a node can map to many

    -- names --
    any(n.name) as name,
    min(n.registered) as registered,
    max(n.expires) as expires,

    -- records --
    groupArrayMerge(r.kv_state) AS records_kv

FROM names AS n
LEFT JOIN addresses AS a USING (node)         -- gives you ≥1 address per node
LEFT JOIN agg_records AS r USING (node)
GROUP BY n.node;

-- FROM records --
CREATE MATERIALIZED VIEW IF NOT EXISTS mv_from_agg_records
TO ens
AS
SELECT
    -- ordering --
    r.node as node,
    max(r.global_sequence) AS global_sequence,

    -- addresses --
    any(a.address) as address,

    -- names --
    any(n.name) as name,
    min(n.registered) as registered,
    max(n.expires) as expires,

    -- records --
    groupArrayMerge(r.kv_state) AS records_kv

FROM agg_records AS r
LEFT JOIN addresses AS a USING (node)
LEFT JOIN names AS n USING (node)
GROUP BY r.node;
