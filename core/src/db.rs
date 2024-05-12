/* Connect to SQLite database */

use rusqlite::{Connection, Error, Result};

pub fn connect() -> Result<Connection> {
    match Connection::open("db.sqlite") {
        Ok(conn) => Ok(conn),
        Err(e) => Err(Error::from(e)),
    }
}

pub fn get_conn() -> Connection {
    connect().unwrap()
}
