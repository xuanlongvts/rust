use std::{time, thread};

fn main() {
	for n in 1..101 { // 1..101 or 1..1001
		let mut handlers: Vec<thread::JoinHandle<()>> = Vec::with_capacity(n);
		let start = time::Instant::now();

		for _m in 0..n {
			let handle = thread::spawn(|| {
				let pause = time::Duration::from_millis(20);
				thread::sleep(pause.clone());
			});
			handlers.push(handle);
		}
		// println!("handlers: {:#?}", handlers);

		while let Some(handle) = handlers.pop() {
			// println!("handle: {:#?}", handle);
			handle.join().unwrap();
		}

		let finish = time::Instant::now();
		println!("{} \t {:02?}", n, finish.duration_since(start));
	}
}