-- ENS Addresses/Names lookup --
SELECT address, name, expires FROM addresses
JOIN names FINAL USING (node)
WHERE expires > now();