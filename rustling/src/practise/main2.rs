#![allow(dead_code)]
#![allow(unused)]
mod generics;
mod lifetimes;
mod lifetimes2;
mod smartpointers;
mod strings;

use generics::*;

fn main() {
    // strings::main();
    // generics::main();
    // lifetimes::main();
    // lifetimes2::main();
    smartpointers::main();
}

/*
struct MyIterWrapper<'a, T> {
    slice: &'a [T],
}

impl<'a, T> Iterator for MyIterWrapper<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {}
}

#[cfg(test)]
mod tests {
    use super::MyIterWrapper;

    #[test]
    fn it_works() {
        let mut collection = vec![1, 2, 3, 4];
        let wrapper = MyIterWrapper { slice: &collection };
        for elem in wrapper.iter_mut() {
            println!("{}", elem);
        }
    }
}
*/
