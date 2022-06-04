use arrayref::array_refs;

pub fn _2_array_refs() {
	let x: [u8; 10] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
	let (a, b, c) = array_refs!(&x, 2, 3, 5);

	assert_eq!(2, a.len());
	assert_eq!(3, b.len());
	assert_eq!(5, c.len());

	println!("_2_array_refs: a ===> {:?}", a);
	println!("_2_array_refs: b ===> {:?}", b);
	println!("_2_array_refs: c ===> {:?}", c);

	assert_eq!(*a, [0, 1]);
	assert_eq!(*b, [2, 3, 4]);
	assert_eq!(*c, [5, 6, 7, 8, 9]);
}
