fn main() {
	// 65_535 max
	let mut i: u16 = 0;

	print!("{}..", i);

	loop {
		i += 1000;
		print!("{}..", i);
		if i % 10_000 == 0 {
			println!();
		}
	}
}