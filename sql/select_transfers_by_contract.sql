-- Select the top sending addresses for DAI by total transferred value.
SELECT
    "from",
    count() as total_transfers,
    sum(value::HUGEINT / 10**18)::DECIMAL(18, 2) as total_value
FROM read_parquet('./out/transfers/*.parquet') AS t
WHERE contract = '6b175474e89094c44da98b954eedeac495271d0f' -- DAI
GROUP BY "from" ORDER BY total_value DESC LIMIT 30;

-- Select the top sending addresses for USDT, USDC, and DAI by total transferred value.
WITH tokens AS (
    SELECT lower('dac17f958d2ee523a2206206994597c13d831ec7') AS contract, 'USDT' AS symbol, 6  AS decimals
    UNION ALL
    SELECT lower('a0b86991c6218b36c1d19d4a2e9eb0ce3606eb48'), 'USDC', 6
    UNION ALL
    SELECT lower('6b175474e89094c44da98b954eedeac495271d0f'), 'DAI', 18
)
SELECT
    t."from",
    tokens.symbol,
    COUNT(*) AS total_transfers,
    SUM(value::HUGEINT / (10 ^ tokens.decimals))::DECIMAL(38, 2) AS total_value,
FROM read_parquet('./out/transfers/*.parquet') AS t
JOIN tokens
    ON lower(t.contract) = tokens.contract
WHERE t."from" != '0000000000000000000000000000000000000000'
GROUP BY t."from", tokens.symbol
ORDER BY total_value DESC
LIMIT 30;


-- Select the top receiver addresses for USDT, USDC, and DAI by total transferred value.
WITH tokens AS (
    SELECT lower('dac17f958d2ee523a2206206994597c13d831ec7') AS contract, 'USDT' AS symbol, 6  AS decimals
    UNION ALL
    SELECT lower('a0b86991c6218b36c1d19d4a2e9eb0ce3606eb48'), 'USDC', 6
    UNION ALL
    SELECT lower('6b175474e89094c44da98b954eedeac495271d0f'), 'DAI', 18
)
SELECT
    t."to",
    tokens.symbol,
    COUNT(*) AS total_transfers,
    SUM(value::HUGEINT / (10 ^ tokens.decimals))::DECIMAL(38, 2) AS total_value,
FROM read_parquet('./out/transfers/*.parquet') AS t
JOIN tokens
    ON lower(t.contract) = tokens.contract
WHERE t."to" != '0000000000000000000000000000000000000000'
GROUP BY t."to", tokens.symbol
ORDER BY total_value DESC
LIMIT 30;
