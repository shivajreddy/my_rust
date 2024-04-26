#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]

use core::fmt;

struct MySmartPointer {
    data: String,
}

impl MySmartPointer {
    fn new(data: String) -> Self {
        MySmartPointer { data }
    }
}

impl Drop for MySmartPointer {
    fn drop(&mut self) {
        println!("[{} is being dropped ]", self.data);
    }
}

struct MySmartpointer2 {}

#[derive(Debug)]
struct Person {
    age: i32,
    name: String,
}

pub fn main() {
    /*
    println!("Program Starts");
    let o1: MySmartPointer = MySmartPointer::new("hello 😙".to_string()); // ---------
    let p: &MySmartPointer = &o1;

    println!("Something");

    println!("{}", p.data);
    println!("Program Ends");
    // */

    let s: String = "hi".to_string();
    let s1: &String = &s;

    println!("{} {:p} {}", s, &s, *&s);
    println!("{} {0:p} {}", s1, *s1);

    let p = Person {
        age: 30,
        name: "bob".to_string(),
    };

    let r1 = &p;
}

struct ListNode {
    val: i32,
    // next: Option<ListNode>,
    next: Option<Box<ListNode>>,
}
