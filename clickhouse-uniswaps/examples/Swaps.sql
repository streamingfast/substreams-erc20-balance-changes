-- Search Swaps by Pool --
WITH (
    pow(10, input_decimals) AS scale0,
    pow(10, output_decimals) AS scale1,
    pow(10, input_decimals - output_decimals) AS scale
)
SELECT
    input_token as input_token,
    input_amount / scale0 AS input_amount,
    input_decimals,
    output_token as output_token,
    output_decimals,
    output_amount / scale1 AS output_amount,
    price * scale AS price
FROM swaps
WHERE pool = '0x72331fcb696b0151904c03584b66dc8365bc63f8a144d89a773384e3a579ca73'
LIMIT 20;