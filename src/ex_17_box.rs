use std::{fmt::Display, ops::Deref};

struct MyBox<T: Display>(T);

impl<T: Display> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T: Display> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

// implement drop function.
impl<T: Display> Drop for MyBox<T> {
    fn drop(&mut self) {
        println!("Dropping with data `{}`!", self.0);
    }
}

fn hello(name: &str) {
    println!("Hello, {name}!");
}

pub fn example_box() {
    // example of using box.
    let x = 5;
    let y = Box::new(x);
    assert_eq!(5, x);
    assert_eq!(5, *y);

    // example of using custom box (need deref implementation).
    let x = 5;
    let y = MyBox::new(x);
    assert_eq!(5, x);
    assert_eq!(5, *y);

    // passing box to a function with reference param
    let m = MyBox::new(String::from("Rust"));
    hello(&(*m)[..]);
    hello(&(*m));
    hello(&m);
}
