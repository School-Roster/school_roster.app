CREATE TABLE IF NOT EXISTS school (
    id INTEGER PRIMARY KEY,
    name TEXT NOT NULL,
    logo_path TEXT
);

-- Insert default empty school
INSERT INTO school (id, name) VALUES (1, '');

-- Down
-- DROP TABLE school;
