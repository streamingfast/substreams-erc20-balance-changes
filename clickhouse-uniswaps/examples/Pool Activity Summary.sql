SELECT
    pool,
    token0,
    token1,
    transactions
FROM pool_activity_summary
WHERE token0 != ''
ORDER BY transactions DESC
LIMIT 10;