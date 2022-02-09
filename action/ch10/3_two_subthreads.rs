use std::{time, thread};

fn main() {
	let start = time::Instant::now();

	let handle_1 = thread::spawn(|| {
		let pause = time::Duration::from_millis(300);
		thread::sleep(pause.clone());
	});
	let handle_2 = thread::spawn(||  {
		let pause = time::Duration::from_millis(300);
		thread::sleep(pause.clone());
	});
	handle_1.join().unwrap();
	handle_2.join().unwrap();

	let finish = time::Instant::now();
	println!("Finish: {:?}", finish.duration_since(start));
}