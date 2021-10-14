fn main() {
	let a: i64 = 42;
	let a_ptr = &a as *const i64;

	println!("a: {}", a);
	println!("Raw pointer of a: {:p}", a_ptr);

	println!("============================================");
	let b: i64 = 42;
	let b_ptr = &b as *const i64;
	let b_addr: usize = unsafe {
		std::mem::transmute(b_ptr)
	};
	println!("{:p}, dereference + 7 ====> 0x{:x}", b_ptr, b_addr + 7);

	println!("============================================");
	let ptr = 42 as *const Vec<String>;
	unsafe {
		let new_addr = ptr.offset(4);
		println!("{:p} -> {:p}", ptr, new_addr);
	}
}