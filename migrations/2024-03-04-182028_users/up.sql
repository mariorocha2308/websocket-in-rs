-- Your SQL goes here
CREATE TABLE users (
  id TEXT PRIMARY KEY,
  username TEXT NOT NULL,
  phone TEXT NOT NULL,
  password TEXT NOT NULL,
  created_at TIMESTAMP NOT NULL DEFAULT NOW()
)