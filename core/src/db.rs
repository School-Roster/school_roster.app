/* Connect to SQLite database */

use sqlite::{Connection, Error};
use thiserror::Error;

/* Error handling */
enum DBErrors {
    #[error("Failed to connect to the database")]
    ConnectionError(#[from] Error),
    #[error("Failed to execute query")]
    QueryError(#[from] Error),
}

/*
*** Connect to the database ***
* Return: Connection or Error
*/
pub fn connect() -> Result<Connection, DBErrors> {
    let conn = sqlite::open("ss.db")?;
    create_table(&conn)?;
    Ok(conn)
}

// Create tables
// It checks if the table exists, if not, it creates it
fn create_table(conn: &Connection) -> Result<(), DBErrors> {
    if !table_exists(conn, "users")? {
        users_table(conn)?;
    }

    if !table_exists(conn, "classrooms")? {
        classrooms_table(conn)?;
    }

    if !table_exists(conn, "groups")? {
        groups_table(conn)?;
    }

    if !table_exists(conn, "subjects")? {
        subjects_table(conn)?;
    }

    if !table_exists(conn, "teachers")? {
        teachers_table(conn)?;
    }
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
fn table_exists(conn: &Connection, table_name: &str) -> Result<bool, DBErrors> {
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

fn users_table(conn: &Connection) -> Result<(), DBErrors> {
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

fn classrooms_table(conn: &Connection) -> Result<(), DBErrors> {
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

fn groups_table(conn: &Connection) -> Result<(), DBErrors> {
    conn.execute(
        "
        CREATE TABLE groups (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            grade INTEGER NOT NULL,
            group TEXT NOT NULL,
            specialty TEXT,
            shift TEXT NOT NULL,
            students_quantity INTEGER NOT NULL,
        )
        ",
    )?;

    Ok(())
}

fn subjects_table(conn: &Connection) -> Result<(), DBErrors> {
    conn.execute(
        "
        CREATE TABLE subjects (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT NOT NULL,
            abbreviation TEXT NOT NULL,
            type TEXT NOT NULL,
            color TEXT NOT NULL,
            teacher_id INTEGER NOT NULL,
        )
        ",
    )?;

    Ok(())
}

fn teachers_table(conn: &Connection) -> Result<(), DBErrors> {
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
            general_stars INTEGER NOT NULL
        )
        ",
    )?;
    Ok(())
}
