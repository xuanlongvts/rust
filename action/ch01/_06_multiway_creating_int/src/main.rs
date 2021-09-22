use std::rc::Rc;
use std::sync::{Arc, Mutex};

fn main() {
    let a = 10; // integer on the stack
    let b = Box::new(20); // integer on the heap, also know as a boxed integer
    let c = Rc::new(Box::new(30)); // boxed integer wrapped within a reference counter
    let d = Arc::new(Mutex::new(40)); // integer wrapped in an atomic reference counter and protected by a mutual exclusion lock
    println!("a = {:?}", a);
    println!("b = {:?}", b);
    println!("c = {:?}", c);
    println!("d = {:?}", d);
}