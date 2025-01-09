/*
    MINIGREP
    Accept its two command line arguments
    1. File path
    2. string to search for
*/

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    dbg!(args);
    println!("Hello, world!");
}
