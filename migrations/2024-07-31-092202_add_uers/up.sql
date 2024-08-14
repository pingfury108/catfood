-- Your SQL goes here
CREATE TABLE users (
  uid TEXT PRIMARY KEY,
  email TEXT NOT NULL,
  name TEXT NOT NULL,
  pwd TEXT NOT NULL,
  CONSTRAINT uname UNIQUE(email)
);
