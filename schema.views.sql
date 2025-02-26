-- latest balances --
CREATE MATERIALIZED VIEW balances
ENGINE = ReplacingMergeTree(version)
ORDER BY (owner, contract)
POPULATE
AS
SELECT
    owner,
    new_balance AS balance,
    contract,
    version
FROM balance_changes;