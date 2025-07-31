SELECT
    rowid,
    username,
    email,
    '********' AS hash
FROM user
ORDER BY rowid;
