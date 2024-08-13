#![allow(unused)]

use std::{error::Error, fmt::format, fs, io};

use thiserror::Error;

fn main() {}

#[derive(Debug)]
struct Node {}

impl Node {
    fn parent(&self) -> Option<&Node> {
        todo!()
    }
}

fn grand_parent(n: &Node) -> Result<&Node, GrandParentError> {
    Ok(n.parent()
        .ok_or(GrandParentError::ParentNotFound)?
        .parent()
        .ok_or(GrandParentError::GrandParentNotFound)?)
}

/// The following block is using `thisError` create for implementing
/// std::error::Error on the GrandParentError, because in the log_some function
/// which returns anohow::Result<()> will
/*
use thiserror::Error;

#[derive(Debug, Error)]
enum GrandParentError {
    #[error("Parent not found")]
    ParentNotFound,
    #[error("Grand Parent not found")]
    GrandParentNotFound,
}
*/

#[derive(Debug, Error)]
enum GrandParentError {
    #[error("Parent not found")]
    ParentNotFound,
    #[error("Grand Parent found")]
    GrandParentNotFound,
}

// fn log_some(n: &Node) -> Result<(), LogGPError> {
fn log_some(n: &Node) -> anyhow::Result<()> {
    let g = grand_parent(n)?;

    fs::write("info.txt", &format!("{g:?}"))?;

    Ok(())
}

// #[derive(Debug)]
// enum LogGPError {
//     GrandParentError(GrandParentError),
//     IOError(io::Error),
// }
