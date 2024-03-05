INSERT INTO users (id)
VALUES ($1) ON CONFLICT (id) DO
UPDATE
SET id = users.id
RETURNING * -- for some reason, ON CONFLICT DO NOTHING RETURNING * doesn't work