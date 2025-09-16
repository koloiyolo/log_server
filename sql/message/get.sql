SELECT
    date,
    host,
    program,
    message
FROM message
WHERE rowid = ?;
