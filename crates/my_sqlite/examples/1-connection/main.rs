#![allow(unused)]
/// 1. add the 'rusqlite' crate as a dependancy
use rusqlite::{params, Connection, Result};

#[derive(Debug)]
struct Person {
    id: i32,
    name: String,
    data: Option<Vec<u8>>,
}

fn main() -> Result<()> {
    /// This is how you make a connection
    let conn = Connection::open_in_memory()?;

    /// Create a query
    let query = "
    CREATE TABLE users (name TEXT, age INTEGER);
    INSERT INTO users VALUES ('Alice', 42);
    INSERT INTO users VALUES ('Bob', 69);
    ";

    let r = connection.execute(query);
    Ok(())
}
