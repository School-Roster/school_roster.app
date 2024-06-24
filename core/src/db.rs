/* Connect to SQLite database */

use sqlite::{Connection, Error};
use std::path::PathBuf;
use dirs::data_dir;

/*
*** Connect to the database ***
* Return: Connection or Error
*/
pub fn connect() -> Result<Connection, Error> {
    let path = default_db_path().unwrap();
    let conn = sqlite::open(&path)?;
    // create_table(&conn, "users")?;
    // create_table(&conn, "classrooms")?;
    create_table(&conn, "groups")?;
    create_table(&conn, "subjects")?;
    create_table(&conn, "teachers")?;
    Ok(conn)
}

// Create tables
// It checks if the table exists, if not, it creates it
fn create_table(conn: &Connection, tname: &str) -> Result<(), Error> {
    if !table_exists(conn, tname)? {
        match tname {
            "users" => users_table(conn)?,
            "classrooms" => classrooms_table(conn)?,
            "groups" => groups_table(conn)?,
            "subjects" => subjects_table(conn)?,
            "teachers" => teachers_table(conn)?,
            _ => println!("Table not found"),
        }
    }

    Ok(())
}

/*
*** Check if table exists ***
* Why?
- Minimal Query Overhead: The table_exists function runs a simple query against
    the sqlite_master table, which is a lightweight operation.
- Avoid unnecessary table creation: If the table already exists, we can avoid
    executing the CREATE TABLE statement, which can be a costly operation.

* Return: bool or Error if cannot execute the query
*/
fn table_exists(conn: &Connection, table_name: &str) -> Result<bool, Error> {
    let mut result = false;
    let mut statement = conn.prepare(&format!(
        "SELECT name FROM sqlite_master WHERE type='table' AND name='{}'",
        table_name
    ))?;

    /* Here we iterate over the rows of the result if any it means the table exists */
    while let sqlite::State::Row = statement.next()? {
        result = true;
    }

    Ok(result)
}

fn users_table(conn: &Connection) -> Result<(), Error> {
    conn.execute(
        "
        CREATE TABLE users (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            email TEXT NOT NULL,
            name TEXT NOT NULL,
            last_name TEXT NOT NULL,
            password TEXT NOT NULL
        )
        ",
    )?;

    Ok(())
}

fn classrooms_table(conn: &Connection) -> Result<(), Error> {
    conn.execute(
        "
        CREATE TABLE classrooms (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            number INTEGER NOT NULL,
            building TEXT NOT NULL,
            type TEXT NOT NULL,
            capacity INTEGER NOT NULL,
        )
        ",
    )?;

    Ok(())
}

fn groups_table(conn: &Connection) -> Result<(), Error> {
    conn.execute(
        "
        CREATE TABLE IF NOT EXISTS groups (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            grade INTEGER NOT NULL,
            group_name TEXT NOT NULL,
            specialty TEXT,
            shift TEXT NOT NULL,
            students_count INTEGER NOT NULL
        )
        ",
    )?;

    Ok(())
}

fn subjects_table(conn: &Connection) -> Result<(), Error> {
    conn.execute(
        "
        CREATE TABLE IF NOT EXISTS subjects (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT NOT NULL,
            shorten TEXT NOT NULL,
            color TEXT NOT NULL,
            type TEXT
        )
        ",
    )?;

    Ok(())
}

fn teachers_table(conn: &Connection) -> Result<(), Error> {
    conn.execute(
        "
        CREATE TABLE IF NOT EXISTS teachers (
            id INTEGER PRIMARY KEY,
            shorten TEXT NOT NULL,
            name TEXT NOT NULL,
            first_last_name TEXT NOT NULL,
            second_last_name TEXT NOT NULL,

            email TEXT,
            phone TEXT,
            degree TEXT,
            commissioned_hours INTEGER,
            active_hours INTEGER,
            general_stars INTEGER
        )
        ",
    )?;

    Ok(())
}

fn default_db_path() -> Result<PathBuf, std::io::Error> {
    let mut path = data_dir().ok_or(std::io::Error::new(
        std::io::ErrorKind::NotFound,
        "Directory not found",
    ))?;
    path.push("school_schedule.db");
    Ok(path)
}

