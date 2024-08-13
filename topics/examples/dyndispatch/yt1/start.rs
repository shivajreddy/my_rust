pub trait Speak {
    fn speak(&self);
}

pub struct Cat;
impl Speak for Cat {
    fn speak(&self) {
        println!("meow");
    }
}

pub struct Dog;
impl Speak for Dog {
    fn speak(&self) {
        println!("woof");
    }
}

fn main() {
    Cat.speak();
    Dog.speak();
}
