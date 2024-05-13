#![allow(dead_code)]
#![allow(unused_variables)]

/*
If A = [3, 4] and B = [1, 2, 3, 4, 5], then A is a sublist of B
If A = [1, 2, 3] and B = [1, 2, 3], then A and B are equal
If A = [1, 2, 3, 4, 5] and B = [2, 3, 4], then A is a superlist of B
*/

pub fn main() {
    let a = [3, 4];
    let b = [1, 2, 3, 4, 5];

    let a = [1, 2, 3];
    let b = [1, 2, 3];

    let a = [1, 2, 3, 4, 5];
    let b = [2, 3, 4];
}
