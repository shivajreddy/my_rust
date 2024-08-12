extern crate c_01;
use c_01::lifetimes::*;

#[test]
fn test1() {
    assert_eq!(1, something());
}

#[test]
fn something2() {
    println!("something2 test function");
    println!("something2 test function");
}
