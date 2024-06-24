#![allow(unused)]

/*
use std::fs;
// crossterm necessary to run the application
use crossterm::{
    event::{self, KeyCode, KeyEventKind},
    terminal::{disable_raw_mode, enable_raw_mode,
               EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand
};

// ratatui necessary to run the application
use ratatui::{
    prelude::{CrosstermBackend, Stylize, Terminal},
    widgets::Paragraph
};


use std::io::{stdout, Result};


/// Add code to set up and restore the terminal
/// 1. App enters the 'alternate screen'
/// - allows app to render, without disturbing normal term.
/// 2. App enables the 'raw mode'
/// - this turns off input and output process by terminal
/// 3. App creates a 'backend' and 'terminal'
///  - and then clears the screen
/// Note: If disabling raw mode doesn't work
///  - windows: close terminal tab
///  - unix: `reset`
fn main() -> Result<()> {
    stdout().execute(EnterAlternateScreen)?;
    enable_raw_mode()?;

    Ok(())
}
// */

use std::fmt::{Debug, Formatter};
use std::fs::File;
use std::io::{self, stdout, Error, Read, Result, Write};
use std::mem::size_of_val;
use std::ops::Deref;

#[cfg(target_os = "windows")]
fn os_specific_function() {
    pritnln!("This is windows specific");
}

#[cfg(target_os = "macos")]
fn os_specific_function() {
    pritnln!("This is MacOs specific");
}

// fn main() -> Result<String, io::Error> {
fn main() -> Result<()> {
    let result_file = std::fs::File::open("foo.txt");
    let mut file = result_file.unwrap(); // panics if file isn't found
                                         // create a file with text
                                         // let mut file = File::create("foo.txt")?;
                                         // file.write_all(b"wow\n")?;
                                         // file.write_all(b"wow")?;

    let mut buffer = vec![];

    // check the current operating system
    let x = file.read_to_end(&mut buffer);

    let r1 = cfg!(target_os = "windows");
    let r2 = cfg!(target_os = "macos");
    let r3 = cfg!(target_os = "linux");
    println!("{r1}{r2}{r3}");

    if cfg!(target_os = "windows") {
        println!("you are running a windows")
    };

    println!("{:?}", buffer);

    // let mut my_boxed: Box<str> = long_string.into_boxed_str();
    // println!("{my_boxed}");

    // long_string.push_str("new bytes");
    // println!("{long_string}");
    // let mut new_file = File::create("bar.txt");

    // create a file in target directory

    // println!("{}", size_of_val(&new_file));

    // read a file, and print content to console, line by line

    // copy data from one file to another

    std::io::Result::Ok(())
}

struct SomeType {}

impl SomeType {
    fn something() {
        println!("something is executed");
    }
}
