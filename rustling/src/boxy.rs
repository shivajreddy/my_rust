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

    let literal_copy = literal.clone();
    println!("size_of_val(&copy) {}", mem::size_of_val(&literal_copy));

    let box_variable = Box::new(BigData {
        ppty1: 10,
        ppty2: 20,
        ppty3: 30,
        ppty4: 30,
        ppty5: 20,
    });
    println!("box1 mem: {:p}", &box_variable);
    println!("Size of box1: {}", mem::size_of_val(&box_variable));
    println!("*box1 {:?}", *box_variable);
    println!("*box1-heap-mem: {:p}", &(*box_variable));

    {
        let box_copy = box_variable.clone();
        // let box_copy = box1;
        println!("box_copy mem: {:p}", &box_copy);
        println!("Size of box_copy: {}", mem::size_of_val(&box_copy));
        println!("*box_copy-heap-mem: {:p}", &(*box_copy));
    }

    println!("box1 mem: {:p}", &box_variable);
    println!("Size of box1: {}", mem::size_of_val(&box_variable));

    // println!("i32: {} bytes", mem::size_of::<i32>());
    // println!("i64: {} bytes", mem::size_of::<i64>());
    // println!("usize: {} bytes", mem::size_of::<usize>());
}
