-- Your SQL goes here

CREATE TABLE find_product_rules ( 
  id INTEGER PRIMARY KEY AUTOINCREMENT,
  vendor VARCHAR NOT NULL,
  xpath VARCHAR NOT NULL,
  regexp_rule VARCHAR NOT NULL,
  regexp_flags VARCHAR NOT NULL
);
