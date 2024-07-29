-- Your SQL goes here
CREATE TABLE cat_food (
  gid TEXT PRIMARY KEY,
  title TEXT NOT NULL,
  describe TEXT NOT NULL,
  img TEXT,
  product_name TEXT,
  brand_owner TEXT,
  brand_owner_addr TEXT,
  manufacturer TEXT,
  manufacturer_addr TEXT,
  manufacturer_license_number TEXT,
  product_standard TEXT,
  net_content TEXT,
  reference_price TEXT,
  ingredients TEXT,
  production_method TEXT,
  additives TEXT,
  guaranteed_analysis TEXT
);
