use std::thread;

fn main() {
    let mut data = 10;

    thread::spawn(|| { data = 111 });

    thread::spawn(|| { data = 222 });

    println!("data: {}", data);
}
