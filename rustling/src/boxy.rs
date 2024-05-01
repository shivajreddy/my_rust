#![allow(dead_code)]
#![allow(unused_variables)]

use std::mem;

struct BigData {
    // ppty1: i32,
    // ppty2: i32,
    // ppty6: i64,
}

struct SomeStruct {
    ppty1: i32,
    ppty2: i32,
    ppty3: i32,
}

impl SomeStruct {
    fn size(&self) {
        let size = mem::size_of::<SomeStruct>();
        println!("Size: {} bytes", size);
    }
}

impl BigData {
    fn size(&self) {
        let size = mem::size_of::<SomeStruct>();
        println!("Size: {} bytes", size);
    }
}

pub fn main() {
    let s1: SomeStruct = SomeStruct {
        ppty1: 10,
        ppty2: 20,
        ppty3: 30,
    };
    s1.size();

    let b1: BigData = BigData {
        // ppty1: 10,
        // ppty2: 20,
        // ppty6: 30,
    };
    b1.size();

    let name = "hi".to_string();
    println!("{}", mem::size_of_val(&name));

    let x: i32 = 10;
    println!("{} bytes", mem::size_of_val(&x));

    let y: i64 = 10;
    println!("{} bytes", mem::size_of_val(&y));
}
