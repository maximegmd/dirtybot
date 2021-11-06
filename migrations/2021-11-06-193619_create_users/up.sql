CREATE TABLE users (
  id VARCHAR(20) PRIMARY KEY NOT NULL,     -- Discord ID (A Snowflake) has a length between 16 and 20 characters
  deposit_address VARCHAR(42) NOT NULL,    -- The deposit address has a maximum length of 42 characters (including the 0x prefix)
  amount VARCHAR(78) NOT NULL DEFAULT '0'  -- u256 (2^256-1) is a number (the maximum amount) with a length of 78 characters
)
