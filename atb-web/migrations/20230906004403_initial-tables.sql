CREATE TABLE IF NOT EXISTS users (
    id UUID PRIMARY KEY -- minecraft uuid
    -- email STRING UNIQUE NOT NULL -- maybe will be added later
);
-- a user can have multiple accounts
CREATE TABLE IF NOT EXISTS accounts (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    userid UUID NOT NULL REFERENCES users (id),
    nickname TEXT NOT NULL,
    -- nickname is unique per user
    CONSTRAINT nickname_unique UNIQUE (userid, nickname)
);
   
    
-- right now it only makes sense to support fungible items
-- e.g. diamond, or diamond block
-- no non-stackable items like swords or armour
CREATE TABLE IF NOT EXISTS mc_items (
    id TEXT PRIMARY KEY -- example: diamond
);

CREATE TABLE IF NOT EXISTS balances (
    accountid UUID REFERENCES accounts (id),
    item TEXT NOT NULL REFERENCES mc_items(id),
    quantity BIGINT NOT NULL,
    PRIMARY KEY(accountid, item)
);

CREATE TABLE IF NOT EXISTS transactions (
    id BIGSERIAL PRIMARY KEY,
    fromid UUID NOT NULL REFERENCES accounts (id),
    toid UUID NOT NULL REFERENCES accounts (id),
    quantity BIGINT NOT NULL,
    item TEXT NOT NULL REFERENCES mc_items(id),
    price BIGINT NOT NULL,
    -- the 6 means microsecond precision since 2000
    time_processed TIMESTAMPTZ(6) NOT NULL DEFAULT NOW(),
    CONSTRAINT from_to_different CHECK (fromid != toid)
);
