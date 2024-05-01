#![allow(dead_code)]
#![allow(unused_variables)]

struct BigData {
    ppty1: i32,
    ppty2: i32,
    ppty3: i32,
    ppty4: usize,
    ppty5: i8,
    ppty6: i64,
}

struct Node<T> {
    val: T,
    next: Option<Box<Node<T>>>,
}

struct LinkedListNode<T> {
    val: T,
    prev: Option<Box<LinkedListNode<T>>>,
    next: Option<Box<LinkedListNode<T>>>,
}

pub fn main() {
    let x = Box::new(10);

    println!("{} {0:p}", &x);
}
