#![allow(dead_code)]
#![allow(unused_variables)]

pub fn main() {
    let x = Box::new(10);

    println!("{} {0:p}", &x);
}

struct Node<T> {
    val: T,
    next: Option<Box<Node<T>>>,
}
