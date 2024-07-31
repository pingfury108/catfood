-- Your SQL goes here
CREATE TABLE users (
  uid TEXT PRIMARY KEY,
  name TEXT NOT NULL,
  pwd TEXT NOT NULL,
  display_name TEXT NOT NULL,
  email TEXT
);
