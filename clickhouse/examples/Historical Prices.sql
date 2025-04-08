-- Historical Prices by Token0 --
SELECT
      pool,
      token0,
      token1
      timestamp,
      argMinMerge(open)  AS open,
      max(high)     AS high,
      min(low)      AS low,
      argMaxMerge(close) AS close,
      sumMerge(transactions) AS transactions,
      sumMerge(volume) AS volume
FROM ohlc_prices
WHERE
      pool = '0x504ab5b9f8c025505a3cc3c06d1cd7b22d32f093' -- WETH/UDOGE
GROUP BY pool,token0,token1,timestamp;

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
