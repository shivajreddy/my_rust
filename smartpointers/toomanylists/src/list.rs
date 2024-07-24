use std::{
    fmt::{Display, Formatter, Result},
    ops::Deref,
};

pub struct List {
    head: Option<Box<Node>>,
}

type Link = Option<Box<Node>>;
struct Node {}

#[derive(Debug)]
struct MyBox<T> {
    val: T,
}
// struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(val: T) -> Self {
        MyBox { val }
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.val
    }
}

impl<T> Display for MyBox<T>
where
    T: Display,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        print!("{}", self.val);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test1() {
        let b1 = MyBox::new(10);
        println!("b1: {:?}", b1);
        println!("b1: {}", b1);
        assert_eq!(10, *b1);
        let b2 = Box::new(5);
        println!("b2: {}", b2);
        assert_eq!(5, *b2);
    }
}
