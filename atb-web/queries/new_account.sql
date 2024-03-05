INSERT INTO accounts (userid, nickname)
VALUES ($1, $2)
RETURNING *