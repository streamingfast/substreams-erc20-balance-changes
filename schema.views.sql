-- latest balances by account --
CREATE MATERIALIZED VIEW balances
ENGINE = ReplacingMergeTree(version)
ORDER BY (owner, contract)
POPULATE
AS
SELECT * FROM balance_changes;

-- latest balances by account & by date --
CREATE MATERIALIZED VIEW balances_by_date
ENGINE = ReplacingMergeTree(version)
ORDER BY (owner, contract, date)
POPULATE
AS
SELECT * FROM balance_changes;

-- create a bloom-filter index for these high-cardinality string columns
CREATE INDEX idx_balance_changes_contract ON balance_changes (contract) TYPE bloom_filter GRANULARITY 4;
CREATE INDEX idx_balance_changes_owner    ON balance_changes (owner)   TYPE bloom_filter GRANULARITY 4;

-- create a bloom-filter index for these high-cardinality string columns
CREATE INDEX idx_transfers_contract ON transfers (contract) TYPE bloom_filter GRANULARITY 4;
CREATE INDEX idx_transfers_from     ON transfers (`from`)   TYPE bloom_filter GRANULARITY 4;
CREATE INDEX idx_transfers_to       ON transfers (`to`)     TYPE bloom_filter GRANULARITY 4;