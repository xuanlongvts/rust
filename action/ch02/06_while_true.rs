use std::time::{Duration, Instant};

fn main() {
	let mut count = 0;
	let time_limit = Duration::new(1, 0);
	let start = Instant::now();

	println!("time_limit: {:?}, start: {:?}", time_limit, start);

	while (Instant::now() - start) < time_limit {
		count += 1;
	}
	println!("count: {}", count);
}