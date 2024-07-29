
## How to split the file into folders(organize modules) and files(modules)

### 1. Project Structure
Here is the structure of the project:

my_project/
├── src/
│   ├── main.rs
│   ├── foo/
│   │   ├── mod.rs        -> this file makes the 'foo' folder a module, and inside are sub-mods
│   │   ├── foo1.rs
│   │   └── foo2.rs
│   └── bar/
│       ├── mod.rs
│       ├── bar1.rs
│       └── bar2.rs

### 2. Define Modules
> src/foo/mod.rs   -> so the folder/mod.rs file makes the folder a module, and also you 
define what this folder(module) should include, that is what sub-modules(folders or files)
This file declares the modules within the foo folder.
```rust
pub mod foo1;
pub mod foo2;
```

> src/foo/foo1.rs
This is one of the module files in the foo folder.
```rust
pub fn foo1_function() {
    println!("This is foo1 function");
}
```
> src/foo/foo2.rs
This is another module file in the foo folder.
```rust
pub fn foo2_function() {
    println!("This is foo2 function");
}
```

> src/bar/mod.rs
This file declares the modules within the bar folder.
```rust
pub mod bar1;
pub mod bar2;
src/bar/bar1.rs
This is one of the module files in the bar folder.
```


> src/bar/bar1.rs
```rust
pub fn bar1_function() {
    println!("This is bar1 function");
}
```
> src/bar/bar2.rs
This is another module file in the bar folder.
```rust
pub fn bar2_function() {
    println!("This is bar2 function");
}
```


### 3. Use Modules in main.rs
Now, we will bring these modules into scope in our main.rs.
> src/main.rs
```rust
// Declare the foo and bar modules
mod foo;
mod bar;

// Use the modules
use foo::foo1;
use foo::foo2;
use bar::bar1;
use bar::bar2;

fn main() {
    foo1::foo1_function();
    foo2::foo2_function();
    bar1::bar1_function();
    bar2::bar2_function();
}
```
