CREATE TABLE IF NOT EXISTS subjects (
    id INTEGER PRIMARY KEY,
    name TEXT NOT NULL,
    shorten TEXT NOT NULL,
    color TEXT,
    spec TEXT,
    required_modules INTEGER NOT NULL,
    priority INTEGER
);
