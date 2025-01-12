// Double Linked List

#[allow(unused_imports)]
use std::{
    cell::RefCell,
    rc::{Rc, Weak},
};

pub struct Node<T: Copy> {
    value: T,
    next: Option<Rc<RefCell<Node<T>>>>,
    prev: Option<Rc<RefCell<Node<T>>>>,
}

impl<T: Copy> Node<T> {
    pub fn new(value: T) -> Self {
        Node {
            value,
            next: None,
            prev: None,
        }
    }
}

// Entry Point
fn main() {
    println!("Doubly Linked List");

    let root: Node<i32> = Node::new(10);
    println!("{}", root.value);
}
