-- addresses --
INSERT INTO address_changed SELECT * FROM address_changed;
SELECT node, address FROM addresses;

-- names --
INSERT INTO name_registered SELECT * FROM name_registered;
SELECT node, name, expires FROM names WHERE expires > now();

-- text_changed --
INSERT INTO text_changed SELECT * FROM text_changed;
SELECT node, groupArrayMerge(kv_pairs_state) FROM agg_records GROUP BY node;
SELECT node, groupArray( (key, value) ) FROM records GROUP BY node;