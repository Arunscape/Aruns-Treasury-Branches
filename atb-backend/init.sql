-- CREATE DATABASE IF NOT EXISTS atb;
-- CONNECT atb;
CREATE TABLE IF NOT EXISTS users (
    id UUID PRIMARY KEY,
    -- minecraft uuid
    email TEXT UNIQUE NOT NULL
);
-- a user can have multiple accounts
CREATE TABLE IF NOT EXISTS accounts (
    id UUID PRIMARY KEY,
    ownerid UUID NOT NULL,
    nickname TEXT NOT NULL,
    FOREIGN KEY (ownerid) REFERENCES users (id)
);
CREATE TABLE IF NOT EXISTS balances (
    accountid UUID,
    item TEXT NOT NULL,
    amount BIGINT,
    FOREIGN KEY (accountid) REFERENCES accounts (id)
);
CREATE TABLE IF NOT EXISTS transactions (
    id BIGSERIAL PRIMARY KEY,
    time_millis BIGINT,
    amount BIGINT NOT NULL,
    item TEXT NOT NULL,
    fromid UUID,
    toid UUID,
    FOREIGN KEY (fromid) REFERENCES accounts (id),
    FOREIGN KEY (toid) REFERENCES accounts (id),
    CONSTRAINT from_to_different CHECK (fromid != toid)
);