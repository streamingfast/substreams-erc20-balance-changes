-- Latest balances by owner
WITH latest_rows AS (
    SELECT
        owner,
        timestamp,
        contract,
        version,
        new_balance as balance,
        ROW_NUMBER() OVER (
            PARTITION BY owner, contract
            ORDER BY version DESC
        ) AS rn
    FROM read_parquet('./out/balance_changes/*.parquet')
    WHERE owner = LOWER('F977814e90dA44bFA03b6295A0616a897441aceC') -- Binance Hot Wallet
)
SELECT
    contract,
    balance,
    timestamp
FROM latest_rows
WHERE rn = 1 AND balance != '0'
ORDER BY timestamp DESC;

-- Count total balances by owner
SELECT
    count() as balance_changes,
    count(DISTINCT(contract)) as contracts,
FROM read_parquet('./out/balance_changes/*.parquet')
WHERE owner = LOWER('5754284f345afc66a98fbB0a0Afe71e0F007B949'); -- Tether: Treasury