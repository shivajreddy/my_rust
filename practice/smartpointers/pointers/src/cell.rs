#![allow(unused)]

/*
Cell:
- use when dealing with smaller size values (like numbers)
- can implement Copy or Default
- for type with Copy trait: get() retrieves the value by duplicating it
- for type with Default trait: take() replaces the current interior value
with the default value and returns the replaced value


RefCell -> use when dealing with Larger data
-
*/

use std::cell::UnsafeCell;

pub struct Cell<T>
where
    T: Copy,
{
    value: UnsafeCell<T>,
}

impl<T> Cell<T>
where
    T: Copy,
{
    pub fn new(value: T) -> Self {
        Cell {
            value: UnsafeCell::new(value),
        }
    }
    pub fn set(&self, value: T) {
        unsafe {
            *self.value.get() = value;
        }
    }
    pub fn get(&self, v: T) -> T {
        unsafe { *self.value.get() }
    }
}

#[cfg(test)]
mod test {
    use super::Cell;
    fn bad() {
        use std::sync::Arc;
        let x = Arc::new(Cell::new(42));
        // let x1 = x.clone();
        let x1 = Arc::clone(&x);

        // std::thread::spawn(|| {
        //     x1.set(43);
        // });
    }
}
