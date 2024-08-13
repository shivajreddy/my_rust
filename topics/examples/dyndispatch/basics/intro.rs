#![allow(unused)]

/// why do we even need dynamic dispatch
// - it's how rust achieves polymorphism

// considred the following example. We want a Vector of shapes, but we know
// that Vector is homogenous, so I can't put Rectangle instance, and a Circle
// instance together, so the Vector should hold Rectangle types or Circle Types,
// or create a new EnumType called Shape with both variants, but thats not what
// we want exactly. So to solve this, we have dynamic dispatch, where, we want
// write code that works with different types that have a commong behaviour
// i.e., a common trait

/*
Why Use Dynamic Dispatch?

Flexibility:
Dynamic dispatch is useful when you need to write code that works with
different types implementing the same trait but doesn't need to know the
specific type at compile time.

Runtime Polymorphism:
It allows you to achieve polymorphism, where you can have different behavior
for different types through a common interface (trait).

Downsides:
There is a performance cost associated with dynamic dispatch because it
involves a level of indirection (often through a vtable, a table of function
pointers). You also lose the ability to use some optimizations that the
Rust compiler can apply with static dispatch.

*/
trait Shape {
    fn area(&self) -> f64;
}

#[derive(Debug)]
struct Circle {
    radius: f64,
}

#[derive(Debug)]
struct Rectangle {
    width: f64,
    height: f64,
}

impl Shape for Circle {
    fn area(&self) -> f64 {
        std::f64::consts::PI * self.radius * self.radius
    }
}

impl Shape for Rectangle {
    fn area(&self) -> f64 {
        self.width * self.height
    }
}

fn main() {
    let r1 = Rectangle {
        width: 10.0,
        height: 20.0,
    };
    let r2 = Rectangle {
        width: 20.0,
        height: 30.0,
    };

    let c1 = Circle { radius: 5.0 };
    let c2 = Circle { radius: 7.0 };

    /// -- using `&dyn TraitName`
    let shapes: Vec<&dyn Shape> = vec![&r1, &r2, &c1, &c2];

    for shape in shapes {
        println!("{}", shape.area());
    }

    /// --- using `Box<dyn TraitName>`
    let mut my_shapes: Vec<Box<dyn Shape>> = vec![];

    my_shapes.push(Box::new(r1));
    my_shapes.push(Box::new(r2));
    my_shapes.push(Box::new(c1));
    my_shapes.push(Box::new(c2));

    for shape in my_shapes {
        println!("{}", shape.area());
    }
}
