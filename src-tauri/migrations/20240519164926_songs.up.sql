CREATE TABLE IF NOT EXISTS songs (
        id INTEGER PRIMARY KEY AUTOINCREMENT
    ,   title TEXT NOT NULL
    ,   path TEXT NOT NULL UNIQUE
    ,   file_hash TEXT NOT NULL UNIQUE
);