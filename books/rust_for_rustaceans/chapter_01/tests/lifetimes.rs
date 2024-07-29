extern crate chapter_01;

use chapter_01::lifetimes::*;

#[test]
fn test1() {
    assert_eq!(1, something());
}
