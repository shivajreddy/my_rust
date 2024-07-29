extern crate chapter_01;

use chapter_01::referances::*;

#[test]
fn test1() {
    assert_eq!("hello", foo());
}
