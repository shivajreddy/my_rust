#![allow(unused)]
/// 1. add the `sqlite` not 'rusqlite' crate as a dependancy

fn main() {
    /// This is how you make a connection
    let connection = sqlite::open(":memory").unwrap();

    /// Create a query
    let query = "
    CREATE TABLE users (name TEXT, age INTEGER);
    INSERT INTO users VALUES ('Alice', 42);
    INSERT INTO users VALUES ('Bob', 69);
    ";

    let r = connection.execute(query);
}
