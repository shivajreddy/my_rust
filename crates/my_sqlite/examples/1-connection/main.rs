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
impl Person {
    fn new(id: i32, name: String, data: Option<Vec<u8>>) -> Self {
        Self { id, name, data }
    }
}

fn main() -> Result<()> {
    /// This is how you make a connection
    let connection = Connection::open_in_memory()?;

    /// Create a query, here we are creating a new table
    let query = "
    CREATE TABLE person (
            id    INTEGER PRIMARY KEY,
            name  TEXT NOT NULL,
            data  BLOB
    )
    ";

    /// Execute the sqlite query
    connection.execute(
        query,
        (), // empty list of parameters
    )?;

    /// Create an instance of Person
    let person_1 = Person::new(
        29,
        "shiva".to_string(),
        // Some(Vec::<u8>::new()), // could be none
        None,
    );

    /// sql to insert the instance into the table
    let query = "
    INSERT INTO person (name, data) VALUES(?1, ?2)
    ";

    connection.execute(
        query,
        (&person_1.name, &person_1.data), // params
    )?;

    /// sql data to rust data
    /// sql query to select all persons in
    let query = "
    SELECT id, name, data FROM person
    ";

    /// This is a statement
    let mut statement = connection.prepare(query)?;
    /// create an iterator of Person type
    let person_iter = statement.query_map([], |row| {
        Ok(Person {
            id: row.get(0)?,
            name: row.get(1)?,
            data: row.get(2)?,
        })
    })?;

    for person in person_iter {
        println!("{:?}", person?);
    }

    Ok(())
}
