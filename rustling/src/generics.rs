#![allow(unused)]

struct MyStruct<T, U> {
    t: T,
    u: U,
}

// 1. Generics `<T>` on the implementation block must be defined infront of impl and after the type name
impl<T, U> MyStruct<T, U> {
    fn log_something(&self) {
        println!("{:p}", &self)
    }
}

pub fn main() {}
