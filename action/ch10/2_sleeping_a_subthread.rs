use std::{time, thread};

fn main() {
	let start = time::Instant::now();

	let handler = thread::spawn(|| {
		let pause = time::Duration::from_millis(300);
		thread::sleep(pause.clone());
	});
	handler.join().unwrap();

	let finish = time::Instant::now();
	println!("Finish: {:02?}", finish.duration_since(start));
}