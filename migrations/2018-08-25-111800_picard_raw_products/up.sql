-- Your SQL goes here

CREATE TABLE picard_raw_products ( 
  id INTEGER PRIMARY KEY,
  name VARCHAR NOT NULL,
  ingr_txt VARCHAR NOT NULL,
  scraper_version VARCHAR NOT NULL,
  parsed BOOLEAN DEFAULT FALSE NOT NULL
);
