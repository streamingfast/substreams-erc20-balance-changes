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
