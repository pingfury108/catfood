-- Your SQL goes here
CREATE TABLE cat_food (
  id SERIAL PRIMARY KEY,
  gid VARCHAR NOT NULL,
  title VARCHAR NOT NULL,
  describe TEXT NOT NULL
);
