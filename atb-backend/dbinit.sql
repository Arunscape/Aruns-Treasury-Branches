CREATE TABLE IF NOT EXISTS users (
    id string,
    email string
);


CREATE TABLE IF NOT EXISTS accounts (
    id string,
    ownerid string,
    nickname string
);

CREATE TABLE IF NOT EXISTS balances (
    accountid string,
    item string,
    amount long
);

CREATE TABLE IF NOT EXISTS transactions (
    id long,
    time_millis date, -- I don't think we'll have more than 1000 transactions per second
    item string,
    fromid string,
    toid string
);

-- -- For a time column expressed as the number of milliseconds since the UNIX epoch, set chunk_time_interval to 24 hour
-- -- SELECT set_chunk_time_interval('conditions', 86400000);
-- CREATE TABLE IF NOT EXISTS users (
--     id UUID PRIMARY KEY,
--     -- minecraft uuid
--     email TEXT UNIQUE NOT NULL
-- );
-- -- a user can have multiple accounts
-- CREATE TABLE IF NOT EXISTS accounts (
--     id UUID PRIMARY KEY,
--     ownerid UUID,
--     nickname TEXT NOT NULL,
--     FOREIGN KEY (ownerid) REFERENCES users (id)
-- );
-- CREATE TABLE IF NOT EXISTS balances (
--     accountid UUID,
--     item TEXT NOT NULL,
--     -- minecraft:diamond
--     amount BIGINT,
--     -- 64 bit int in postgresA,
--     FOREIGN KEY (accountid) REFERENCES accounts (id)
-- );
-- CREATE TABLE IF NOT EXISTS transactions (
--     id BIGINT PRIMARY KEY,
--     -- milliseconds since 1970
--     time_millis BIGINT,
--     item TEXT NOT NULL,
--     fromid UUID,
--     toid UUID,
--     FOREIGN KEY (fromid) REFERENCES accounts (id),
--     FOREIGN KEY (toid) REFERENCES accounts (id),
--     CONSTRAINT from_to_different CHECK (fromid != toid)
-- );
-- 