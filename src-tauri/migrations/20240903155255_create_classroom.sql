CREATE TABLE IF NOT EXISTS classroom (
    id INTEGER PRIMARY KEY,
    building_id TEXT NOT NULL,
    building_number INTEGER NOT NULL,
    building_type TEXT,
    capacity INTEGER
);
