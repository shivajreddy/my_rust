#![allow(unused)]

/// 0: Main Dependencies
/// - tokio, anyhow, surrealdb

/// All required imports
use anyhow::{anyhow, Result};
use std::collections::BTreeMap;
use surrealdb::engine::remote::ws::Ws;
use surrealdb::opt::auth::Root;
use surrealdb::sql::{thing, Datetime, Object, Thing, Value};
use surrealdb::{Response, Surreal};

/// Entry point of the program
#[tokio::main]
async fn main() -> Result<()> {
    let db = Surreal::new::<Ws>("localhost:8000").await?;

    // singin in as a namespace, database, or root user
    db.signin(Root {
        username: "root",
        password: "root",
    })
    .await?;

    // Create a session

    // ---- Create
    let sql = "CREATE task:"

    Ok(())
}
