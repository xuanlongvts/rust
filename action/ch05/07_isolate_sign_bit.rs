fn main() {
	let n: f32 = 42.42;
	let n_bits: u32 = n.to_bits();
	println!("n_bits: {}", n_bits);
	let sign_bit = n_bits >> 31;
	println!("sign_bit: {}", sign_bit); // 0 ==> +
}

// https://www.binaryconvert.com/result_float.html?decimal=052050046052050