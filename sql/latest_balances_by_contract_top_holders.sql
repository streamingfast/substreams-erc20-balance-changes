-- Latest balances by contract
-- Top Holders of USDT
WITH latest_rows AS (
    SELECT
        date,
        timestamp,
        owner,
        (new_balance::HUGEINT / 10^6)::DECIMAL(18, 2) AS balance, -- USDT has 6 decimals
        ROW_NUMBER() OVER (
            PARTITION BY owner, contract
            ORDER BY version DESC
        ) AS rn
    FROM read_parquet('./out/balance_changes/*.parquet')
    WHERE contract = LOWER('dac17f958d2ee523a2206206994597c13d831ec7') -- USDT
)
SELECT
    date,
    owner,
    balance
FROM latest_rows
WHERE rn = 1 AND balance > 1.0 -- Exclude below 1.0 USDT balances
ORDER BY balance DESC
LIMIT 30;
