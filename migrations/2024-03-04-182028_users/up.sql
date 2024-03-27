-- Your SQL goes here
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TABLE users (
  _id           UUID DEFAULT UUID_GENERATE_V4()::UUID NOT NULL,
  nickname      TEXT NOT NULL,
  telephone     TEXT NOT NULL,
  keypass       TEXT NOT NULL,
  created_at    TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP NOT NULL,
                PRIMARY KEY(_id)
)