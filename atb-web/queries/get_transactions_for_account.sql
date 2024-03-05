SELECT *
FROM transactions
WHERE fromid = $1
    OR toid = $1