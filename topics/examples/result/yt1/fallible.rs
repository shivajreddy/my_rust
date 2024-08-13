#![allow(unused)]

#[derive(Debug)]
struct Q(i32);

fn main() {
    /// simple example to understand what map actually is doing inside the
    /// constructor of W's implementation
    let mut res: Result<i32, ()> = Err(());
    let q1 = res.map(|val| Q(val)); // verbose
    let mut res: Result<i32, ()> = Err(());
    let q2 = res.map(Q); // same as above, but simpler

    println!("q1:{:?}", q1);
    println!("q1:{:?}", q1);
}

fn something() {}

#[derive(Debug)]
struct E;

fn fallible(_: u8) -> Result<String, E> {
    todo!()
}

struct W(String);
impl W {
    /*
    say, we have to create instance of W using the 'fallible' method
    this means that the output would be of type Result<> because according
    to `fallible` there can be an error, so we want `new` also to be able to
    handle those errors, so we output Result type
    */
    fn new(x: u8) -> Result<Self, E> {
        fallible(x).map(Self)
    }
}

// Say now we have 2 unit struct, and we should perform fallible on both the
// the units, and only if both fallible calls result Ok() variant only then
// we create the W2 instance
struct W2(String, String);
impl W2 {
    fn new(x: u8, y: u8) -> Result<Self, E> {
        // one way of doing is using the .and_then if we no error encountered
        fallible(x).and_then(|x_result| {
            // after a successful Ok() of first argument, then we move to 2nd
            fallible(y).map(|y_result| Self(x_result, y_result))
        })
    }
}

// Implementing W2 but with the `?` operator
/*
the question mark operator: ?
this is what the ? actually does
```
match operand {
    Ok(x) => x, // if the operand is Ok,
                // then we unwrap it, and type of Ok is T, which Result<T, E>
    Err(e) => return Err(From::from(e))
}
```

*/

struct W3(String, String);
impl W3 {
    fn new(x: u8, y: u8) -> Result<Self, E> {
        Ok(Self(fallible(x)?, fallible(y)?))
    }

    /// `?` is called as a unary postfix operator
    // The following will throw error because, `?` is expanded into the match
    // operation, and it will either return Ok(T) or Err(E), so the output of
    // any fn that uses the `?` operator must return the type Result or Option
    /*
    fn new(x: u8, y: u8) -> Self {
        Self(fallible(x)?, fallible(y)?)
    }
    */

    /// both new2 and new3 functions are not return a Result, so using `?` with
    /// in them is not allowed. So a good rule of thumb is you always avoid
    /// the use of unwrap() because you are not handling possible errors at
    /// compile time, you are going to panic i.e., RE and that's not good.
    fn new2(x: u8, y: u8) -> Self {
        fallible(x)
            .and_then(|x_result| fallible(y).map(|y_result| Self(x_result, y_result)))
            .unwrap()
    }
    fn new3(x: u8, y: u8) -> Self {
        let x_result = fallible(x).map(|v| String::from(v)).unwrap();
        let y_result = fallible(y).map(|v| String::from(v)).unwrap();
        Self(x_result, y_result)
    }
}

struct W4(Vec<String>);
impl W4 {
    /// this is how you could do with a priliminary approach, but there is
    /// a better way
    fn new_verbose(v: &[u8]) -> Result<Self, E> {
        let mut res: Vec<String> = vec![];
        for &item in v {
            res.push(fallible(item)?)
        }
        Ok(Self(res))
    }
}
