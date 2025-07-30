UPDATE user
SET
    username = COALESCE(?, user.username),
    email = COALESCE(?, user.email),
    hash = COALESCE(?, user.hash)
WHERE
    rowid = ?;
