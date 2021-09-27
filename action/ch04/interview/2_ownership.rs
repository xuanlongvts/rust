struct Demo {
	val: i32
}

fn use_value(d: Demo) {}

fn main() {
	let demo = Demo {
		val: 10
	};
	use_value(demo);

	println!("demo: {:?}", demo.val); // value borrowed here after move
}