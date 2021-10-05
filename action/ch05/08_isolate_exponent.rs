fn main() {
	let n: f32 = 42.42;
	let n_bits: u32 = n.to_bits();
	let exponent = n_bits >> 23;	
	let exponent = exponent & 0xff;
	let exponent = (exponent as i32) - 127;
	println!("exponent: {}", exponent);
}

// https://www.binaryconvert.com/result_float.html?decimal=052050046052050

// https://www.youtube.com/watch?v=zSbtY2ZaZvw&list=PLYWC7VlGXIfUn5M8IL2Sy-DHQxYagh2qO&index=20
