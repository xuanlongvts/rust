fn main() {
	let result: f32 = 0.1 + 0.1;
	let desired: f32 = 0.2;
	let absolute_difference = (desired - result).abs();
	assert!(absolute_difference < f32::EPSILON);

	let x = (-42.0_f32).sqrt();
	println!("x: {}", x);
	// assert_eq!(x, x); // NaN

	let y: f32 = 1.0 / 0.0;
	println!("y: {}", y);
	assert!(y.is_finite());
}