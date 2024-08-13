#![allow(unused)]
use std::{marker::PhantomData, ptr::NonNull};

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

// now say we want to sketch a Type that is kind of like a referance to any Type that
// implemented Speak-Trait

struct AnythingSpeak<'a> {
    // first thing we have to do is throw the lifetime parameter into
    // a `PhantomData` because we want to be bound by this lifetime
    // parameter, but we are not actually going to use this referance
    // in here. we are going to be using the raw pointer, cuz we are
    // going to point to something, but we dont know what Type we are
    // going to pointing to and do some casting, so using a raw pointer
    // is easy to do that stuff
    _p: PhantomData<&'a ()>,
    // so for the actual data, we will `NonNull` from std, and this says
    // that this Type is a raw pointer but it's not null.
    // the type of this NonNull is, well we don't know that all we know is
    // that it implements Speak trait, which is why we for now we will
    // just say its a unit
    data: NonNull<()>,
}

fn main() {
    Cat.speak();
    Dog.speak();
}
