/* Connect to SQLite database */

use sqlite;

/*
*** Connect to the database ***
* TODO: the database if not exists it creates one, put a default directory
        for the installation (if not it will create the database in the current directory)
* This function returns a connection to the database
*/
pub fn connect() -> sqlite::Connection {
    let conn = sqlite::open("ss.db").unwrap();
    create_table(&conn);
    conn
}

// Create table if not exists
fn create_table(conn: &sqlite::Connection) {
    conn.execute(
        "
        CREATE TABLE IF NOT EXISTS users (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            email TEXT NOT NULL,
            name TEXT NOT NULL,
            last_name TEXT NOT NULL,
            password TEXT NOT NULL
        )
        ",
    )
    .unwrap();

    conn.execute(
        "
        CREATE TABLE IF NOT EXISTS classrooms (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            number INTEGER NOT NULL,
            building TEXT NOT NULL,
            type TEXT NOT NULL,
            capacity INTEGER NOT NULL,
        )
        ",
    ).unwrap();

    conn.execute(
        "
        CREATE TABLE IF NOT EXISTS groups (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            grade INTEGER NOT NULL,
            group TEXT NOT NULL,
            specialty TEXT,
            shift TEXT NOT NULL,
            students_quantity INTEGER NOT NULL,
        )
        ",
    ).unwrap();
            
    conn.execute(
        "
        CREATE TABLE IF NOT EXISTS subjects (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT NOT NULL,
            abbreviation TEXT NOT NULL,
            type TEXT NOT NULL,
            color TEXT NOT NULL,
            teacher_id INTEGER NOT NULL,
        )
        ",
    ).unwrap();

    conn.execute(
        "
        CREATE TABLE IF NOT EXISTS teachers (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT NOT NULL,
            first_last_name TEXT NOT NULL,
            second_last_name TEXT NOT NULL,
            email TEXT NOT NULL,
            commissioned_hours INTEGER NOT NULL,
            active_hours INTEGER NOT NULL,
            general_stars INTEGER NOT NULL,
        )
        ",
    ).unwrap();

    conn.execute(
        "
        CREATE TABLE IF NOT EXISTS schedules (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            subject_id INTEGER NOT NULL,
            group_id INTEGER NOT NULL,
            classroom_id INTEGER NOT NULL,
            day TEXT NOT NULL,
            start_time TEXT NOT NULL,
            end_time TEXT NOT NULL,
        )
        ",
    ).unwrap();
}
