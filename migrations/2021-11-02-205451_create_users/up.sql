CREATE TABLE users (
  discord_id VARCHAR PRIMARY KEY,
  deposit_address VARCHAR NOT NULL,
  amount VARCHAR NOT NULL DEFAULT '0'
)