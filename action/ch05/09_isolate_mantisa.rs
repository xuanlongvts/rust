fn main() {
	let n: f32 = 42.42;
	let n_bists: u32 = n.to_bits();
	let mut mantissa: f32 = 1.0;

	for i in 0..23 {
		let mask = 1 << i;
		let one_at_bit_i = n_bists & mask;
		if one_at_bit_i != 0 {
			let i_ = i as f32;
			let weight = 2_f32.powf(i_ - 23.0);
			mantissa += weight;
		}
	}
	println!("mantissa: {}", mantissa);
}

// for CPU present details float
// https://vi.wikipedia.org/wiki/S%E1%BB%91_th%E1%BB%B1c_d%E1%BA%A5u_ph%E1%BA%A9y_%C4%91%E1%BB%99ng

// https://www.youtube.com/watch?v=CJw8n8kBfNQ
// https://www.youtube.com/watch?v=040sNZSelnc