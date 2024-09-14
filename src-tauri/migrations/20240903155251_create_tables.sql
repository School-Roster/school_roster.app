-- Add migration script here
CREATE TABLE IF NOT EXISTS subjects (
    id INTEGER PRIMARY KEY,
    name TEXT NOT NULL,
    shorten TEXT NOT NULL,
    color TEXT NOT NULL,
    spec TEXT NOT NULL
);

-- Teachers
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

-- Teachers Subjects
CREATE TABLE IF NOT EXISTS teachers_subjects (
    id INTEGER PRIMARY KEY,
    teacher_id INTEGER NOT NULL,
    subject_id INTEGER NOT NULL,
    FOREIGN KEY (teacher_id) REFERENCES teachers(id),
    FOREIGN KEY (subject_id) REFERENCES subjects(id)
);
