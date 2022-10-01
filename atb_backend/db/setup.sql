CREATE TABLE users (
    id UUID PRIMARY KEY -- minecraft uuid
    -- email STRING UNIQUE NOT NULL -- maybe will be added later
);
-- a user can have multiple accounts
CREATE TABLE accounts (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    userid UUID NOT NULL REFERENCES users (id),
    nickname TEXT NOT NULL
);
-- right now it only makes sense to support fungible items
-- e.g. diamond, or diamond block
-- no non-stackable items like swords or armour
CREATE TABLE mc_items (
    id TEXT PRIMARY KEY -- example: diamond
);
CREATE TABLE balances (
    accountid UUID REFERENCES accounts (id),
    item TEXT NOT NULL REFERENCES mc_items(id),
    amount BIGINT,
    PRIMARY KEY(accountid, item)
);
CREATE TABLE transactions (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    time_millis TIMESTAMP(6),
    -- the 6 means microsecond precision since 2000
    fromid UUID NOT NULL REFERENCES accounts (id),
    toid UUID NOT NULL REFERENCES accounts (id),
    amount BIGINT NOT NULL,
    item TEXT NOT NULL,
    CONSTRAINT from_to_different CHECK (fromid != toid)
);