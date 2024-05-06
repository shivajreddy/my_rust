#![allow(dead_code)]
#![allow(unused_variables)]

use std::mem;

#[derive(Clone, Copy, Debug)]
struct BigData {
    ppty1: i32,
    ppty2: i32,
    ppty3: i64,
    ppty4: i64,
    ppty5: i32,
}

pub fn main() {
    let literal: BigData = BigData {
        ppty1: 10,
        ppty2: 20,
        ppty3: 30,
        ppty4: 30,
        ppty5: 20,
    };
    println!("size_of_val(&b1) {} ", mem::size_of_val(&literal));

    let copy = literal.clone();

    println!("size_of_val(&copy) {}", mem::size_of_val(&copy));

    let box1 = Box::new(BigData {
        ppty1: 10,
        ppty2: 20,
        ppty3: 30,
        ppty4: 30,
        ppty5: 20,
    });
    println!("box1 mem: {:p}", &box1);
    println!("Size of box1: {}", mem::size_of_val(&box1));
    println!("*box1 {:?}", *box1);
    println!("*box1-heap-mem: {:p}", &(*box1));

    {
        let box_copy = box1.clone();
        // let box_copy = box1;
        println!("box_copy mem: {:p}", &box_copy);
        println!("Size of box_copy: {}", mem::size_of_val(&box_copy));
        println!("*box_copy-heap-mem: {:p}", &(*box_copy));
    }

    println!("box1 mem: {:p}", &box1);
    println!("Size of box1: {}", mem::size_of_val(&box1));

    // println!("i32: {} bytes", mem::size_of::<i32>());
    // println!("i64: {} bytes", mem::size_of::<i64>());
    // println!("usize: {} bytes", mem::size_of::<usize>());
}

struct ListNode {
    val: i32,
    next: Option<Box<ListNode>>,
}

fn reverse_linked_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut prev: Option<Box<ListNode>> = None;
    let mut curr = head;

    while let Some(mut node) = curr {
        curr = node.next.take();

        node.next = prev;

        prev = Some(node);
    }

    prev
}
