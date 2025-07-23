SELECT
    date,
    host,
    program,
    message
FROM
    message
WHERE
    message LIKE ?
ORDER BY rowid;
