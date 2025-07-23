SELECT
    rowid,
    username,
    email,
    hash
FROM user WHERE username LIKE ?;
