-- OHLC Prices by Pool --
SELECT
      pool,
      timestamp,

      -- swaps --
      argMinMerge(open0) * pow(10, 12)       AS open0,
      max(high0) * pow(10, 12)               AS high0,
      min(low0) * pow(10, 12)                AS low0,
      argMaxMerge(close0) * pow(10, 12)      AS close0,
      sumMerge(volume0)                      AS volume0, -- volume is in wei, so we need to convert it to ether
      volume0 * 100 / pow(10, 6)             AS fee0, -- Uniswap V3 fee 0.01%

      -- swaps (inverse) --
      argMinMerge(open1) / pow(10, 12)       AS open1,
      max(high1) / pow(10, 12)               AS high1,
      min(low1) / pow(10, 12)                AS low1,
      argMaxMerge(close1) / pow(10, 12)      AS close1,
      sumMerge(volume1) * pow(10, 12)        AS volume1, -- volume is in wei, so we need to convert it to USDT precision 6
      volume1 * 100 / pow(10, 6)             AS fee1, -- Uniswap V3 fee 0.01%

      -- universal --
      uniqMerge(uaw) AS uaw,
      sumMerge(transactions) AS transactions
FROM ohlc_prices
WHERE pool = '0xc7bbec68d12a0d1830360f8ec58fa599ba1b0e9b' -- WETH/USDT
GROUP BY pool, timestamp
ORDER BY timestamp DESC\G;

-- Unique Active Wallets & Transactions by ERC-20 Contracts --
SELECT
      contract,
      timestamp,
      uniqMerge(uaw) AS uaw,
      sumMerge(transactions) AS transactions
FROM historical_erc20_balances_by_contract
GROUP by contract,timestamp;

-- Unique Active Wallets & Transactions by Native asset (ex: ETH/BNB) --
SELECT
      timestamp,
      uniqMerge(uaw) AS uaw,
      sumMerge(transactions) AS transactions
FROM historical_native_balances
GROUP by timestamp;
