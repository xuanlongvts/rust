use std::convert::TryInto;

fn main() {
	let a: i32 = 10;
	let b: u16 = 100;
	
	if a < (b as i32) {
		println!("10 < 16");
	}

	let b_ = b.try_into().unwrap();

	if a < b_ {
		println!("10 < 16");
	}
}