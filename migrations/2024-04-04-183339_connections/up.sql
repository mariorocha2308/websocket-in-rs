-- Your SQL goes here
CREATE TABLE connections (
  conn_id       TEXT NOT NULL PRIMARY KEY,
  nickname      TEXT NOT NULL,
  user_ref      UUID NOT NULL
)