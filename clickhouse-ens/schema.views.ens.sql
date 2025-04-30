CREATE TABLE ens (
    global_sequence     UInt64,

    -- addresses --
    address             FixedString(42),
    node                FixedString(66),

    -- names --
    name                String,
    registered          DateTime('UTC'),
    expires             DateTime('UTC'),

    -- records --
    records_json        String          -- {"k1":"v1", ...}
)
ENGINE = ReplacingMergeTree(global_sequence)
ORDER BY (address, name);

CREATE MATERIALIZED VIEW mv_from_addresses
TO ens
AS
SELECT
    a.global_sequence AS global_sequence,
    a.address,
    a.node,

    n.name,
    n.registered,
    n.expires

    -- toJSONString(
    --     mapFromEntries( groupArrayMerge(r.kv_state) )
    -- ) AS records_json,

FROM addresses AS a
LEFT JOIN names AS n FINAL USING (node);
