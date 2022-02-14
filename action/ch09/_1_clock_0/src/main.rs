// extern crate chrono;

use chrono::Local; // timezone type

fn main() {
    // let now = chrono::Local::now();
    let now = Local::now();
    println!("Time now: {}", now);
}
