CREATE TABLE IF NOT EXISTS assignments (
    id INTEGER PRIMARY KEY,
    group_id INTEGER NOT NULL,
    day TEXT NOT NULL,
    module_index INTEGER NOT NULL,
    subject_id INTEGER NOT NULL,
    teacher_id INTEGER NOT NULL,
    FOREIGN KEY (group_id) REFERENCES groups(id) ON DELETE CASCADE,
    FOREIGN KEY (subject_id) REFERENCES subjects(id) ON DELETE CASCADE,
    FOREIGN KEY (teacher_id) REFERENCES teachers(id) ON DELETE CASCADE,
    UNIQUE (group_id, day, module_index)
);
