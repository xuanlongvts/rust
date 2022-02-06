fn main() {
	
static GLOBAL: i32 = 1000;
	let local_str = "a";
	let local_int = 123;
	let boxed_str = Box::new('b');
	let boxed_int = Box::new(456);

	println!("GLOBAL:		{:p}", &GLOBAL as *const i32);
	println!("local_str:	{:p}", local_str as *const str);
	println!("local_int:	{:p}", local_int as *const i32);
	println!("boxed_str: 	{:p}", Box::into_raw(boxed_str));
	println!("boxed_int: 	{:p}", Box::into_raw(boxed_int));
	println!("fn_int:		{:p}", noop());
}

fn noop() -> *const i32 {
	let noop_local = 12345;
	&noop_local as *const i32
}