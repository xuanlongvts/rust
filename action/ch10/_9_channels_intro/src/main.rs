#[macro_use] // The `select!` macro used on line 14 originates here.
extern crate crossbeam;

use std::thread;
use crossbeam::channel::{unbounded};

fn main() {
    let (tx, rx) = unbounded(); // Creating a channel involves calling a function that returns `Sender<T>` and `Receiver<T>`. In this example, the compiler detects that we are using a number and creates `Sender<i32>` and `Receiver<i32>`.

    thread::spawn(move || {
        tx.send(42).unwrap();
    });

    select!{
        recv(rx) -> msg => println!("{:?}", msg)
    }
}
