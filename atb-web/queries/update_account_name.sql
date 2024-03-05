UPDATE accounts
SET nickname = $1
WHERE nickname = $2
    AND userid = $3
RETURNING *