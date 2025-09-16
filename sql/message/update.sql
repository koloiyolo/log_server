UPDATE message
SET
    date = COALESCE(?, message.date),
    host = COALESCE(?, message.host),
    program = COALESCE(?, message.program),
    message = COALESCE(?, message.message)
WHERE
    rowid = ?;
