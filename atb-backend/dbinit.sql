CREATE TABLE users (
    id UUID PRIMARY KEY -- minecraft uuid
    email STRING UNIQUE NOT NULL
);

-- a user can have multiple accounts
CREATE TABLE accounts (
    id UUID PRIMARY KEY
    userid UUID NOT NULL REFERENCES users (id)
    nickname STRING NOT NULL
);

CREATE TABLE balances (
    accountid REFERENCES accounts (id)
    item STRING NOT NULL -- minecraft:diamond
    amount INT -- 64 bit in cockroachdb 
);

CREATE TABLE transactions (
    time_millis TIMESTAMP PRIMARY KEY -- milliseconds since 1970
    from UUID NOT NULL REFERENCES accounts (id)
    to UUID NOT NULL REFERENCESS accounts (id)
    amount INT NOT NULL
    item STRING NOT NULL
    CONSTRAINT from_to_different CHECK (from != to)
);