-- Your SQL goes here
CREATE TABLE users (
  id SERIAL PRIMARY KEY,
  username TEXT NOT NULL,
  firstname TEXT NOT NULL,
  email TEXT NOT NULL,
  ip TEXT NOT NULL
)