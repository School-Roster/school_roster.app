CREATE TABLE IF NOT EXISTS groups (
    id INTEGER PRIMARY KEY,
    grade INTEGER NOT NULL,
    "group" TEXT NOT NULL,
    career TEXT,
    students INTEGER,
    max_modules_per_day INTEGER
);
