#[derive(Debug)]
enum List {
    Cons(i32, Rc<List>),
    Nil,
}

use crate::List::{Cons, Nil};
use std::rc::Rc;

impl Drop for List {
    fn drop(&mut self) {
        println!("Dropping List with data {:?}", self);
    }
}

fn main() {
    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    println!("a: {:?}", a);
    println!("Count after creating a: {}", Rc::strong_count(&a));

    let b = Cons(3, Rc::clone(&a));
    println!("b: {:?}", b);
    println!("Count after creating b: {}", Rc::strong_count(&a));

    {
    let c = Cons(4, Rc::clone(&a));
    println!("c: {:?}", c);
    println!("Count after creating a, b, c: {}", Rc::strong_count(&a));
    println!("WEAK Count : {}", Rc::weak_count(&a));
    }

    println!("Count after dropping c: {}", Rc::strong_count(&a));
    println!("Count after dropping c: {}", Rc::weak_count(&a));
    println!("Exiting main.");
}
