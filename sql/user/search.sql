SELECT
    rowid,
    username,
    email,
    '' AS hash
FROM user
WHERE username LIKE ?
ORDER BY rowid;;
