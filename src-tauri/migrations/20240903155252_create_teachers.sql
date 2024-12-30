CREATE TABLE IF NOT EXISTS teachers (
    id INTEGER PRIMARY KEY,
    name TEXT NOT NULL,
    father_lastname TEXT NOT NULL,
    mother_lastname TEXT,
    email TEXT,
    phone TEXT,
    degree TEXT,
    commisioned_hours INTEGER,
    active_hours INTEGER,
    performance INTEGER
);
