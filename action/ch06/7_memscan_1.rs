fn main() {
	let mut n_nonzero = 0;

	for i in 1..10_000 { // 0..10_000 or 1..10_000 will segmentation fault
		let ptr = i as *const u8;
		let byte_ad_addr = unsafe {
			*ptr
		};

		if byte_ad_addr != 0 {
			n_nonzero += 1;
		}
	}
	println!("non-zero bytes in memory: {}", n_nonzero);
}
