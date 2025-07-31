SELECT
    rowid,
    username,
    email,
    '********' AS hash
FROM user WHERE rowid LIKE ?;
