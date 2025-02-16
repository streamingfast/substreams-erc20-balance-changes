-- Select the top holders for DAI by balance.
WITH latest_balances AS (
    SELECT
        owner,
        version,
        (value::HUGEINT / 10^18)::DECIMAL(18, 2) AS balance,
        ROW_NUMBER() OVER (
            PARTITION BY owner
            ORDER BY version DESC
        ) AS rn
    FROM read_parquet('./out/balance_changes/*.parquet')
    WHERE contract = '6b175474e89094c44da98b954eedeac495271d0f'  -- DAI
)
SELECT
    owner,
    balance
FROM latest_balances
WHERE rn = 1
ORDER BY balance DESC
LIMIT 30;



SELECT
    owner,
    block_num,
    storage_ordinal,
    log_index,
    (value::HUGEINT / 10^18)::DECIMAL(18, 2) AS balance,
FROM read_parquet('./out/balance_changes/*.parquet')
WHERE block_num = 21529225 AND owner = lower('d03c7dFB28983067f5aCd43D0B983A287014eD2D') AND contract = '6b175474e89094c44da98b954eedeac495271d0f' AND transaction_id ='ad5b1e1310caadb93dd64b044171e5950a7eaa502f26183a89fc469d1ff8000b'
ORDER BY storage_ordinal DESC;