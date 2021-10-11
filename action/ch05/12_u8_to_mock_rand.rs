fn mock_rand(n: u8) -> f32 {
	let base: u32 = 0b0_01111110_00000000000000000000000; // 1056964608
	let large_n = (n as u32) << 15;
	let f32_bits = base | large_n;
	let m = f32::from_bits(f32_bits);
	2.0 * (m - 0.5)
}

fn main() {
	let oxff = 0xff;
	let ox7f = 0x7f;
	let oxoo = 0x00;

	println!("max of input range: {:08b} -> {:?}", oxff, mock_rand(oxff));
	println!("mid of input range: {:08b} -> {:?}", ox7f, mock_rand(ox7f));
	println!("min of input range: {:08b} -> {:?}", oxoo, mock_rand(oxoo));
}