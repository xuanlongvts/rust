fn main() {
	let one				= [1, 2, 3];
	let two: [u8; 3]	= [1, 2, 3];
	let blank1			= [0; 3];
	let blank2: [u8; 3] = [0; 3];

	let arrs = [one, two, blank1, blank2];
	for a in &arrs {
		print!("{:?}: ", a);
		for i in a.iter() {
			print!("\t{} + 10 = {}", i, i + 10);
		}

		let mut sum = 0;
		for i in 0..a.len() {
			sum += a[i]
		}
		println!("\t({:?} = {})", a, sum);
	}
}