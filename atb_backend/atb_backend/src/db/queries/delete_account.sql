DELETE FROM accounts
WHERE id = $1
    and userid = $2
RETURNING *