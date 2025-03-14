/*
    MINIGREP
    Accept its two command line arguments
    1. File path
    2. string to search for
*/

#![allow(unused_variables)]

use std::env;

fn print_strings(v: &Vec<String>) {
    for s in v {
        println!("{}--", s);
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    print_strings(&args);
    // dbg!(args);
    println!("Hello, world!");
    println!("Hello, world!");
}
