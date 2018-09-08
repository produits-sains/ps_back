-- Your SQL goes here

CREATE TABLE raw_products ( 
  id INTEGER PRIMARY KEY,
  vendor VARCHAR NOT NULL,
  vendor_id VARCHAR NOT NULL,
  name VARCHAR NOT NULL,
  url VARCHAR NOT NULL,
  ingr_txt VARCHAR NOT NULL,
  scraper_version VARCHAR NOT NULL,
  parsed BOOLEAN DEFAULT FALSE NOT NULL,
  UNIQUE( vendor, vendor_id) ON CONFLICT FAIL
);
