SELECT contract, count()
FROM read_parquet('./out/transfers/*.parquet') AS t
WHERE NOT EXISTS (
  SELECT 1
  FROM read_parquet('./out/balance_changes/*.parquet') AS b
  WHERE b.transaction_id = t.transaction_id
    AND b.log_index = t.log_index
) GROUP BY contract ORDER BY count() DESC LIMIT 30;
