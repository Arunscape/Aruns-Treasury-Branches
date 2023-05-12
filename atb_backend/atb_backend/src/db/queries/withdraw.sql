UPDATE balances
SET quantity = (quantity - $3)
WHERE accountid = $1
    AND item = $2
RETURNING *