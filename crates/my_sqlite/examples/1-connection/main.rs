#![allow(unused)]
/// 1. add the 'rusqlite' crate as a dependancy
/// the 'bundled' feature helped with build issue
use rusqlite::{params, Connection, Result};

#[derive(Debug)]
struct Person {
    id: i32,
    name: String,
    data: Option<Vec<u8>>,
}

fn main() -> Result<()> {
    /// This is how you make a connection
    let connection = Connection::open_in_memory()?;

    /// Create a query
    let query = "
    CREATE TABLE person (
            id    INTEGER PRIMARY KEY,
            name  TEXT NOT NULL,
            data  BLOB
    )
    ";

    let r = connection.execute(query, ());
    Ok(())
}
