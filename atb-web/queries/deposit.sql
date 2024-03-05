INSERT INTO balances (accountid, item, quantity)
VALUES ($1, $2, $3) ON CONFLICT (accountid, item) DO
UPDATE
SET quantity = (balances.quantity + $3)
RETURNING *