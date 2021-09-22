fn main() {
	let a: u8 = 42;
	let r: &u8 = &a;
	let b: u8 = a + *r;
	println!("r: {}", *r);
	println!("b = {}", b);

	println!("=====================");
	let needle = 0o64;
	println!("needle: {}", needle);
	println!("needle: {}", 0o204);
	let haystack = [1, 1, 2, 5, 15, 52, 203, 877, 4140, 21147];
	for item in &haystack {
		if *item == needle {
			println!("item: {}", item);
		}
	}
}