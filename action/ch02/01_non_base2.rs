fn main() {
	let three = 0b11; // 0b prefix indicates binary (base 2)
	let thirty = 0o36; // 0o prfix indicates by octal (base 8)
	let three_hundred = 0x12C; // 0x prfix indicates by hexadecimal (base 16)

	println!("base 10: {}, {}, {}", three, thirty, three_hundred);
	println!("base 2: {:b}, {:b}, {:b}", three, thirty, three_hundred);
	println!("base 8: {:o}, {:o}, {:o}", three, thirty, three_hundred);
	println!("base 16: {:x}, {:x}, {:x}", three, thirty, three_hundred);
}