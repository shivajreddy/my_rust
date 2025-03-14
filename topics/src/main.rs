<<<<<<< HEAD
fn main() {
    println!("Hello there");
    println!("Hello there");
=======
struct Person {
    name: String,
    age: i32,
}

impl Person {
    fn print(self) {
        println!("{} {}", self.name, self.age);
    }
}

fn main() {
    println!("Hello, World!");

    let p = Person {
        name: "Shiva".to_string(),
        age: 20,
    };
    p.print();
    // println!("Age: {}", p.age);
>>>>>>> refs/remotes/origin/main
}
