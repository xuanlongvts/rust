use arrayref::array_mut_ref;

fn add_three(arr: &[u8; 3]) -> u8 {
	arr[0] + arr[1] + arr[2]
}

pub fn _1_simple_case() {
	let mut x = [0u8; 30];
	x[20] = 1;
	x[21] = 4;
	x[22] = 3;
	x[0] = add_three(array_mut_ref![x, 20, 3]);

	println!("_1_simple_case: ===> {:?}", x);

	assert_eq!(x[0], 8);
}
