use std::ops::Add;
use std::time::Duration;

// way 1
fn add<T: Add<Output = T>>(i: T, j: T) -> T {
	i + j
}

// way 2
fn add2<T>(i: T, j: T) -> T where T: Add<Output = T> {
	i + j
}

fn main() {
	let floats = add(1.2, 3.4);
	let ints = add(10, 20);
	let durations = add(
		Duration::new(5, 1),
		Duration::new(10, 2)
	);
	println!("=============== way 1");
	println!("floats: {}", floats);
	println!("ints: {}", ints);
	println!("durations: {:?}", durations);

	let floats = add2(1.2, 3.4);
	let ints = add2(10, 20);
	let durations = add2(
		Duration::new(5, 1),
		Duration::new(10, 2)
	);
	println!("=============== way 2");
	println!("floats: {}", floats);
	println!("ints: {}", ints);
	println!("durations: {:?}", durations);
}