-- For a time column expressed as the number of milliseconds since the UNIX epoch, set chunk_time_interval to 24 hour
-- SELECT set_chunk_time_interval('conditions', 86400000);
CREATE TABLE users (
    id UUID PRIMARY KEY,
    -- minecraft uuid
    email TEXT UNIQUE NOT NULL
);
-- a user can have multiple accounts
CREATE TABLE accounts (
    id UUID PRIMARY KEY,
    ownerid UUID,
    nickname TEXT NOT NULL,
    FOREIGN KEY (ownerid) REFERENCES users (id)
);
CREATE TABLE balances (
    accountid UUID,
    item TEXT NOT NULL,
    -- minecraft:diamond
    amount BIGINT,
    -- 64 bit int in postgresA,
    FOREIGN KEY (accountid) REFERENCES accounts (id)
);
CREATE TABLE transactions (
    id BIGSERIAL PRIMARY KEY,
    -- milliseconds since 1970
    time_millis BIGINT UNIQUE,
    amount BIGINT NOT NULL,
    item TEXT NOT NULL,
    fromid UUID,
    toid UUID,
    FOREIGN KEY (fromid) REFERENCES accounts (id),
    FOREIGN KEY (toid) REFERENCES accounts (id),
    CONSTRAINT from_to_different CHECK (fromid != toid)
);
SELECT create_hypertable('transactions', time_millis);
CREATE INDEX ON transactions (item, time_millis desc);