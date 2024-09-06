CREATE TABLE IF NOT EXISTS groups_subjects (
    group_id INTEGER NOT NULL,
    subject_id INTEGER NOT NULL,
    PRIMARY KEY (group_id, subject_id),
    FOREIGN KEY (group_id) REFERENCES groups(id) ON DELETE CASCADE,
    FOREIGN KEY (subject_id) REFERENCES subjects(id)
);

