#![allow(unused)]

use std::{fs, io};

use thiserror::Error;

/// https://www.youtube.com/watch?v=s5S2Ed5T-dc&t=639s
/// --- This is the example(last before) section of the video

fn main() {}

#[derive(Debug)]
struct Node {}

impl Node {
    fn parent(&self) -> Option<&Node> {
        todo!()
    }
}

fn grand_parent_not_very_clear(n: &Node) -> Option<&Node> {
    // yes you could write like this, BUT the process getting to grandparent
    // tells us that  there are 2 possible ways we can get an error
    // and we are not gracefully catching/reporting them. Now downstream code
    // wouldn't know exactly what is happening for better error handling
    n.parent()?.parent()

    /*
    - the same of write above is this, which clearly shows the 2 possible
        error cases that we should be handling
    ```
    Some(
        n.parent()? // get the parent
            .parent()?, // then it's parent
    )
    ```
    */
}

fn grand_parent(n: &Node) -> Result<&Node, GrandParentError> {
    Ok(n.parent()
        .ok_or(GrandParentError::ParentNotFound)?
        .parent()
        .ok_or(GrandParentError::GrandParentNotFound)?)
}

#[derive(Debug, thiserror::Error)]
enum GrandParentError {
    #[error("ParentNotFound")]
    ParentNotFound,
    #[error("GrandParentNotFound")]
    GrandParentNotFound,
}

/*
impl Error for GrandParentError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }

    fn description(&self) -> &str {
        "description() is deprecated; use Display"
    }

    fn cause(&self) -> Option<&dyn Error> {
        self.source()
    }

    fn provide<'a>(&'a self, request: &mut std::error::Request<'a>) {}
}
*/

/// --- The final part(enum) of the video
// no say we are writing a function which would call the `grand_parent` function
// and after calling that fn we get a result and
// we want write this result into a file. but as we know writing into file can
// also throw an error, so our function now will have 3 possible errors
fn log_some(n: &Node) -> Result<(), LogGPError> {
    let g = grand_parent(n)?;

    fs::write("info.txt", &format!("{g:?}"))?;

    Ok(())
}

/*
   - this wouldn't work because, even though now our enum holds both
   the GrandParentError and the IOError variant, our enum can't convert from
    one variant to the other variant

#[derive(Debug)]
enum LogGPError {
    GrandParentError(GrandParentError),
    IOError(io::Error),
}
*/

#[derive(Debug, thiserror::Error)]
enum LogGPError {
    #[error("grandparent error {0:?}")]
    GrandParentError(#[from] GrandParentError),

    #[error(transparent)]
    IOError(#[from] io::Error),
}
